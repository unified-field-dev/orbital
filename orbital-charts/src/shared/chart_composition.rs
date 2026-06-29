//! Composition mode preview — mixed charts and custom SVG layers.

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::preview::fixtures::{
    full_grid, mixed_bar_line_series, mixed_bar_line_x_axis, revenue_y_axis,
};
use crate::shared::{BarPlot, ChartContainer, ChartCustomBaseline, LinePlot, PlotClip};

/// Wrap your data definitions in a chart container, then stack plot, axis, and interaction components as children.
///
/// Explicit `chart_type` on each [`SeriesDef`] tells plot layers which geometry to render;
/// child order controls z-order (bars beneath lines, annotations on top).
///
/// # When to use
///
/// Build mixed-type dashboards by placing explicit plot children inside
/// [`ChartContainer`]. Each [`SeriesDef`] must declare `chart_type`; child
/// order controls z-order.
///
/// # Usage
///
/// 1. Place [`ChartContainer`] at the root with series, axes, and optional `dataset` binding.
/// 2. Add plot children (`BarPlot`, `LinePlot`, …) with required `chart_type` on each series.
/// 3. Wrap plots in [`PlotClip`] when marks extend past plot bounds.
/// 4. Add [`ChartCustomBaseline`] or custom SVG using scale hooks for annotations.
///
/// # Best Practices
///
/// ## Do's
///
/// * Place [`BarPlot`] before [`LinePlot`] when bars should sit beneath lines.
/// * Use unique clip path ids per chart instance (`orb-clip-*` prefix).
/// * Prefer [`ResponsiveChartContainer`] for dashboard tiles that fill parent width.
/// * Respect [`ChartCompositionOrder`]: first plot child inside [`PlotClip`] renders on the bottom.
///
/// ## Don'ts
///
/// * Do not omit `chart_type` on series in composition mode — inference is disabled.
/// * Do not mix composition and convenience `*Chart` wrappers on the same surface.
/// * Do not place legend or tooltip as composition children — configure them via container props.
///
/// # Examples
///
/// ## Mixed bar and line chart
/// Revenue bars with a target line overlay, clipped to the plot area. Demonstrates z-order,
/// mixed `chart_type` series, and [`PlotClip`] for overflow control.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ChartComposition;
/// use crate::preview::fixtures::{full_grid, mixed_bar_line_series, mixed_bar_line_x_axis, revenue_y_axis};
/// view! {
///     <div data-testid="chart-composition-preview">
///         <ChartComposition
///             variant=ChartCompositionVariant::MixedBarLine
///             series=mixed_bar_line_series()
///             x_axis=vec![mixed_bar_line_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Custom SVG layer via hooks
/// Dashed revenue baseline using [`use_drawing_area`] and [`use_y_scale`]. Use when annotations
/// must stay aligned to data coordinates on resize.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ChartComposition;
/// use crate::preview::fixtures::{quarter_x_axis, revenue_series, revenue_y_axis};
/// view! {
///     <div data-testid="chart-composition-custom-layer-preview">
///         <ChartComposition
///             variant=ChartCompositionVariant::CustomLayer
///             series=vec![revenue_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "chart-composition",
    preview_label = "Chart Composition",
    preview_icon = icondata::AiAppstoreOutlined,
)]
#[component]
pub fn ChartComposition(
    /// Which documented composition example to render.
    #[prop(default = ChartCompositionVariant::MixedBarLine)]
    variant: ChartCompositionVariant,
    /// Series definitions with explicit `chart_type` per entry.
    #[prop(default = mixed_bar_line_series())]
    series: Vec<crate::SeriesDef>,
    /// X-axis definitions.
    #[prop(default = vec![mixed_bar_line_x_axis()])]
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
    let _ = &class;

    match variant {
        ChartCompositionVariant::MixedBarLine => view! {
            <ChartContainer
                series=Some(series)
                x_axis=Some(x_axis)
                y_axis=Some(y_axis)
                grid=Some(grid)
                width=Some(width)
                height=Some(height)
            >
                <PlotClip id="orb-clip-composition".to_string()>
                    <BarPlot />
                    <LinePlot />
                </PlotClip>
            </ChartContainer>
        },
        ChartCompositionVariant::CustomLayer => view! {
            <ChartContainer
                series=Some(series)
                x_axis=Some(x_axis)
                y_axis=Some(y_axis)
                width=Some(width)
                height=Some(height)
            >
                <BarPlot />
                <ChartCustomBaseline />
            </ChartContainer>
        },
    }
}

/// Composition preview variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartCompositionVariant {
    /// Bar + line overlay with clip path.
    #[default]
    MixedBarLine,
    /// Custom SVG layer using chart context hooks.
    CustomLayer,
}
