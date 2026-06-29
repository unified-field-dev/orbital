//! Pure zoom window math for axis domain slicing.

use crate::engine::{ProjectedChartData, ProjectedSeries};
use crate::{AxisDef, ChartFeatures, ScaleType, ZoomConfig, ZoomFilterMode, ZoomWindow};

/// Whether zoom is active for an axis given feature flags and axis config.
pub fn axis_zoom_active(features: ChartFeatures, axis: &AxisDef) -> bool {
    features.contains(ChartFeatures::ZOOM_PAN)
        && axis.zoom.as_ref().is_some_and(|config| config.enabled)
}

/// Collect axis ids with active zoom from x and y axis lists.
pub fn enabled_zoom_axes<'a>(
    features: ChartFeatures,
    x_axes: &'a [AxisDef],
    y_axes: &'a [AxisDef],
) -> Vec<(&'a AxisDef, ZoomConfig)> {
    x_axes
        .iter()
        .chain(y_axes.iter())
        .filter_map(|axis| {
            axis.zoom
                .as_ref()
                .filter(|_| axis_zoom_active(features, axis))
                .map(|config| (axis, config.clone()))
        })
        .collect()
}

/// Default full-range window for an axis.
pub fn default_zoom_window(axis_id: impl Into<String>) -> ZoomWindow {
    ZoomWindow::full(axis_id)
}

/// Clamp and snap a zoom window per axis zoom config.
pub fn clamp_zoom_window(mut window: ZoomWindow, config: &ZoomConfig) -> ZoomWindow {
    let (mut start, mut end) = if window.start <= window.end {
        (window.start, window.end)
    } else {
        (window.end, window.start)
    };

    if let Some(min_start) = config.min_start {
        start = start.max(min_start);
    }
    if let Some(max_end) = config.max_end {
        end = end.min(max_end);
    }

    let mut span = (end - start).max(0.0);

    if let Some(min_span) = config.min_span {
        if span < min_span {
            let center = (start + end) / 2.0;
            span = min_span;
            start = center - span / 2.0;
            end = center + span / 2.0;
        }
    }

    if let Some(max_span) = config.max_span {
        if span > max_span {
            let center = (start + end) / 2.0;
            span = max_span;
            start = center - span / 2.0;
            end = center + span / 2.0;
        }
    }

    start = start.clamp(0.0, 100.0);
    end = end.clamp(0.0, 100.0);

    if end - start < 0.01 {
        end = (start + 0.01).min(100.0);
    }

    if let Some(step) = config.step {
        if step > 0.0 {
            start = (start / step).round() * step;
            end = (end / step).round() * step;
        }
    }

    start = start.clamp(0.0, 100.0);
    end = end.clamp(0.0, 100.0).max(start + 0.01);

    window.start = start;
    window.end = end.min(100.0);
    window
}

/// Map a percent window to inclusive-exclusive band index range.
pub fn zoom_window_to_band_indices(count: usize, window: &ZoomWindow) -> (usize, usize) {
    if count == 0 {
        return (0, 0);
    }
    let start_idx = ((window.start / 100.0) * count as f64).floor() as usize;
    let end_idx = ((window.end / 100.0) * count as f64).ceil() as usize;
    let start_idx = start_idx.min(count.saturating_sub(1));
    let end_idx = end_idx.max(start_idx + 1).min(count);
    (start_idx, end_idx)
}

/// Map percent window to linear domain subset.
pub fn zoom_window_to_linear_domain(full_domain: (f64, f64), window: &ZoomWindow) -> (f64, f64) {
    let span = full_domain.1 - full_domain.0;
    let start = full_domain.0 + span * (window.start / 100.0);
    let end = full_domain.0 + span * (window.end / 100.0);
    (start, end)
}

/// Slice categories for a band/point axis zoom window.
pub fn slice_categories(categories: &[String], window: &ZoomWindow) -> Vec<String> {
    let (start, end) = zoom_window_to_band_indices(categories.len(), window);
    categories[start..end].to_vec()
}

/// Filter projected data to visible band indices when filter mode is discard.
pub fn filter_projected_by_band_window(
    data: &ProjectedChartData,
    window: &ZoomWindow,
) -> ProjectedChartData {
    let (start, end) = zoom_window_to_band_indices(data.categories.len(), window);
    let categories = data.categories[start..end].to_vec();
    let series = data
        .series
        .iter()
        .map(|s| {
            let slice_end = end.min(s.data.len());
            ProjectedSeries {
                id: s.id.clone(),
                label: s.label.clone(),
                data: s.data[start..slice_end].to_vec(),
            }
        })
        .collect();
    ProjectedChartData { categories, series }
}

