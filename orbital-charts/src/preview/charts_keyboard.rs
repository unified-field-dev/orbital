//! Keyboard navigation and line x-domain preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::preview::fixtures::{cost_series, revenue_series};

/// Keyboard navigation moves focus between marks with Arrow, Home, and End. Escape clears
/// the active highlight. Line charts default to a strict inferred x-axis domain; override
/// with `AxisDef.domain_limit` when nice padding is preferred.
///
/// # When to use
///
/// - Dashboards that must be operable without a pointer.
/// - Verifying roving focus and tooltip sync on line marks.
/// - Comparing strict vs nice x-axis domain limits on line cartesian charts.
///
/// # Usage
///
/// 1. Focus the chart root (`tabindex=0`) and press Arrow keys to move between marks.
/// 2. Set `keyboard_navigation=false` or clear `ChartFeatures::KEYBOARD_NAV` to opt out.
/// 3. Set `show_markers: true` when visible marks help keyboard discovery (off by default).
///
/// # Best Practices
///
/// ## Do's
///
/// * Pair keyboard nav with `tooltip=TooltipConfig::item()` so focus shows values.
/// * Opt into `show_markers` on sparse line series used in keyboard-first flows.
///
/// ## Don'ts
///
/// * Do not expect keyboard zoom — range selection via keys is deferred (CH-24).
/// * Do not rely on keyboard nav when mark geometry is omitted (lazy render above 500 points).
///
/// # Examples
///
/// ## Keyboard navigation on a line chart
/// Arrow keys move between marks; Home and End jump to the first and last points.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{
///     domain_demo_nice_x_axis, domain_demo_series, domain_demo_y_axis, full_grid, quarter_x_axis,
///     revenue_series, revenue_y_axis,
/// };
/// use crate::{
///     ChartContainer, ChartOrientation, FadeMode, HighlightMode, HighlightScope, LineChart, LinePlot,
///     TooltipConfig,
/// };
/// let highlight = HighlightScope {
///     highlight: HighlightMode::Item,
///     fade: FadeMode::Global,
/// };
/// view! {
///     <div style="display:flex;flex-direction:column;gap:1.5rem;">
///         <div data-testid="charts-keyboard-preview">
///             <p>"Focus the chart and use Arrow keys, Home, and End. Escape clears highlight."</p>
///             <LineChart
///                 series=vec![revenue_series()]
///                 x_axis=vec![quarter_x_axis()]
///                 y_axis=vec![revenue_y_axis()]
///                 grid=full_grid()
///                 tooltip=TooltipConfig::item()
///                 highlight_scope=highlight
///                 width=560.0
///                 height=280.0
///             />
///         </div>
///         <div data-testid="charts-keyboard-domain-preview" style="display:flex;gap:1rem;flex-wrap:wrap;">
///             <div data-testid="charts-keyboard-domain-strict-preview" style="flex:1;min-width:240px;">
///                 <p>"Inferred x-axis: Strict domain (line default)"</p>
///                 <ChartContainer
///                     series=Some(vec![domain_demo_series()])
///                     y_axis=Some(vec![domain_demo_y_axis()])
///                     orientation=ChartOrientation::Horizontal
///                     prefer_line_x_strict=true
///                     grid=Some(full_grid())
///                     width=Some(280.0)
///                     height=Some(240.0)
///                 >
///                     <LinePlot />
///                 </ChartContainer>
///             </div>
///             <div data-testid="charts-keyboard-domain-nice-preview" style="flex:1;min-width:240px;">
///                 <p>"Explicit x-axis: Nice domain override"</p>
///                 <ChartContainer
///                     series=Some(vec![domain_demo_series()])
///                     x_axis=Some(vec![domain_demo_nice_x_axis()])
///                     y_axis=Some(vec![domain_demo_y_axis()])
///                     orientation=ChartOrientation::Horizontal
///                     prefer_line_x_strict=true
///                     grid=Some(full_grid())
///                     width=Some(280.0)
///                     height=Some(240.0)
///                 >
///                     <LinePlot />
///                 </ChartContainer>
///             </div>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-keyboard",
    preview_label = "Charts Keyboard",
    preview_icon = icondata::AiEnterOutlined,
)]
#[component]
pub fn ChartsKeyboardPreview() -> impl IntoView {
    view! { () }
}

/// Hidden export so multi-series keyboard fixtures stay referenced in non-preview builds.
#[allow(dead_code)]
pub fn _keyboard_multi_series() -> Vec<crate::SeriesDef> {
    vec![revenue_series(), cost_series()]
}
