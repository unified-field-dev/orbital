//! Gauge context and [`use_gauge_state`] hook.

use leptos::prelude::*;

use crate::GaugeLayout;

/// Computed gauge geometry exposed to custom overlay children.
#[derive(Clone, Debug, PartialEq)]
pub struct GaugeState {
    /// Current metric value.
    pub value: Option<f64>,
    /// Scale minimum.
    pub value_min: f64,
    /// Scale maximum.
    pub value_max: f64,
    /// Arc center x in SVG coordinates.
    pub cx: f64,
    /// Arc center y in SVG coordinates.
    pub cy: f64,
    /// Inner radius in pixels.
    pub inner_radius: f64,
    /// Outer radius in pixels.
    pub outer_radius: f64,
    /// Maximum fitting radius.
    pub max_radius: f64,
    /// Start angle in radians.
    pub start_rad: f64,
    /// End angle in radians.
    pub end_rad: f64,
    /// Value angle in radians (partial sweep).
    pub value_angle: Option<f64>,
}

impl From<GaugeLayout> for GaugeState {
    fn from(layout: GaugeLayout) -> Self {
        Self {
            value: layout.value,
            value_min: layout.value_min,
            value_max: layout.value_max,
            cx: layout.cx,
            cy: layout.cy,
            inner_radius: layout.inner_radius,
            outer_radius: layout.outer_radius,
            max_radius: layout.max_radius,
            start_rad: layout.start_rad,
            end_rad: layout.end_rad,
            value_angle: layout.value_rad,
        }
    }
}

/// Read gauge layout from context inside [`Gauge`] children.
pub fn use_gauge_state() -> GaugeState {
    expect_context::<GaugeState>()
}

/// Provide gauge state to child components.
#[component]
pub fn GaugeContextProvider(state: GaugeState, children: Children) -> impl IntoView {
    provide_context(state);
    children()
}
