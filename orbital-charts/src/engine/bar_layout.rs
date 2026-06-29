//! Grouped bar geometry for cartesian bar charts.

use crate::engine::{BandScale, LinearScale, ProjectedSeries};
use crate::ChartOrientation;

/// Render-ready bar rectangle in plot coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct BarGeometry {
    /// Left edge (vertical) or top edge (horizontal) in plot space.
    pub x: f64,
    /// Top edge (vertical) or left edge (horizontal) in plot space.
    pub y: f64,
    /// Bar width in plot space.
    pub width: f64,
    /// Bar height in plot space.
    pub height: f64,
    /// Owning series id.
    pub series_id: String,
    /// Index within the category axis.
    pub data_index: usize,
    /// Numeric value encoded by this bar.
    pub value: f64,
}

/// Compute grouped bar rectangles for all series and categories.
pub fn compute_grouped_bars(
    orientation: ChartOrientation,
    categories: &[String],
    series: &[ProjectedSeries],
    category_scale: &BandScale,
    value_scale: &LinearScale,
    bar_gap_ratio: f64,
    baseline: f64,
) -> Vec<BarGeometry> {
    if categories.is_empty() || series.is_empty() {
        return Vec::new();
    }

    let series_count = series.len() as f64;
    let band_width = category_scale.bandwidth();
    let gap_ratio = bar_gap_ratio.clamp(-1.0, 1.0);
    let total_gap = (series_count - 1.0).max(0.0) * gap_ratio;
    let bar_thickness = if series_count > 0.0 {
        band_width / (series_count + total_gap)
    } else {
        band_width
    };
    let step = bar_thickness * (1.0 + gap_ratio);

    let baseline_px = value_scale.scale(baseline);

    let mut bars = Vec::new();
    for (cat_idx, category) in categories.iter().enumerate() {
        let Some(center) = category_scale.scale(category) else {
            continue;
        };
        let group_start = center - band_width / 2.0;

        for (series_idx, s) in series.iter().enumerate() {
            let value = s.data.get(cat_idx).copied().unwrap_or(0.0);
            let value_px = value_scale.scale(value);

            match orientation {
                ChartOrientation::Vertical => {
                    let x = group_start + series_idx as f64 * step;
                    let top = value_px.min(baseline_px);
                    let bottom = value_px.max(baseline_px);
                    bars.push(BarGeometry {
                        x,
                        y: top,
                        width: bar_thickness,
                        height: bottom - top,
                        series_id: s.id.clone(),
                        data_index: cat_idx,
                        value,
                    });
                }
                ChartOrientation::Horizontal => {
                    let y = group_start + series_idx as f64 * step;
                    let left = value_px.min(baseline_px);
                    let right = value_px.max(baseline_px);
                    bars.push(BarGeometry {
                        x: left,
                        y,
                        width: right - left,
                        height: bar_thickness,
                        series_id: s.id.clone(),
                        data_index: cat_idx,
                        value,
                    });
                }
            }
        }
    }
    bars
}

/// Compute stacked bar rectangles for series sharing stack groups.
pub fn compute_stacked_bars(
    orientation: ChartOrientation,
    categories: &[String],
    raw_series: &[ProjectedSeries],
    stacked_series: &[ProjectedSeries],
    stack_groups: &std::collections::HashMap<String, String>,
    series_defs: &[crate::SeriesDef],
    category_scale: &BandScale,
    value_scale: &LinearScale,
    offset: crate::StackOffset,
    order: crate::StackOrder,
) -> Vec<BarGeometry> {
    if categories.is_empty() || raw_series.is_empty() {
        return Vec::new();
    }

    let band_width = category_scale.bandwidth();

    let mut bars = Vec::new();
    for (cat_idx, category) in categories.iter().enumerate() {
        let Some(center) = category_scale.scale(category) else {
            continue;
        };
        let x = center - band_width / 2.0;

        for s in stacked_series {
            if !stack_groups.contains_key(&s.id) {
                continue;
            }
            let raw = raw_series
                .iter()
                .find(|r| r.id == s.id)
                .and_then(|r| r.data.get(cat_idx))
                .copied()
                .unwrap_or(0.0);
            let top_val = s.data.get(cat_idx).copied().unwrap_or(0.0);
            let bottom_val = crate::engine::stacking::stack_segment_bottom(
                raw_series,
                stacked_series,
                stack_groups,
                series_defs,
                &s.id,
                cat_idx,
                offset,
                order,
            );

            let (value_min, value_max) = if raw >= 0.0 {
                (bottom_val, top_val)
            } else {
                (bottom_val, top_val)
            };

            let min_px = value_scale.scale(value_min);
            let max_px = value_scale.scale(value_max);

            match orientation {
                ChartOrientation::Vertical => {
                    let top = min_px.min(max_px);
                    let bottom = min_px.max(max_px);
                    bars.push(BarGeometry {
                        x,
                        y: top,
                        width: band_width,
                        height: (bottom - top).abs(),
                        series_id: s.id.clone(),
                        data_index: cat_idx,
                        value: raw,
                    });
                }
                ChartOrientation::Horizontal => {
                    let left = min_px.min(max_px);
                    let right = min_px.max(max_px);
                    bars.push(BarGeometry {
                        x: left,
                        y: center - band_width / 2.0,
                        width: (right - left).abs(),
                        height: band_width,
                        series_id: s.id.clone(),
                        data_index: cat_idx,
                        value: raw,
                    });
                }
            }
        }
    }
    bars
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_series() -> Vec<ProjectedSeries> {
        vec![
            ProjectedSeries {
                id: "a".into(),
                label: "A".into(),
                data: vec![10.0, 20.0],
            },
            ProjectedSeries {
                id: "b".into(),
                label: "B".into(),
                data: vec![15.0, 5.0],
            },
        ]
    }

    #[test]
    fn vertical_grouped_bars_two_series() {
        let categories = vec!["Q1".into(), "Q2".into()];
        let band = BandScale::new(categories.clone(), (0.0, 200.0), 0.1);
        let value = LinearScale::new((0.0, 100.0), (200.0, 0.0));
        let bars = compute_grouped_bars(
            ChartOrientation::Vertical,
            &categories,
            &sample_series(),
            &band,
            &value,
            0.1,
            0.0,
        );
        assert_eq!(bars.len(), 4);
        assert!(bars[0].width > 0.0);
        assert!(bars[0].height > 0.0);
    }

    #[test]
    fn horizontal_swaps_dimensions() {
        let categories = vec!["Q1".into()];
        let band = BandScale::new(categories.clone(), (0.0, 100.0), 0.1);
        let value = LinearScale::new((0.0, 100.0), (0.0, 300.0));
        let bars = compute_grouped_bars(
            ChartOrientation::Horizontal,
            &categories,
            &sample_series(),
            &band,
            &value,
            0.1,
            0.0,
        );
        assert_eq!(bars.len(), 2);
        assert!(bars[0].width > 0.0);
        assert!(bars[0].height > 0.0);
    }

    #[test]
    fn zero_value_bar_has_zero_extent() {
        let categories = vec!["Q1".into()];
        let series = vec![ProjectedSeries {
            id: "a".into(),
            label: "A".into(),
            data: vec![0.0],
        }];
        let band = BandScale::new(categories.clone(), (0.0, 100.0), 0.0);
        let value = LinearScale::new((0.0, 100.0), (100.0, 0.0));
        let bars = compute_grouped_bars(
            ChartOrientation::Vertical,
            &categories,
            &series,
            &band,
            &value,
            0.1,
            0.0,
        );
        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].height, 0.0);
    }
}
