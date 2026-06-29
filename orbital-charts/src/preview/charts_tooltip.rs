//! Tooltip preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Tooltips show the exact values behind chart marks — for one item or every series at a shared axis position.
///
/// Choose item mode to describe one mark, or axis mode to compare every series at the same x position.
/// Format values with series and axis formatters, or supply custom content when you need extra columns.
/// Axis tooltips require axis tracking on the chart container.
///
/// # When to use
///
/// - Dashboards where users need precise values on hover without cluttering the plot.
/// - Axis crosshair tooltips that list all series at one category or time index.
/// - Currency or unit formatting via `value_formatter` on series and axes.
///
/// # Usage
///
/// 1. Set `tooltip=TooltipConfig::item()` for mark-level values (bars, points, slices).
/// 2. Set `tooltip=TooltipConfig::axis()` for multi-series comparison at a shared x position.
/// 3. Wire `value_formatter` on series and `tick_format` on axes for domain-specific labels.
/// 4. Use composition mode with `ChartTooltip` when you need fully custom tooltip panels.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use item mode for sparse plots where one mark maps to one story.
/// * Use axis mode when comparing all series at the same category is the primary task.
/// * Keep tooltip content concise — link to detail views for long explanations.
///
/// ## Don'ts
///
/// * Do not enable axis tooltips if `disable_axis_listener` is set on the container.
/// * Do not duplicate tooltip setup on every chart root — configure once per dashboard surface.
///
/// # Examples
///
/// ## Item tooltip on bar chart
/// Hover a single bar to see mark-level values.
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::{BarChart, HighlightScope, TooltipConfig};
/// view! {
///     <div data-testid="charts-tooltip-preview" style="min-width:560px;min-height:320px;">
///         <BarChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             tooltip=TooltipConfig::item()
///             highlight_scope=HighlightScope::default()
///             width=560.0
///             height=280.0
///         />
///     </div>
/// }
/// ```
///
/// ## Axis tooltip on line chart
/// Compare every series at the hovered category.
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::{LineChart, TooltipConfig};
/// view! {
///     <div data-testid="charts-tooltip-axis-preview" style="min-width:560px;min-height:320px;">
///         <LineChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             tooltip=TooltipConfig::axis()
///             width=560.0
///             height=280.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-tooltip",
    preview_label = "Charts Tooltip",
    preview_icon = icondata::AiInfoCircleOutlined,
)]
#[component]
pub fn ChartsTooltipPreview() -> impl IntoView {
    view! { () }
}
