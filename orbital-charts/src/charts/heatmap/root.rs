//! [`Heatmap`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::context::{ChartKind, HeatmapPlotContext, HeatmapPlotProvider};
use crate::engine::default_continuous_scale;
use crate::shared::{ChartContainer, ContinuousColorLegend, HeatmapPlot};
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, AxisPosition, ColorScale, FadeMode, HeatmapCell,
    HighlightMode, HighlightScope, LegendConfig, PlotInset, ScaleType, TooltipConfig,
};

/// Two-dimensional categorical heatmap with z-axis color scale.
///
/// Map a numeric value to color for every combination of two categorical axes. Supply
/// x/y category lists plus `[x_index, y_index, value]` cell tuples, then tune the
/// color scale on the z axis.
///
/// # When to use
///
/// - Calendar grids, correlation matrices, or category × metric intensity tables.
/// - Severity or utilization bands via piecewise `color_scale` thresholds.
/// - Dashboard cells where exact values matter on hover — enable `tooltip`.
///
/// # Usage
///
/// 1. Pass `x_categories`, `y_categories`, and sparse or dense `cells` tuples (0-based indexes).
/// 2. Set `color_scale` to continuous (default) or piecewise for severity buckets.
/// 3. Enable `legend` for a color-bar key; `tooltip` for per-cell values.
/// 4. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use continuous color scales for magnitude; piecewise scales for named severity bands.
/// * Keep category label arrays short — heatmaps grow with x × y cell count.
/// * Link to `charts-styling` for palette and gradient patterns on cartesian charts.
///
/// ## Don'ts
///
/// * Do not use heatmaps for time-series lines — prefer [`crate::LineChart`].
/// * Do not rely on color alone — pair tooltips or cell labels for exact values.
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-legend`, `charts-tooltip`, `charts-highlighting`, `charts-styling`.
///
/// # Examples
///
/// ## Basic heatmap
/// Region × hour grid with default continuous color scale. Tuple indexes refer to positions
/// in the category arrays — sparse tuples omit empty cells.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::Heatmap;
/// use crate::preview::fixtures::{heatmap_cells, heatmap_x_categories, heatmap_y_categories};
/// view! {
///     <div data-testid="heatmap-preview">
///         <Heatmap
///             x_categories=heatmap_x_categories()
///             y_categories=heatmap_y_categories()
///             cells=heatmap_cells()
///             width=680.0
///             height=418.0
///         />
///     </div>
/// }
/// ```
///
/// ## Piecewise color scale
/// Bucketed colors via `thresholds` for low/medium/high severity. Thresholds split the
/// z domain into bands — each band maps to the next color in `colors`.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::Heatmap;
/// use crate::preview::fixtures::{heatmap_cells, heatmap_x_categories, heatmap_y_categories};
/// use crate::{ColorScale, ColorScaleKind};
/// view! {
///     <div data-testid="heatmap-piecewise-preview">
///         <Heatmap
///             x_categories=heatmap_x_categories()
///             y_categories=heatmap_y_categories()
///             cells=heatmap_cells()
///             color_scale=ColorScale {
///                 kind: ColorScaleKind::Piecewise,
///                 colors: vec!["#dbeafe".into(), "#3b82f6".into(), "#1e3a8a".into()],
///                 thresholds: Some(vec![20.0, 50.0]),
///             }
///             width=680.0
///             height=418.0
///         />
///     </div>
/// }
/// ```
///
/// ## Tooltip, legend, and cell highlight
/// Color-scale legend, item tooltip on cell hover, and global fade. See `charts-tooltip`
/// and `charts-highlighting` for trigger and fade patterns shared across chart types.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::Heatmap;
/// use crate::preview::fixtures::{heatmap_cells, heatmap_x_categories, heatmap_y_categories};
/// use crate::{FadeMode, HighlightMode, HighlightScope, LegendConfig, TooltipConfig};
/// view! {
///     <div data-testid="heatmap-interaction-preview">
///         <Heatmap
///             x_categories=heatmap_x_categories()
///             y_categories=heatmap_y_categories()
///             cells=heatmap_cells()
///             legend=LegendConfig::default()
///             tooltip=TooltipConfig::item()
///             highlight_scope=HighlightScope {
///                 highlight: HighlightMode::Item,
///                 fade: FadeMode::Global,
///             }
///             width=680.0
///             height=418.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "heatmap",
    preview_label = "Heatmap",
    preview_icon = icondata::AiHeatMapOutlined,
)]
#[component]
pub fn Heatmap(
    /// X-axis category labels.
    x_categories: Vec<String>,
    /// Y-axis category labels.
    y_categories: Vec<String>,
    /// Cell tuples `[x_index, y_index, value]`.
    cells: Vec<HeatmapCell>,
    /// Z-axis color scale.
    #[prop(optional)]
    color_scale: Option<ColorScale>,
    /// Optional color domain minimum.
    #[prop(optional)]
    value_min: Option<f64>,
    /// Optional color domain maximum.
    #[prop(optional)]
    value_max: Option<f64>,
    /// Chart width in pixels.
    #[prop(default = 520.0)]
    width: f64,
    /// Chart height in pixels.
    #[prop(default = 320.0)]
    height: f64,
    /// Plot inset.
    #[prop(optional)]
    margin: Option<PlotInset>,
    /// Loading overlay.
    #[prop(optional)]
    loading: Option<bool>,
    /// Highlight and fade scope for cells.
    #[prop(optional)]
    highlight_scope: Option<HighlightScope>,
    /// Axis highlight configuration.
    #[prop(optional)]
    axis_highlight: Option<AxisHighlightConfig>,
    /// Continuous color scale legend configuration.
    #[prop(optional)]
    legend: Option<LegendConfig>,
    /// Tooltip configuration.
    #[prop(optional)]
    tooltip: Option<TooltipConfig>,
    /// Fired when an x-axis category band is clicked.
    #[prop(optional)]
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let scale = color_scale.unwrap_or_else(default_continuous_scale);
    let renderer = if cells.len() > crate::engine::HEATMAP_CANVAS_THRESHOLD {
        "canvas"
    } else {
        "svg"
    };

    let x_axis = vec![AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(x_categories),
        position: AxisPosition::Bottom,
        category_gap_ratio: Some(0.08),
        ..Default::default()
    }];
    let y_axis = vec![AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Band,
        data: Some(y_categories),
        position: AxisPosition::Left,
        category_gap_ratio: Some(0.08),
        ..Default::default()
    }];

    let plot_context = HeatmapPlotContext {
        cells,
        color_scale: scale,
        value_min,
        value_max,
    };

    let inset = margin.unwrap_or_else(PlotInset::with_axes);
    let highlight_scope = highlight_scope.or({
        Some(HighlightScope {
            highlight: HighlightMode::Item,
            fade: FadeMode::Global,
        })
    });
    let tooltip = tooltip.or_else(|| Some(TooltipConfig::item()));

    let shell_style = format!("width: {width}px; height: {height}px;");

    view! {
        <div data-orbital-heatmap-renderer=renderer class="orb-heatmap-root">
            <HeatmapPlotProvider context=plot_context>
                <div class="orb-heatmap-shell" style=shell_style>
                    <ChartContainer
                        class=class
                        x_axis=Some(x_axis)
                        y_axis=Some(y_axis)
                        width=Some(width)
                        height=Some(height)
                        margin=Some(inset)
                        loading=loading
                        chart_kind=ChartKind::Heatmap
                        highlight_scope=highlight_scope
                        axis_highlight=axis_highlight
                        tooltip=tooltip
                        on_axis_click=on_axis_click
                    >
                        <HeatmapPlot />
                    </ChartContainer>
                    {legend
                        .filter(|c| !c.hidden)
                        .map(|config| view! { <ContinuousColorLegend config=config /> })}
                </div>
            </HeatmapPlotProvider>
        </div>
    }
}
