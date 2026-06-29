//! Zoom and pan preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::{AxisDef, ZoomConfig};

/// Build a monthly x-axis with zoom enabled for preview examples.
pub fn zoomable_monthly_x_axis() -> AxisDef {
    use crate::preview::fixtures::monthly_x_axis;
    let mut axis = monthly_x_axis();
    axis.zoom = Some(ZoomConfig {
        enabled: true,
        min_span: Some(10.0),
        ..Default::default()
    });
    axis
}

/// Zoom and pan let users focus on part of a dense chart — scroll to zoom, drag to pan, pinch on touch devices.
///
/// Turn it on per axis with `ZoomConfig`, constrain span with `min_span`, and optionally
/// sync companion axes with `filter_mode` when the vertical axis should follow the visible horizontal range.
///
/// # When to use
///
/// - Long time series where the default view is too dense to read.
/// - Exploration dashboards where users drill into a date range before taking action.
/// - Line and bar charts with `ChartFeatures::ZOOM_PAN` enabled on the chart root.
///
/// # Usage
///
/// 1. Set `zoom` config on the axis definition (`enabled: true`, optional `min_span`).
/// 2. Enable `features=ChartFeatures::ZOOM_PAN` on the chart component.
/// 3. Hold zoom state in an `RwSignal<Vec<ZoomWindow>>` for controlled mode and reset buttons.
/// 4. Wire `on_zoom_change` when parent state must persist across re-renders.
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide a reset control when users can zoom far enough to lose context.
/// * Keep `min_span` wide enough that ticks remain legible.
/// * Test pinch and scroll on trackpads and touch devices separately.
///
/// ## Don'ts
///
/// * Do not enable zoom on sparklines or gauges — it applies to cartesian charts only.
/// * Do not assume keyboard range selection; gesture zoom is the supported path today.
///
/// # Accessibility
///
/// - Chart root: `role="img"`, zoom-aware `aria-label`, `Escape` dismisses tooltips.
/// - Reference lines: `aria-hidden` with SVG `<title>` for labels.
/// - Zoom is gesture-only; keyboard range selection deferred per CH-24.
/// - Gauge meter roles unchanged from Phase 4.
///
/// # Examples
///
/// ## Zoom and pan on line and bar charts
/// Scroll wheel zooms the monthly x-axis; drag pans; Reset restores the full domain.
/// Controlled `zoom` signal is shared between line and bar so both stay in sync.
/// <!-- preview -->
/// ```rust,ignore
/// use leptos::prelude::*;
/// use orbital_core_components::Button;
/// use crate::preview::fixtures::{full_grid, monthly_revenue_series, monthly_y_axis};
/// use crate::{BarChart, ChartFeatures, LineChart, ZoomWindow};
/// let zoom = RwSignal::new(vec![ZoomWindow::full("x")]);
/// let x_axis = zoomable_monthly_x_axis();
/// view! {
///     <div data-testid="charts-zoom-pan-preview" style="display:flex;flex-direction:column;gap:1rem;">
///         <div style="display:flex;gap:0.5rem;align-items:center;">
///             <Button
///                 on_click=Callback::new(move |_| {
///                     zoom.set(vec![ZoomWindow::full("x")]);
///                 })
///             >
///                 "Reset zoom"
///             </Button>
///         </div>
///         {move || {
///             let windows = zoom.get();
///             view! {
///                 <LineChart
///                     series=vec![monthly_revenue_series()]
///                     x_axis=vec![x_axis.clone()]
///                     y_axis=vec![monthly_y_axis()]
///                     grid=full_grid()
///                     features=ChartFeatures::ZOOM_PAN | ChartFeatures::ANIMATION
///                     zoom=windows.clone()
///                     on_zoom_change=Callback::new(move |(w,)| zoom.set(w))
///                     width=640.0
///                     height=280.0
///                 />
///                 <BarChart
///                     series=vec![monthly_revenue_series()]
///                     x_axis=vec![x_axis.clone()]
///                     y_axis=vec![monthly_y_axis()]
///                     grid=full_grid()
///                     features=ChartFeatures::ZOOM_PAN | ChartFeatures::ANIMATION
///                     zoom=windows
///                     on_zoom_change=Callback::new(move |(w,)| zoom.set(w))
///                     width=640.0
///                     height=280.0
///                 />
///             }
///         }}
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-zoom-pan",
    preview_label = "Charts Zoom Pan",
    preview_icon = icondata::AiZoomInOutlined,
)]
#[component]
pub fn ChartsZoomPan() -> impl IntoView {
    view! { () }
}
