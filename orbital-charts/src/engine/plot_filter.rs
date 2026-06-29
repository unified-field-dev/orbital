//! Plot-type filtering for composition mode.

use crate::engine::{ProjectedChartData, ProjectedSeries};
use crate::{ChartType, SeriesDef};

use super::stacking::is_composition_mode;

/// Filter projected data to series matching a plot type in composition mode.
pub fn projected_for_plot_type(
    projected: &ProjectedChartData,
    series_defs: &[SeriesDef],
    plot_type: ChartType,
) -> ProjectedChartData {
    if !is_composition_mode(series_defs) {
        return projected.clone();
    }

    let ids: Vec<String> = series_defs
        .iter()
        .filter(|s| s.chart_type == Some(plot_type))
        .map(|s| s.id.clone())
        .collect();

    let series: Vec<ProjectedSeries> = projected
        .series
        .iter()
        .filter(|s| ids.contains(&s.id))
        .cloned()
        .collect();

    ProjectedChartData {
        categories: projected.categories.clone(),
        series,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ProjectedSeries;

    #[test]
    fn composition_mode_filters_by_chart_type() {
        let projected = ProjectedChartData {
            categories: vec!["A".into()],
            series: vec![
                ProjectedSeries {
                    id: "bars".into(),
                    label: "Bars".into(),
                    data: vec![1.0],
                },
                ProjectedSeries {
                    id: "line".into(),
                    label: "Line".into(),
                    data: vec![2.0],
                },
            ],
        };
        let defs = vec![
            SeriesDef {
                id: "bars".into(),
                chart_type: Some(ChartType::Bar),
                ..Default::default()
            },
            SeriesDef {
                id: "line".into(),
                chart_type: Some(ChartType::Line),
                ..Default::default()
            },
        ];
        let bars = projected_for_plot_type(&projected, &defs, ChartType::Bar);
        assert_eq!(bars.series.len(), 1);
        assert_eq!(bars.series[0].id, "bars");
    }

    #[test]
    fn single_chart_mode_returns_all_series() {
        let projected = ProjectedChartData {
            categories: vec!["A".into()],
            series: vec![ProjectedSeries {
                id: "a".into(),
                label: "A".into(),
                data: vec![1.0],
            }],
        };
        let result = projected_for_plot_type(&projected, &[], ChartType::Bar);
        assert_eq!(result.series.len(), 1);
    }
}
