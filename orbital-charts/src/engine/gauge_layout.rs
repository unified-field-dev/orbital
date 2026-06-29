//! Gauge arc layout and angle math.

use std::f64::consts::PI;

use crate::{GaugeGeometry, GaugeLayout, PieRadius};

/// Resolve gauge geometry to pixel layout.
pub fn resolve_gauge_layout(
    plot_width: f64,
    plot_height: f64,
    geometry: &GaugeGeometry,
    value: Option<f64>,
    value_min: f64,
    value_max: f64,
) -> GaugeLayout {
    let max_radius = plot_width.min(plot_height) / 2.0;
    let cx = geometry.cx.resolve_center(plot_width, plot_height, true);
    let cy = geometry.cy.resolve_center(plot_width, plot_height, false);
    let inner_radius = geometry.inner_radius.resolve(plot_width, plot_height, true);
    let outer_radius = geometry.outer_radius.resolve(plot_width, plot_height, true);

    let start_rad = deg_to_rad(geometry.start_angle);
    let end_rad = deg_to_rad(geometry.end_angle);
    let value_rad = value.map(|v| {
        let frac = value_fraction(v, value_min, value_max);
        start_rad + frac * (end_rad - start_rad)
    });

    GaugeLayout {
        cx,
        cy,
        inner_radius,
        outer_radius,
        max_radius,
        start_rad,
        end_rad,
        value_rad,
        value_min,
        value_max,
        value,
    }
}

/// Parse a radius string into [`PieRadius`].
pub fn parse_gauge_radius(s: &str, default: PieRadius) -> PieRadius {
    if s.is_empty() {
        return default;
    }
    PieRadius::parse(s)
}

/// Map value to 0..1 fraction within min/max.
pub fn value_fraction(value: f64, value_min: f64, value_max: f64) -> f64 {
    if (value_max - value_min).abs() < f64::EPSILON {
        return 0.0;
    }
    ((value - value_min) / (value_max - value_min)).clamp(0.0, 1.0)
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_fraction_clamps() {
        assert_eq!(value_fraction(50.0, 0.0, 100.0), 0.5);
        assert_eq!(value_fraction(-10.0, 0.0, 100.0), 0.0);
        assert_eq!(value_fraction(150.0, 0.0, 100.0), 1.0);
    }

    #[test]
    fn resolve_gauge_layout_value_angle() {
        let geom = GaugeGeometry::default();
        let layout = resolve_gauge_layout(200.0, 200.0, &geom, Some(75.0), 0.0, 100.0);
        assert!(layout.value_rad.is_some());
        let value_rad = layout.value_rad.unwrap();
        assert!(value_rad > layout.start_rad);
        assert!(value_rad <= layout.end_rad + 1e-9);
    }
}
