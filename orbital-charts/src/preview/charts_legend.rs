//! Legend preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// The legend tells users which color belongs to which series or slice.
///
/// Adjust placement and mark size with legend props, or add a color-bar legend when values
/// map to a continuous scale (see `charts-styling` and heatmap previews).
///
/// # When to use
///
/// - Multi-series cartesian charts where color alone is ambiguous.
/// - Dashboards where users toggle series visibility to reduce clutter.
/// - Heatmaps or styled axes that need a color-scale key (continuous legend).
///
/// # Usage
///
/// 1. Set `legend=LegendConfig::default()` on any `*Chart` or [`ChartContainer`].
/// 2. Wire `on_legend_click` when the parent must react to visibility toggles.
/// 3. Pair with `tooltip` so hidden series do not leave unexplained colors on the plot.
///
/// # Best Practices
///
/// ## Do's
///
/// * Place the legend in card padding — Orbital defaults keep it out of the plot area.
/// * Use click-to-toggle when users routinely hide noisy series.
/// * Keep legend labels in sync with tooltip text via shared `label` or `label_formatter`.
///
/// ## Don'ts
///
/// * Do not show a legend for single-series charts unless the label adds necessary context.
/// * Do not duplicate legend examples on every chart type — link here from chart roots.
///
/// # Examples
///
/// ## Series legend with visibility toggles
/// Click legend entries to show or hide series. `highlight_scope` pairs with hover fade so
/// the active series stays visually dominant — see `charts-highlighting` for fade modes.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::{BarChart, HighlightScope, LegendConfig, TooltipConfig};
/// view! {
///     <div data-testid="charts-legend-preview">
///         <BarChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             legend=LegendConfig::default()
///             tooltip=TooltipConfig::item()
///             highlight_scope=HighlightScope::default()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-legend",
    preview_label = "Charts Legend",
    preview_icon = icondata::AiUnorderedListOutlined,
)]
#[component]
pub fn ChartsLegendPreview() -> impl IntoView {
    view! { () }
}
