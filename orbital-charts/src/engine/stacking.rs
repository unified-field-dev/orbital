//! Stack offsets and orders for bar, line, and area charts.

use std::collections::HashMap;

use crate::engine::ProjectedSeries;
use crate::{ChartType, SeriesDef, StackOffset, StackOrder};

/// Whether any series definition declares an explicit chart type (composition mode).
pub fn is_composition_mode(series_defs: &[SeriesDef]) -> bool {
    series_defs.iter().any(|s| s.chart_type.is_some())
}

/// Stack group map from series definitions.
pub fn stack_groups_from_series(series_defs: &[SeriesDef]) -> HashMap<String, String> {
    series_defs
        .iter()
        .filter_map(|s| s.stack_group.as_ref().map(|g| (s.id.clone(), g.clone())))
        .collect()
}

/// Resolve offset and order for a stack group from member series definitions.
pub fn resolve_stack_config(
    group: &str,
    series_defs: &[SeriesDef],
    default_chart_type: ChartType,
) -> (StackOffset, StackOrder) {
    let members: Vec<_> = series_defs
        .iter()
        .filter(|s| s.stack_group.as_deref() == Some(group))
        .collect();
    let offset = members
        .iter()
        .find_map(|s| s.stack_offset)
        .unwrap_or(match default_chart_type {
            ChartType::Bar => StackOffset::Diverging,
            _ => StackOffset::None,
        });
    let order = members
        .iter()
        .find_map(|s| s.stack_order)
        .unwrap_or(StackOrder::None);
    (offset, order)
}

/// Resolve the dominant stack offset across all groups in a chart.
pub fn resolve_chart_stack_offset(
    series_defs: &[SeriesDef],
    default_chart_type: ChartType,
) -> StackOffset {
    let mut groups: Vec<&str> = series_defs
        .iter()
        .filter_map(|s| s.stack_group.as_deref())
        .collect();
    groups.sort_unstable();
    groups.dedup();
    groups
        .first()
        .map(|group| resolve_stack_config(group, series_defs, default_chart_type).0)
        .unwrap_or(StackOffset::None)
}

