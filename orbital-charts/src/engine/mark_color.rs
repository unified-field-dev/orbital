//! Mark color resolution with axis color scale support.

use crate::engine::{resolve_color, resolve_series_color, BarGeometry};
use crate::{AxisDef, ColorScaleKind, OrbitalChartPalette, SeriesDef};

/// Resolve fill/stroke color for a cartesian series mark.
pub fn resolve_mark_color(
    series_index: usize,
    series: &SeriesDef,
    palette: &OrbitalChartPalette,
    category: Option<&str>,
    value: Option<f64>,
    category_axis: Option<&AxisDef>,
    value_axis: Option<&AxisDef>,
) -> String {
    if let Some(color) = &series.color {
        return color.clone();
    }

    if let (Some(cat), Some(axis)) = (category, category_axis) {
        if let Some(scale) = axis.color_scale.as_ref() {
            if scale.kind == ColorScaleKind::Ordinal {
                if let Some(idx) = axis
                    .data
                    .as_ref()
                    .and_then(|data| data.iter().position(|c| c == cat))
                {
                    if let Some(color) = scale.colors.get(idx) {
                        return color.clone();
                    }
                }
            }
        }
    }

    if let (Some(v), Some(axis)) = (value, value_axis) {
        if let Some(scale) = axis.color_scale.as_ref() {
            let domain = value_domain(axis, v);
            return resolve_color(v, scale, domain);
        }
    }

    resolve_series_color(series_index, series, palette)
}

/// Resolve color for a bar geometry entry.
pub fn resolve_bar_color(
    bar: &BarGeometry,
    series_index: usize,
    series: &SeriesDef,
    palette: &OrbitalChartPalette,
    categories: &[String],
    category_axis: Option<&AxisDef>,
    value_axis: Option<&AxisDef>,
) -> String {
    let category = categories.get(bar.data_index).map(String::as_str);
    resolve_mark_color(
        series_index,
        series,
        palette,
        category,
        Some(bar.value),
        category_axis,
        value_axis,
    )
}

fn value_domain(axis: &AxisDef, value: f64) -> (f64, f64) {
    if let Some(thresholds) = axis
        .color_scale
        .as_ref()
        .and_then(|s| s.thresholds.as_ref())
    {
        if let (Some(min), Some(max)) = (thresholds.first(), thresholds.last()) {
            return (*min, *max);
        }
    }
    let pad = value.abs().max(1.0) * 0.1;
    (value - pad, value + pad)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ColorScale, ColorScaleKind};

    #[test]
    fn explicit_series_color_wins() {
        let series = SeriesDef {
            id: "a".into(),
            color: Some("#abc".into()),
            ..Default::default()
        };
        let color = resolve_mark_color(
            0,
            &series,
            &OrbitalChartPalette::default(),
            None,
            None,
            None,
            None,
        );
        assert_eq!(color, "#abc");
    }

    #[test]
    fn ordinal_axis_color_maps_category() {
        let axis = AxisDef {
            id: "x".into(),
            data: Some(vec!["A".into(), "B".into()]),
            color_scale: Some(ColorScale {
                kind: ColorScaleKind::Ordinal,
                colors: vec!["#111".into(), "#222".into()],
                thresholds: None,
            }),
            ..Default::default()
        };
        let series = SeriesDef {
            id: "a".into(),
            ..Default::default()
        };
        let color = resolve_mark_color(
            0,
            &series,
            &OrbitalChartPalette::default(),
            Some("B"),
            None,
            Some(&axis),
            None,
        );
        assert_eq!(color, "#222");
    }
}
