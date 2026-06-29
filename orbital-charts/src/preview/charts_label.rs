//! Label preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::preview::fixtures::{cost_series, revenue_series};

/// Names in the legend and tooltip come from the same series label field.
///
/// Pass a string for a fixed name, or a `label_formatter` when the wording should change
/// by context (`Legend`, `Tooltip`, or pie `Arc`).
///
/// # When to use
///
/// - Short legend labels with longer tooltip descriptions (e.g. "Rev" vs "Revenue (USD)").
/// - Pie arc labels that differ from legend text for space reasons.
/// - Consistent naming across legend, tooltip, and export without duplicating series defs.
///
/// # Usage
///
/// 1. Set `label` on [`SeriesDef`] for a fixed name everywhere.
/// 2. Set `label_formatter` with [`LabelLocation`] when text should vary by surface.
/// 3. Configure `arc_label` on [`PieChart`] for slice-specific label modes.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep legend labels short; put units and qualifiers in tooltip formatters.
/// * Use one formatter callback per series rather than maintaining parallel strings.
/// * Test arc labels with `min_angle` so thin slices do not collide.
///
/// ## Don'ts
///
/// * Do not hard-code tooltip strings in app code when `label_formatter` can centralize them.
/// * Do not show arc labels on very small slices without `min_angle` guards.
///
/// # Examples
///
/// ## Location-aware bar labels
/// Abbreviated legend entries with full tooltip names.
/// <!-- preview -->
/// ```rust
/// use leptos::callback::Callback;
/// use leptos::prelude::*;
/// use crate::preview::fixtures::{
///     cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis,
/// };
/// use crate::{BarChart, HighlightScope, LabelLocation, LegendConfig, TooltipConfig};
/// let mut revenue = revenue_series();
/// revenue.label_formatter = Some(Callback::new(|(loc,): (LabelLocation,)| match loc {
///     LabelLocation::Legend => "Rev".into(),
///     LabelLocation::Tooltip => "Revenue (USD)".into(),
///     LabelLocation::Arc => "Revenue".into(),
/// }));
/// let mut cost = cost_series();
/// cost.label_formatter = Some(Callback::new(|(loc,): (LabelLocation,)| match loc {
///     LabelLocation::Legend => "Cost".into(),
///     LabelLocation::Tooltip => "Operating cost".into(),
///     LabelLocation::Arc => "Cost".into(),
/// }));
/// view! {
///     <div data-testid="charts-label-preview" style="min-width:560px;min-height:320px;">
///         <BarChart
///             series=vec![revenue, cost]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             legend=LegendConfig::default()
///             tooltip=TooltipConfig::item()
///             highlight_scope=HighlightScope::default()
///             width=560.0
///             height=280.0
///         />
///     </div>
/// }
/// ```
///
/// ## Pie arc labels
/// Formatted arc values on slices above the minimum angle threshold.
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::{market_share_pie_slices, market_share_x_axis};
/// use crate::{ArcLabelMode, PieArcLabelConfig, PieChart};
/// view! {
///     <div data-testid="charts-label-pie-preview" style="min-width:560px;min-height:320px;">
///         <PieChart
///             series=vec![market_share_pie_slices()]
///             x_axis=vec![market_share_x_axis()]
///             arc_label=PieArcLabelConfig {
///                 mode: Some(ArcLabelMode::Label),
///                 min_angle: Some(15.0),
///                 ..Default::default()
///             }
///             width=400.0
///             height=280.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-label",
    preview_label = "Charts Label",
    preview_icon = icondata::AiFontSizeOutlined,
)]
#[component]
pub fn ChartsLabelPreview() -> impl IntoView {
    view! { () }
}

/// Hidden export so multi-series label fixtures stay referenced in non-preview builds.
#[allow(dead_code)]
pub fn _label_multi_series() -> Vec<crate::SeriesDef> {
    vec![revenue_series(), cost_series()]
}
