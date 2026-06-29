//! [`Gauge`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_macros::component_doc;

use super::container::GaugeContainer;
use crate::engine::parse_gauge_radius;
use crate::{ChartMotion, GaugeGeometry, PieRadius};

/// Center label content: static string or formatter callback.
#[derive(Clone)]
pub enum GaugeLabel {
    /// Fixed label text.
    Text(String),
    /// Formatter receiving `(value, value_min, value_max)`.
    Formatter(Callback<(Option<f64>, f64, f64), String>),
}

impl Default for GaugeLabel {
    fn default() -> Self {
        Self::Formatter(Callback::new(|(value, _, _): (Option<f64>, f64, f64)| {
            value
                .map(|v| {
                    if (v - v.round()).abs() < f64::EPSILON {
                        format!("{}", v.round() as i64)
                    } else {
                        format!("{v:.1}")
                    }
                })
                .unwrap_or_default()
        }))
    }
}

/// Arc gauge for single-metric readouts with meter accessibility.
///
/// Show capacity, completion, or score at a glance in dashboards and status panels.
/// Adjust arc angles for semicircles or full rings; format the center readout for domain units.
///
/// # When to use
///
/// - SLA progress, storage utilization, or survey scores on dashboard cards.
/// - Semicircular gauges in compact layouts (`start_angle=-110`, `end_angle=110`).
/// - Accessible meter readouts with `aria_value_text` when units matter to screen readers.
///
/// # Usage
///
/// 1. Set `value` between `value_min` and `value_max`; `None` hides the value arc.
/// 2. Pick `start_angle` / `end_angle` for semicircle vs full ring layouts.
/// 3. Customize `label` with [`GaugeLabel`] for static text or a formatter callback.
/// 4. Leave `skip_animation` unset to honor reduced-motion; arc tween runs on value changes otherwise.
/// 5. Wrap the gauge in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer semicircular gauges in compact cards — they leave room for labels below the arc.
/// * Provide meaningful `aria_value_text` when the visible label abbreviates units.
/// * Use theme accent colors via `value_color` / `reference_color` for semantic states.
///
/// ## Don'ts
///
/// * Do not use gauges for multi-series comparison — prefer [`crate::BarChart`] or [`crate::Sparkline`].
/// * Do not rely on color alone for critical thresholds — pair with `label` text.
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// ## Planned (Phase 5)
///
/// Tooltip and highlight interaction are not yet available on gauges.
///
/// # Examples
///
/// ## Default gauge
/// Circular gauge at 75 on a 0–100 scale. Arc sweep animation runs on mount and value
/// updates unless reduced motion or `skip_animation` is set.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::Gauge;
/// view! {
///     <div data-testid="gauge-preview">
///         <Gauge value=Some(75.0) width=400.0 height=400.0 />
///     </div>
/// }
/// ```
///
/// ## Semicircle gauge
/// Compact semicircular arc for card layouts. Negative `start_angle` and positive
/// `end_angle` open the gauge upward — common pattern for KPI tiles.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::Gauge;
/// view! {
///     <div data-testid="gauge-semicircle-preview">
///         <Gauge
///             value=Some(62.0)
///             start_angle=-110.0
///             end_angle=110.0
///             width=480.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Accessible value text
/// `role="meter"` with `aria-valuetext` describing units for screen readers. Set when the
/// visible center label omits context (e.g. shows "42" but means "42 minutes remaining").
/// <!-- preview -->
/// ```rust,ignore
/// use crate::Gauge;
/// use crate::charts::gauge::GaugeLabel;
/// view! {
///     <div data-testid="gauge-a11y-preview">
///         <Gauge
///             value=Some(42.0)
///             value_max=60.0
///             label=GaugeLabel::Text("42 min".into())
///             aria_value_text="42 minutes of 60 remaining"
///             width=400.0
///             height=400.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "gauge",
    preview_label = "Gauge",
    preview_icon = icondata::AiDashboardOutlined,
)]
#[component]
pub fn Gauge(
    /// Current metric; `None` hides value arc and label.
    #[prop(default = None)]
    value: Option<f64>,
    /// Scale minimum.
    #[prop(default = 0.0)]
    value_min: f64,
    /// Scale maximum.
    #[prop(default = 100.0)]
    value_max: f64,
    /// Arc start angle in degrees.
    #[prop(default = 0.0)]
    start_angle: f64,
    /// Arc end angle in degrees.
    #[prop(default = 360.0)]
    end_angle: f64,
    /// Inner radius (px number or percent string).
    #[prop(optional, into, default = String::new())]
    inner_radius: String,
    /// Outer radius (px number or percent string).
    #[prop(optional, into, default = String::new())]
    outer_radius: String,
    /// Center label text or formatter.
    #[prop(default = GaugeLabel::default())]
    label: GaugeLabel,
    /// Accessible value description for screen readers.
    #[prop(optional, into)]
    aria_value_text: Option<String>,
    /// Chart width in pixels.
    #[prop(default = 200.0)]
    width: f64,
    /// Chart height in pixels.
    #[prop(default = 200.0)]
    height: f64,
    /// Skip animations.
    #[prop(optional)]
    skip_animation: Option<bool>,
    /// Chart motion configuration.
    #[prop(optional)]
    motion: Option<ChartMotion>,
    /// Value arc fill color override.
    #[prop(optional, into)]
    value_color: Option<String>,
    /// Custom overlay children (use [`use_gauge_state`]).
    #[prop(optional)]
    children: Option<Children>,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let geometry = GaugeGeometry {
        inner_radius: parse_gauge_radius(&inner_radius, PieRadius::Percent(80.0)),
        outer_radius: parse_gauge_radius(&outer_radius, PieRadius::Percent(100.0)),
        start_angle,
        end_angle,
        ..Default::default()
    };

    let label_text = Memo::new(move |_| match &label {
        GaugeLabel::Text(s) => s.clone(),
        GaugeLabel::Formatter(cb) => cb.run((value, value_min, value_max)),
    });

    let show_label = value.is_some();

    view! {
        <GaugeContainer
            value=value
            value_min=value_min
            value_max=value_max
            geometry=geometry
            width=width
            height=height
            skip_animation=skip_animation
            motion=motion
            value_color=value_color
            aria_value_text=aria_value_text
            class=class
        >
            {show_label.then(|| {
                let text = label_text;
                view! {
                    <foreignObject
                        x="0"
                        y="0"
                        width=width
                        height=height
                        style="pointer-events: none;"
                    >
                        <div
                            class="orb-gauge-value-host"
                            style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;"
                        >
                            <orbital_core_components::Text
                                font=orbital_core_components::TextFont::Numeric
                            >
                                {move || text.get()}
                            </orbital_core_components::Text>
                        </div>
                    </foreignObject>
                }
            })}
            {children.map(|c| c())}
        </GaugeContainer>
    }
}
