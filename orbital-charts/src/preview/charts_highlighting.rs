//! Highlighting preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Highlighting gives immediate visual feedback — crosshairs along axes, emphasis on the
/// hovered series or point, and optional fading of everything else.
///
/// Axis highlighting draws a crosshair or category band where the pointer lands. Series
/// highlighting dims everything except the hovered point or series — configure both per
/// series, or drive selection from outside to link charts together.
///
/// # When to use
///
/// - Dense multi-series plots where hover must isolate one mark or category.
/// - Dashboards that sync highlight across multiple charts via controlled `highlighted_item`.
/// - Axis exploration with band crosshairs and axis-triggered tooltips.
///
/// # Usage
///
/// 1. Set `highlight_scope` with `highlight` and `fade` modes on the chart or per series.
/// 2. Enable `axis_highlight` for band or line crosshairs on cartesian charts.
/// 3. Pair with `tooltip=TooltipConfig::axis()` when the crosshair should list all series at x.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `FadeMode::Global` when one hovered item should de-emphasize the rest of the plot.
/// * Drive `highlighted_item` externally when linking two charts to the same selection.
/// * Test highlight contrast in both light and dark themes.
///
/// ## Don'ts
///
/// * Do not enable axis highlight without axis tooltips if users expect values at the crosshair.
/// * Do not stack aggressive fade with already-low-contrast palette colors.
///
/// # Examples
///
/// ## Global fade and axis crosshair
/// Hover a bar to fade siblings; the axis band follows the pointer. Combines item highlight,
/// global fade, axis band, and axis tooltip — the common interactive dashboard pattern.
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::{
///     AxisHighlightConfig, BarChart, FadeMode, HighlightMode, HighlightScope, TooltipConfig,
/// };
/// view! {
///     <div data-testid="charts-highlighting-preview" style="min-width: 560px; min-height: 320px;">
///         <BarChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             highlight_scope=HighlightScope {
///                 highlight: HighlightMode::Item,
///                 fade: FadeMode::Global,
///             }
///             axis_highlight=AxisHighlightConfig::bar_default()
///             tooltip=TooltipConfig::axis()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-highlighting",
    preview_label = "Charts Highlighting",
    preview_icon = icondata::AiHighlightOutlined,
)]
#[component]
pub fn ChartsHighlightingPreview() -> impl IntoView {
    view! { () }
}
