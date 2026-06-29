//! Gauge chart types.

use crate::PieRadius;

/// Gauge arc geometry configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct GaugeGeometry {
    /// Inner arc radius (px or `%` of max radius).
    pub inner_radius: PieRadius,
    /// Outer arc radius (px or `%` of max radius).
    pub outer_radius: PieRadius,
    /// Arc start angle in degrees.
    pub start_angle: f64,
    /// Arc end angle in degrees.
    pub end_angle: f64,
    /// Center X (px or `%` of width).
    pub cx: PieRadius,
    /// Center Y (px or `%` of height).
    pub cy: PieRadius,
    /// Arc corner radius in pixels.
    pub corner_radius: f64,
}

impl Default for GaugeGeometry {
    fn default() -> Self {
        Self {
            inner_radius: PieRadius::Percent(80.0),
            outer_radius: PieRadius::Percent(100.0),
            start_angle: 0.0,
            end_angle: 360.0,
            cx: PieRadius::Percent(50.0),
            cy: PieRadius::Percent(50.0),
            corner_radius: 0.0,
        }
    }
}

/// Resolved gauge layout in pixel space.
#[derive(Clone, Debug, PartialEq)]
pub struct GaugeLayout {
    /// Center x in plot coordinates.
    pub cx: f64,
    /// Center y in plot coordinates.
    pub cy: f64,
    /// Inner radius in pixels.
    pub inner_radius: f64,
    /// Outer radius in pixels.
    pub outer_radius: f64,
    /// Maximum fitting radius before inset.
    pub max_radius: f64,
    /// Start angle in radians.
    pub start_rad: f64,
    /// End angle in radians.
    pub end_rad: f64,
    /// Value angle in radians (partial sweep); `None` when value is absent.
    pub value_rad: Option<f64>,
    /// Scale minimum.
    pub value_min: f64,
    /// Scale maximum.
    pub value_max: f64,
    /// Current value (if any).
    pub value: Option<f64>,
}
