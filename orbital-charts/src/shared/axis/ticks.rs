//! Pure helpers for axis tick layout.

use leptos::prelude::*;

use crate::engine::default_tick_format;
use crate::shared::chart_container::DrawingArea;
use crate::{AxisPosition, ChartScale};

/// Tick mark length in pixels.
pub const TICK_SIZE: f64 = 6.0;
/// Gap between the axis line and tick label text.
pub const TICK_LABEL_OFFSET: f64 = 8.0;
/// Estimated tick label height for layout (matches `--orbital-chart-tick-font-size`).
pub const TICK_LABEL_HEIGHT: f64 = 14.0;
/// Gap between the tick label row and the axis title.
pub const AXIS_TITLE_GAP: f64 = 12.0;
/// Horizontal space reserved for a rotated y-axis title.
pub const AXIS_TITLE_WIDTH: f64 = 28.0;
/// Gap between the y-axis title column and tick labels.
pub const AXIS_TITLE_TICK_GAP: f64 = 10.0;
/// Pixel tolerance for detecting ticks at the plot edge (origin corner).
pub const EDGE_TICK_EPSILON: f64 = 1.5;

/// One rendered tick mark with label.
#[derive(Clone, Debug, PartialEq)]
pub struct TickMark {
    /// Pixel position along the axis.
    pub position: f64,
    /// Display label.
    pub label: String,
}

/// Format a tick value using an optional formatter callback.
pub fn format_tick_value(
    value: f64,
    tick_format: Option<&leptos::callback::Callback<(f64,), String>>,
) -> String {
    if let Some(fmt) = tick_format {
        fmt.run((value,))
    } else {
        default_tick_format(value)
    }
}

/// Band axis ticks — one per category, with optional placement offset.
pub fn band_ticks(
    scale: &ChartScale,
    categories: &[String],
    tick_placement: Option<crate::TickPlacement>,
) -> Vec<TickMark> {
    let ChartScale::Band(band) = scale else {
        return Vec::new();
    };
    let half = band.bandwidth() / 2.0;
    categories
        .iter()
        .filter_map(|cat| {
            band.scale(cat).map(|center| {
                let position = match tick_placement {
                    Some(crate::TickPlacement::Start) => center - half,
                    Some(crate::TickPlacement::End) => center + half,
                    Some(crate::TickPlacement::Extremities) => center,
                    _ => center,
                };
                TickMark {
                    position,
                    label: cat.clone(),
                }
            })
        })
        .collect()
}

/// Linear axis ticks from precomputed values.
pub fn linear_ticks(
    scale: &ChartScale,
    values: &[f64],
    tick_format: Option<&leptos::callback::Callback<(f64,), String>>,
) -> Vec<TickMark> {
    let ChartScale::Linear(linear) = scale else {
        return Vec::new();
    };
    values
        .iter()
        .map(|&value| TickMark {
            position: linear.scale(value),
            label: format_tick_value(value, tick_format),
        })
        .collect()
}

/// Tick line endpoint for an x-axis tick (extends downward).
pub fn x_tick_line(x: f64, y_base: f64) -> ((f64, f64), (f64, f64)) {
    ((x, y_base), (x, y_base + TICK_SIZE))
}

/// Tick line endpoint for a y-axis tick (extends leftward).
pub fn y_tick_line(y: f64, x_base: f64) -> ((f64, f64), (f64, f64)) {
    ((x_base, y), (x_base - TICK_SIZE, y))
}

/// Whether a y-axis tick sits on the plot bottom edge (domain minimum).
pub fn is_y_tick_at_plot_bottom(position: f64, plot_height: f64) -> bool {
    (position - plot_height).abs() < EDGE_TICK_EPSILON
}

pub fn x_label_position(x: f64, y_base: f64) -> (f64, f64) {
    (x, y_base + TICK_SIZE + TICK_LABEL_OFFSET)
}

/// Label position for a y-axis value tick.
pub fn y_label_position(y: f64, x_base: f64) -> (f64, f64) {
    (x_base - TICK_SIZE - TICK_LABEL_OFFSET, y)
}

/// Y position for an x-axis title below or above tick labels.
pub fn x_axis_title_y(y_base: f64, position: AxisPosition) -> f64 {
    let tick_row_extent = TICK_SIZE + TICK_LABEL_OFFSET + TICK_LABEL_HEIGHT;
    match position {
        AxisPosition::Top => y_base - tick_row_extent - AXIS_TITLE_GAP,
        _ => y_base + tick_row_extent + AXIS_TITLE_GAP,
    }
}

/// Position for a rotated y-axis title centered in the axis gutter.
pub fn y_axis_title_position(area: &DrawingArea, position: AxisPosition) -> (f64, f64) {
    let y = area.top + area.plot_height / 2.0;
    match position {
        AxisPosition::Right => {
            let right_gutter = (area.width - area.left - area.plot_width).max(0.0);
            (area.left + area.plot_width + right_gutter * 0.5, y)
        }
        _ => (AXIS_TITLE_WIDTH * 0.5, y),
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;

    #[test]
    fn x_axis_title_sits_below_tick_labels() {
        let y_base = 200.0;
        let tick_y = x_label_position(0.0, y_base).1;
        let title_y = x_axis_title_y(y_base, AxisPosition::Bottom);
        assert!(title_y > tick_y + TICK_LABEL_HEIGHT);
    }

    #[test]
    fn y_axis_title_sits_left_of_tick_labels() {
        let area = DrawingArea {
            width: 520.0,
            height: 320.0,
            left: 80.0,
            top: 36.0,
            plot_width: 400.0,
            plot_height: 212.0,
        };
        let (x, y) = y_axis_title_position(&area, AxisPosition::Left);
        assert!(x < y_label_position(0.0, area.left).0 - 20.0);
        assert_eq!(y, area.top + area.plot_height / 2.0);
    }
}