/// Apply band zoom by slicing categories on the axis definition.
pub fn apply_band_zoom_to_axis(axis: &mut AxisDef, window: &ZoomWindow) {
    if let Some(data) = axis.data.as_ref() {
        axis.data = Some(slice_categories(data, window));
    }
}

/// Zoom in/out centered on a pointer position within the full domain (0–1 fraction).
pub fn zoom_at_pointer(
    window: &ZoomWindow,
    pointer_fraction: f64,
    zoom_in: bool,
    config: &ZoomConfig,
) -> ZoomWindow {
    let span = window.end - window.start;
    let factor = if zoom_in { 0.85 } else { 1.0 / 0.85 };
    let new_span = span * factor;
    let anchor = window.start + span * pointer_fraction.clamp(0.0, 1.0);
    let new_start = anchor - new_span * pointer_fraction;
    clamp_zoom_window(
        ZoomWindow {
            axis_id: window.axis_id.clone(),
            start: new_start,
            end: new_start + new_span,
        },
        config,
    )
}

/// Pan the zoom window by a delta in percent of full domain.
pub fn pan_window(window: &ZoomWindow, delta_pct: f64, config: &ZoomConfig) -> ZoomWindow {
    let span = window.end - window.start;
    let mut new_start = window.start + delta_pct;
    let mut new_end = window.end + delta_pct;

    if new_start < 0.0 {
        new_end -= new_start;
        new_start = 0.0;
    }
    if new_end > 100.0 {
        new_start -= new_end - 100.0;
        new_end = 100.0;
    }

    if new_end - new_start > span + f64::EPSILON {
        new_end = new_start + span;
    }

    clamp_zoom_window(
        ZoomWindow {
            axis_id: window.axis_id.clone(),
            start: new_start,
            end: new_end,
        },
        config,
    )
}

/// Resolve zoom window for an axis id, defaulting to full range.
pub fn window_for_axis(windows: &[ZoomWindow], axis_id: &str) -> ZoomWindow {
    windows
        .iter()
        .find(|w| w.axis_id == axis_id)
        .cloned()
        .unwrap_or_else(|| default_zoom_window(axis_id))
}

/// Whether any zoomed axis on the primary category axis uses discard filtering.
pub fn should_filter_projected(
    primary_axis_id: &str,
    x_axes: &[AxisDef],
    windows: &[ZoomWindow],
) -> bool {
    x_axes
        .iter()
        .find(|a| a.id == primary_axis_id)
        .and_then(|a| a.zoom.as_ref())
        .map(|z| z.filter_mode() == ZoomFilterMode::Discard)
        .unwrap_or(false)
        && windows
            .iter()
            .find(|w| w.axis_id == primary_axis_id)
            .is_some_and(|w| w.start > 0.0 || w.end < 100.0)
}

/// Map band index to fraction of full domain (center of band).
pub fn band_index_to_fraction(index: usize, count: usize) -> f64 {
    if count == 0 {
        return 0.5;
    }
    (index as f64 + 0.5) / count as f64
}

/// Whether axis scale type supports percent-window zoom.
pub fn axis_supports_zoom(scale_type: ScaleType) -> bool {
    matches!(
        scale_type,
        ScaleType::Band | ScaleType::Point | ScaleType::Linear
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_respects_min_span() {
        let config = ZoomConfig {
            enabled: true,
            min_span: Some(20.0),
            ..Default::default()
        };
        let window = clamp_zoom_window(
            ZoomWindow {
                axis_id: "x".into(),
                start: 40.0,
                end: 45.0,
            },
            &config,
        );
        assert!((window.end - window.start - 20.0).abs() < 0.01);
    }

    #[test]
    fn band_indices_from_window() {
        let window = ZoomWindow {
            axis_id: "x".into(),
            start: 25.0,
            end: 75.0,
        };
        let (start, end) = zoom_window_to_band_indices(12, &window);
        assert_eq!(start, 3);
        assert_eq!(end, 9);
    }

    #[test]
    fn zoom_at_pointer_narrows_window() {
        let config = ZoomConfig::enabled();
        let window = ZoomWindow::full("x");
        let zoomed = zoom_at_pointer(&window, 0.5, true, &config);
        assert!(zoomed.end - zoomed.start < 100.0);
    }

    #[test]
    fn filter_projected_slices_series() {
        let data = ProjectedChartData {
            categories: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            series: vec![ProjectedSeries {
                id: "s".into(),
                label: "S".into(),
                data: vec![1.0, 2.0, 3.0, 4.0],
            }],
        };
        let window = ZoomWindow {
            axis_id: "x".into(),
            start: 25.0,
            end: 75.0,
        };
        let filtered = filter_projected_by_band_window(&data, &window);
        assert_eq!(filtered.categories, vec!["B".to_string(), "C".to_string()]);
        assert_eq!(filtered.series[0].data, vec![2.0, 3.0]);
    }
}
