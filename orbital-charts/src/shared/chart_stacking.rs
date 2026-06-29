//! Stacking preview — stacked bars and stacked areas.

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::preview::fixtures::{
    full_grid, month_categories, revenue_y_axis, stacked_bar_series, stacked_bar_x_axis,
};
use crate::shared::{AreaPlot, BarPlot, ChartContainer};

/// Stack series when each category total is meaningful and segments show contribution.
///
/// **Decision guide:** Same `stack_group` id → stacked. Negative values → `StackOffset::Diverging`
/// (bar default). Share of whole → `StackOffset::Expand`. Unsure about layer order → try
/// `StackOrder::Ascending`.
///
/// # When to use
///
/// Assign the same `stack_group` on multiple series to stack segments per
/// category. Bar stacks default to [`StackOffset::Diverging`] for signed data;
/// lines and areas default to [`StackOffset::None`].
///
/// # Usage
///
/// 1. Assign the same `stack_group` string on series that should stack per category.
/// 2. Set `stack_offset` on any member — `Diverging` for signed bars, `Expand` for 100% stacks.
/// 3. Set `stack_order` when layer sequence at the baseline matters.
/// 4. Use [`ChartStacking`] preview variants to compare bar vs area stacking side by side.
///
/// # Best Practices
///
/// ## Do's
///
/// * Include zero in the value axis domain for diverging stacks with negatives.
/// * Use `stack_order: Ascending` when the smallest segment should anchor the baseline.
/// * Link to `area-chart` percent stacked example for normalized area stacks.
///
/// ## Don'ts
///
/// * Do not stack unrelated series — they must share categories and a stack group id.
/// * Do not use expand offset when absolute totals must remain readable.
///
/// # Examples
///
/// ## Stacked bar chart (diverging default)
/// Three stacked segments per month including signed adjustments. Bar stacks default to
/// diverging offset so negative segments mirror below the baseline.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ChartStacking;
/// use crate::preview::fixtures::{full_grid, revenue_y_axis, stacked_bar_series, stacked_bar_x_axis};
/// view! {
///     <div data-testid="chart-stacking-preview">
///         <ChartStacking
///             variant=ChartStackingVariant::StackedBar
///             series=stacked_bar_series()
///             x_axis=vec![stacked_bar_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Stacked area chart
/// Multiple filled series sharing a stack group. Compare with `area-chart-percent-preview`
/// when you need `stack_offset: Expand` normalization.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ChartStacking;
/// use crate::preview::fixtures::{full_grid, month_categories, quarter_x_axis, revenue_y_axis, stacked_area_series};
/// view! {
///     <div data-testid="chart-stacking-area-preview">
///         <ChartStacking
///             variant=ChartStackingVariant::StackedArea
///             series=stacked_area_series()
///             x_axis=vec![{
///                 let mut a = quarter_x_axis();
///                 a.data = Some(month_categories());
///                 a
///             }]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "chart-stacking",
    preview_label = "Chart Stacking",
    preview_icon = icondata::AiBlockOutlined,
)]
#[component]
pub fn ChartStacking(
    /// Which stacking example to render.
    #[prop(default = ChartStackingVariant::StackedBar)]
    variant: ChartStackingVariant,
    /// Series with shared `stack_group` identifiers.
    #[prop(default = stacked_bar_series())]
    series: Vec<crate::SeriesDef>,
    /// X-axis definitions.
    #[prop(default = vec![stacked_bar_x_axis()])]
    x_axis: Vec<crate::AxisDef>,
    /// Y-axis definitions.
    #[prop(default = vec![revenue_y_axis()])]
    y_axis: Vec<crate::AxisDef>,
    /// Background grid configuration.
    #[prop(default = full_grid())]
    grid: crate::GridConfig,
    /// Chart width in pixels.
    #[prop(default = 560.0)]
    width: f64,
    /// Chart height in pixels.
    #[prop(default = 320.0)]
    height: f64,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let _ = (&class, month_categories);

    match variant {
        ChartStackingVariant::StackedBar => view! {
            <ChartContainer
                series=Some(series)
                x_axis=Some(x_axis)
                y_axis=Some(y_axis)
                grid=Some(grid)
                width=Some(width)
                height=Some(height)
            >
                <BarPlot />
            </ChartContainer>
        },
        ChartStackingVariant::StackedArea => view! {
            <ChartContainer
                series=Some(series)
                x_axis=Some(x_axis)
                y_axis=Some(y_axis)
                grid=Some(grid)
                width=Some(width)
                height=Some(height)
            >
                <AreaPlot />
            </ChartContainer>
        },
    }
}

/// Stacking preview variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartStackingVariant {
    /// Diverging stacked bars with signed values.
    #[default]
    StackedBar,
    /// Stacked area fills with gradient.
    StackedArea,
}