/// Reorder series indices within a stack group.
pub fn apply_stack_order(
    indices: &[usize],
    series: &[ProjectedSeries],
    order: StackOrder,
) -> Vec<usize> {
    match order {
        StackOrder::None => indices.to_vec(),
        StackOrder::Reverse => indices.iter().rev().copied().collect(),
        StackOrder::Appearance => {
            let mut ordered = indices.to_vec();
            ordered.sort_by_key(|&idx| {
                series[idx]
                    .data
                    .iter()
                    .enumerate()
                    .filter_map(|(row, &value)| value.is_finite().then_some((value, row)))
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(_, row)| row)
                    .unwrap_or(usize::MAX)
            });
            ordered
        }
        StackOrder::Ascending => {
            let mut ordered = indices.to_vec();
            ordered.sort_by(|&a, &b| {
                total_abs(&series[a].data)
                    .partial_cmp(&total_abs(&series[b].data))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            ordered
        }
        StackOrder::Descending => {
            let mut ordered = indices.to_vec();
            ordered.sort_by(|&a, &b| {
                total_abs(&series[b].data)
                    .partial_cmp(&total_abs(&series[a].data))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            ordered
        }
    }
}

fn total_abs(data: &[f64]) -> f64 {
    data.iter().filter(|v| v.is_finite()).map(|v| v.abs()).sum()
}

/// Stack series values within shared stack groups.
pub fn stack_series(
    series: &[ProjectedSeries],
    stack_groups: &HashMap<String, String>,
    offset: StackOffset,
    order: StackOrder,
) -> Vec<ProjectedSeries> {
    if series.is_empty() {
        return Vec::new();
    }

    let row_count = series.first().map(|s| s.data.len()).unwrap_or(0);
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, s) in series.iter().enumerate() {
        if let Some(group) = stack_groups.get(&s.id) {
            groups.entry(group.clone()).or_default().push(idx);
        }
    }

    let mut result: Vec<ProjectedSeries> = series
        .iter()
        .map(|s| ProjectedSeries {
            id: s.id.clone(),
            label: s.label.clone(),
            data: if stack_groups.contains_key(&s.id) {
                vec![0.0; row_count]
            } else {
                s.data.clone()
            },
        })
        .collect();

    for indices in groups.values() {
        let ordered = apply_stack_order(indices, series, order);
        for row in 0..row_count {
            match offset {
                StackOffset::Diverging => stack_row_diverging(&ordered, series, &mut result, row),
                StackOffset::Expand => stack_row_expand(&ordered, series, &mut result, row),
                StackOffset::None => stack_row_none(&ordered, series, &mut result, row),
            }
        }
    }

    result
}

fn stack_row_none(
    ordered: &[usize],
    series: &[ProjectedSeries],
    result: &mut [ProjectedSeries],
    row: usize,
) {
    let mut cumulative = 0.0;
    for &idx in ordered {
        let raw = series[idx].data.get(row).copied().unwrap_or(0.0);
        cumulative += raw;
        result[idx].data[row] = cumulative;
    }
}

fn stack_row_expand(
    ordered: &[usize],
    series: &[ProjectedSeries],
    result: &mut [ProjectedSeries],
    row: usize,
) {
    let row_total: f64 = ordered
        .iter()
        .map(|&idx| series[idx].data.get(row).copied().unwrap_or(0.0))
        .sum();
    if row_total.abs() <= f64::EPSILON {
        for &idx in ordered {
            result[idx].data[row] = 0.0;
        }
        return;
    }
    let mut cumulative = 0.0;
    for &idx in ordered {
        let raw = series[idx].data.get(row).copied().unwrap_or(0.0);
        cumulative += raw;
        result[idx].data[row] = cumulative / row_total;
    }
}

fn stack_row_diverging(
    ordered: &[usize],
    series: &[ProjectedSeries],
    result: &mut [ProjectedSeries],
    row: usize,
) {
    let mut pos_top = 0.0;
    let mut neg_ceiling = 0.0;
    for &idx in ordered {
        let raw = series[idx].data.get(row).copied().unwrap_or(0.0);
        if raw >= 0.0 {
            pos_top += raw;
            result[idx].data[row] = pos_top;
        } else {
            let y1 = neg_ceiling;
            neg_ceiling += raw;
            result[idx].data[row] = y1;
        }
    }
}

/// Bottom of a stacked segment at a row (cumulative of prior series in group).
pub fn stack_segment_bottom(
    raw_series: &[ProjectedSeries],
    stacked: &[ProjectedSeries],
    stack_groups: &HashMap<String, String>,
    _series_defs: &[SeriesDef],
    series_id: &str,
    row: usize,
    offset: StackOffset,
    order: StackOrder,
) -> f64 {
    let Some(group) = stack_groups.get(series_id) else {
        return 0.0;
    };

    let indices: Vec<usize> = stacked
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| (stack_groups.get(&s.id) == Some(group)).then_some(idx))
        .collect();
    let ordered = apply_stack_order(&indices, raw_series, order);

    let raw = raw_series
        .iter()
        .find(|s| s.id == series_id)
        .and_then(|s| s.data.get(row))
        .copied()
        .unwrap_or(0.0);

    match offset {
        StackOffset::Diverging => {
            if raw >= 0.0 {
                let mut bottom = 0.0;
                for &idx in &ordered {
                    if raw_series[idx].id == series_id {
                        break;
                    }
                    let prior_raw = raw_series[idx].data.get(row).copied().unwrap_or(0.0);
                    if prior_raw >= 0.0 {
                        bottom = stacked[idx].data.get(row).copied().unwrap_or(bottom);
                    }
                }
                bottom
            } else {
                stacked
                    .iter()
                    .find(|s| s.id == series_id)
                    .and_then(|s| s.data.get(row))
                    .copied()
                    .unwrap_or(0.0)
                    + raw
            }
        }
        _ => {
            let mut bottom = 0.0;
            for s in stacked {
                if stack_groups.get(&s.id) != Some(group) {
                    continue;
                }
                if s.id == series_id {
                    break;
                }
                bottom = s.data.get(row).copied().unwrap_or(bottom);
            }
            bottom
        }
    }
}

/// Convenience wrapper using stack config resolved from series definitions.
pub fn stack_segment_bottom_for_series(
    raw_series: &[ProjectedSeries],
    stacked: &[ProjectedSeries],
    stack_groups: &HashMap<String, String>,
    series_defs: &[SeriesDef],
    series_id: &str,
    row: usize,
) -> f64 {
    let Some(group) = stack_groups.get(series_id) else {
        return 0.0;
    };
    let chart_type = series_defs
        .iter()
        .find(|s| s.id == series_id)
        .and_then(|s| s.chart_type)
        .unwrap_or(ChartType::Bar);
    let (offset, order) = resolve_stack_config(group, series_defs, chart_type);
    stack_segment_bottom(
        raw_series,
        stacked,
        stack_groups,
        series_defs,
        series_id,
        row,
        offset,
        order,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> (Vec<ProjectedSeries>, HashMap<String, String>) {
        let series = vec![
            ProjectedSeries {
                id: "a".into(),
                label: "A".into(),
                data: vec![10.0, 20.0],
            },
            ProjectedSeries {
                id: "b".into(),
                label: "B".into(),
                data: vec![5.0, 10.0],
            },
        ];
        let groups = HashMap::from([("a".into(), "g".into()), ("b".into(), "g".into())]);
        (series, groups)
    }

    #[test]
    fn stack_cumulative_values() {
        let (series, groups) = sample();
        let stacked = stack_series(&series, &groups, StackOffset::None, StackOrder::None);
        assert_eq!(stacked[0].data, vec![10.0, 20.0]);
        assert_eq!(stacked[1].data, vec![15.0, 30.0]);
    }

    #[test]
    fn expand_normalizes_to_one() {
        let (series, groups) = sample();
        let stacked = stack_series(&series, &groups, StackOffset::Expand, StackOrder::None);
        assert!((stacked[1].data[0] - 1.0).abs() < f64::EPSILON);
        let seg_a = stacked[0].data[0]
            - stack_segment_bottom(
                &series,
                &stacked,
                &groups,
                &[],
                "a",
                0,
                StackOffset::Expand,
                StackOrder::None,
            );
        let seg_b = stacked[1].data[0]
            - stack_segment_bottom(
                &series,
                &stacked,
                &groups,
                &[],
                "b",
                0,
                StackOffset::Expand,
                StackOrder::None,
            );
        assert!((seg_a + seg_b - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn diverging_splits_signs() {
        let series = vec![
            ProjectedSeries {
                id: "pos".into(),
                label: "Pos".into(),
                data: vec![10.0],
            },
            ProjectedSeries {
                id: "neg".into(),
                label: "Neg".into(),
                data: vec![-5.0],
            },
        ];
        let groups = HashMap::from([("pos".into(), "g".into()), ("neg".into(), "g".into())]);
        let stacked = stack_series(&series, &groups, StackOffset::Diverging, StackOrder::None);
        assert_eq!(stacked[0].data, vec![10.0]);
        assert_eq!(stacked[1].data, vec![0.0]);
        let neg_bottom = stack_segment_bottom(
            &series,
            &stacked,
            &groups,
            &[],
            "neg",
            0,
            StackOffset::Diverging,
            StackOrder::None,
        );
        assert_eq!(neg_bottom, -5.0);
    }

    #[test]
    fn stack_order_reverse() {
        let (series, groups) = sample();
        let stacked = stack_series(&series, &groups, StackOffset::None, StackOrder::Reverse);
        assert_eq!(stacked[0].data, vec![15.0, 30.0]);
        assert_eq!(stacked[1].data, vec![5.0, 10.0]);
    }

    #[test]
    fn resolve_stack_config_bar_defaults_diverging() {
        let defs = vec![SeriesDef {
            id: "a".into(),
            stack_group: Some("g".into()),
            chart_type: Some(ChartType::Bar),
            ..Default::default()
        }];
        let (offset, order) = resolve_stack_config("g", &defs, ChartType::Bar);
        assert_eq!(offset, StackOffset::Diverging);
        assert_eq!(order, StackOrder::None);
    }
}
