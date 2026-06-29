//! Plot area bounds after inset is applied.

use crate::PlotInset;

/// Computed drawing area inside plot insets.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DrawingArea {
    /// Total chart width.
    pub width: f64,
    /// Total chart height.
    pub height: f64,
    /// Left edge of the plot area.
    pub left: f64,
    /// Top edge of the plot area.
    pub top: f64,
    /// Plot area width.
    pub plot_width: f64,
    /// Plot area height.
    pub plot_height: f64,
}

/// Compute the drawing area from chart dimensions and plot inset.
pub fn compute_drawing_area(width: f64, height: f64, inset: PlotInset) -> DrawingArea {
    let plot_width = (width - inset.left - inset.right).max(0.0);
    let plot_height = (height - inset.top - inset.bottom).max(0.0);
    DrawingArea {
        width,
        height,
        left: inset.left,
        top: inset.top,
        plot_width,
        plot_height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_drawing_area_applies_inset() {
        let area = compute_drawing_area(
            400.0,
            300.0,
            PlotInset {
                top: 20.0,
                bottom: 30.0,
                left: 40.0,
                right: 10.0,
            },
        );
        assert_eq!(area.plot_width, 350.0);
        assert_eq!(area.plot_height, 250.0);
        assert_eq!(area.left, 40.0);
        assert_eq!(area.top, 20.0);
    }
}
