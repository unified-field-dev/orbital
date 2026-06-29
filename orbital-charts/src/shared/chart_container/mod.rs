//! [`ChartContainer`] — fixed-size chart context provider.

mod chart_root;
mod custom_layer;
mod keyboard_listener;
mod overlay_layer;
mod overlays;
mod plot_area;
mod resize;
mod responsive;

use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::context::ChartKind;
use crate::engine::default_plot_inset;
#[cfg(feature = "preview")]
use crate::preview::fixtures::{
    cost_series, full_grid, processed_binding, processed_dataset, quarter_x_axis, revenue_series,
    revenue_y_axis,
};
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, ChartEmbedMode, ChartFeatures, ChartItemId,
    ChartMotion, ChartOrientation, GridConfig, HighlightScope, LegendConfig, OrbitalChartPalette,
    OrbitalChartsTheme, OverlayMount, PlotInset, SeriesDef, TooltipConfig, ZoomWindow,
    CHART_FEATURES_DEFAULT,
};
use leptos::callback::Callback;

pub use chart_root::ChartRoot;
pub use custom_layer::ChartCustomBaseline;
pub use overlay_layer::{ChartOverlayLayer, ChartRootOverlayChrome};
pub use plot_area::{compute_drawing_area, DrawingArea};
pub use responsive::ResponsiveChartContainer;

/// Ready-made chart layout root — bind a [`Dataset`], tune props, or compose plot children.
///
/// Use [`ChartContainer`] when you need explicit control over axes, series, plot inset,
/// and child plot layers. For common chart types, prefer the `*Chart` convenience
/// components (e.g. [`crate::BarChart`]).
///
/// # Chart type family
///
/// Pick the chart that matches the story you are telling:
///
/// - **Compare categories** — [`crate::BarChart`] when magnitude at a shared baseline matters.
/// - **Show change over time** — [`crate::LineChart`] or [`crate::AreaChart`] for trends and stacked volume.
/// - **Show parts of a whole** — [`crate::PieChart`] when proportions beat precise comparisons.
/// - **Correlate two metrics** — [`crate::ScatterChart`] for point-by-point relationships.
/// - **Inline trend glyph** — [`crate::Sparkline`] beside KPIs without axis chrome.
/// - **Single-metric readout** — [`crate::Gauge`] for capacity or score at a glance.
/// - **Intensity grid** — [`crate::Heatmap`] for two categorical dimensions plus a value.
///
/// # Customization layers
///
/// 1. **Single `*Chart` component** — bind a [`Dataset`] and optional `legend` / `tooltip` props.
/// 2. **Configuration props** — axes, plot inset (`margin`), palette, highlight scope on the container.
/// 3. **Composition children** — [`ChartContainer`] + explicit `BarPlot` / `LinePlot` / custom SVG layers.
///    See `chart-composition` and `chart-stacking` previews.
///
/// Cross-cutting UX lives on dedicated previews: `charts-legend`, `charts-tooltip`,
/// `charts-highlighting`, `charts-label`, `charts-styling`, `charts-axis`.
///
/// # When to use
///
/// - Mixed-type dashboards (bar + line overlay) or custom SVG annotations.
/// - Full control over which plot children render and their z-order.
/// - Processed [`Dataset`] output from a DataTable without a convenience `*Chart` wrapper.
///
/// # Usage
///
/// 1. Bind a shared [`Dataset`] with `binding` or field keys that match your DataTable columns.
/// 2. Set explicit `width` and `height` for fixed tiles, or wrap with [`ResponsiveChartContainer`] for parent-fill layouts.
/// 3. Add plot children (`BarPlot`, `LinePlot`, etc.) in composition mode; otherwise the container renders axes only.
/// 4. Opt into `legend`, `tooltip`, and `highlight_scope` when the dashboard needs exploration — defaults are `None` for lightweight embeds.
/// 5. Wrap previews in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use the same field keys as [`orbital_data::FieldDef::key`] in your DataTable columns.
/// * Leave `skip_animation` unset so reduced-motion preferences are honored automatically.
/// * Set plot inset via `margin` when legends or long axis labels need extra room.
/// * Link users to cross-cutting previews instead of repeating legend/tooltip setup on every chart type.
/// * Follow [`ChartCompositionOrder`] for child z-order: grid → [`PlotClip`] → plots → axes → overlay chrome.
/// * Set `embed_mode=ChartEmbedMode::ScrollHost` when the chart lives inside a [`ScrollArea`](orbital_core_components::ScrollArea) so tooltips escape clip.
///
/// ## Don'ts
///
/// * Do not reach for composition when a `*Chart` wrapper already covers your chart type.
/// * Do not put `data-testid` on the chart component — wrap it in a native `div` or `span`.
/// * Do not rely on absolute tooltip positioning in scroll hosts — use `ChartEmbedMode` instead.
///
/// # Examples
///
/// ## Basic container with axes and grid
/// Minimal shell proving band x-axis, linear y-axis, and grid wiring. Start here before
/// adding plot children or binding a live [`Dataset`] from a DataTable.
/// <!-- preview -->
/// ```rust
/// use crate::ChartContainer;
/// use crate::preview::fixtures::{full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// view! {
///     <div data-testid="chart-container-preview" style="min-width: 560px; min-height: 320px;">
///         <ChartContainer
///             series=Some(vec![revenue_series()])
///             x_axis=Some(vec![quarter_x_axis()])
///             y_axis=Some(vec![revenue_y_axis()])
///             grid=Some(full_grid())
///             width=Some(520.0)
///             height=Some(320.0)
///         />
///     </div>
/// }
/// ```
///
/// ## Loading overlay
/// Sets `loading=true` while async data fetches run. The container shows [`Spinner`] and
/// [`Skeleton`] chrome instead of an empty plot — pair with your data resource's pending state.
/// <!-- preview -->
/// ```rust
/// use crate::ChartContainer;
/// view! {
///     <div data-testid="chart-container-loading-preview">
///         <ChartContainer loading=Some(true) width=Some(400.0) height=Some(280.0) />
///     </div>
/// }
/// ```
///
/// ## Custom layer via hooks
/// Dashed baseline drawn in SVG space using [`crate::use_drawing_area`] and
/// [`crate::use_y_scale`]. Use this pattern for annotations that must stay aligned on resize.
/// <!-- preview -->
/// ```rust
/// use crate::{ChartContainer, ChartCustomBaseline};
/// use crate::preview::fixtures::{quarter_x_axis, revenue_series, revenue_y_axis};
/// view! {
///     <div data-testid="chart-container-custom-layer-preview">
///         <ChartContainer
///             series=Some(vec![revenue_series()])
///             x_axis=Some(vec![quarter_x_axis()])
///             y_axis=Some(vec![revenue_y_axis()])
///             width=Some(520.0)
///             height=Some(320.0)
///         >
///             <ChartCustomBaseline />
///         </ChartContainer>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "chart-container",
    preview_label = "Chart Container",
    preview_icon = icondata::AiBorderOuterOutlined,
)]
#[component]
pub fn ChartContainer(
    /// Series definitions (explicit or derived from dataset + binding).
    #[prop(default = None)]
    series: Option<Vec<SeriesDef>>,
    /// Tabular data — primary consumption mode.
    #[prop(default = None)]
    dataset: Option<Dataset>,
    /// Shorthand field binding when using dataset.
    #[prop(default = None)]
    binding: Option<ChartFieldBinding>,
    /// X-axis definitions.
    #[prop(default = None)]
    x_axis: Option<Vec<AxisDef>>,
    /// Y-axis definitions.
    #[prop(default = None)]
    y_axis: Option<Vec<AxisDef>>,
    /// Chart width in pixels.
    #[prop(default = None)]
    width: Option<f64>,
    /// Chart height in pixels.
    #[prop(default = None)]
    height: Option<f64>,
    /// Space between SVG border and plot. User docs: "plot inset".
    #[prop(default = None)]
    margin: Option<PlotInset>,
    /// Background grid configuration.
    #[prop(default = None)]
    grid: Option<GridConfig>,
    /// Whether the chart is in a loading state.
    #[prop(default = None)]
    loading: Option<bool>,
    /// When true, skip all enter/update animations.
    #[prop(default = None)]
    skip_animation: Option<bool>,
    /// Chart-level animation configuration.
    #[prop(default = None)]
    motion: Option<ChartMotion>,
    /// Chart orientation for cartesian plots.
    #[prop(default = ChartOrientation::Vertical)]
    orientation: ChartOrientation,
    /// Chart geometry family.
    #[prop(default = ChartKind::Cartesian)]
    chart_kind: ChartKind,
    /// Highlight and fade scope.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
    /// Axis crosshair / band highlight configuration.
    #[prop(default = None)]
    axis_highlight: Option<AxisHighlightConfig>,
    /// Legend configuration (`None` hides legend).
    #[prop(default = None)]
    legend: Option<LegendConfig>,
    /// Tooltip configuration.
    #[prop(default = None)]
    tooltip: Option<TooltipConfig>,
    /// Chart theme extension overrides.
    #[prop(default = None)]
    charts_theme: Option<OrbitalChartsTheme>,
    /// Color palette override.
    #[prop(default = None)]
    palette: Option<OrbitalChartPalette>,
    /// Controlled highlighted item.
    #[prop(default = None)]
    highlighted_item: Option<RwSignal<Option<ChartItemId>>>,
    /// Fired when highlight changes.
    #[prop(default = None)]
    on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
    /// Fired when a legend entry is clicked.
    #[prop(default = None)]
    on_legend_click: Option<Callback<(String,), ()>>,
    /// Fired when a bar/mark is clicked.
    #[prop(default = None)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Fired when a category axis band is clicked.
    #[prop(default = None)]
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    /// Opt-in capability flags (`ChartFeatures::ZOOM_PAN`, `ChartFeatures::ANIMATION`, `ChartFeatures::KEYBOARD_NAV`).
    #[prop(default = CHART_FEATURES_DEFAULT)]
    features: ChartFeatures,
    /// Enable arrow-key navigation between marks (CH-22). Keyboard zoom (CH-24) is deferred.
    #[prop(default = true)]
    keyboard_navigation: bool,
    /// When true, inferred line/area x-axes use [`crate::DomainLimit::Strict`].
    #[prop(default = false)]
    prefer_line_x_strict: bool,
    /// Controlled zoom windows per axis (percent 0–100).
    #[prop(default = None)]
    zoom: Option<Vec<ZoomWindow>>,
    /// Fired when the user or program changes zoom state.
    #[prop(default = None)]
    on_zoom_change: Option<Callback<(Vec<ZoomWindow>,), ()>>,
    /// Custom loading overlay slot.
    #[prop(default = None)]
    loading_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// Custom empty overlay slot.
    #[prop(default = None)]
    empty_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// How the chart is embedded in its host (scroll, dialog, table cell).
    #[prop(default = ChartEmbedMode::Inline)]
    embed_mode: ChartEmbedMode,
    /// Portal mount override for overlay chrome.
    #[prop(default = OverlayMount::ChartLocal)]
    overlay_mount: OverlayMount,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Composition children (plot, axis, legend layers).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let w = width.unwrap_or(520.0);
    let h = height.unwrap_or(320.0);
    let theme_options = use_theme_options();
    let inset = margin.unwrap_or_else(|| default_plot_inset(theme_options.get_untracked().density));
    let is_loading = loading.unwrap_or(false);
    let root_class = class;

    // Reference fixtures so preview doc examples compile in crate context.
    #[cfg(feature = "preview")]
    let _ = (
        revenue_series,
        cost_series,
        quarter_x_axis,
        revenue_y_axis,
        full_grid,
        processed_dataset,
        processed_binding,
    );

    view! {
        <ChartRoot
            class=root_class
            dataset=dataset
            binding=binding
            width=w
            height=h
            margin=inset
            skip_animation=skip_animation
            motion=motion
            loading=is_loading
            series=series
            x_axis=x_axis
            y_axis=y_axis
            grid=grid
            palette=palette
            loading_view=loading_view
            empty_view=empty_view
            orientation=orientation
            chart_kind=chart_kind
            highlight_scope=highlight_scope
            axis_highlight=axis_highlight
            legend=legend
            tooltip=tooltip
            charts_theme=charts_theme
            highlighted_item=highlighted_item
            on_highlight_change=on_highlight_change
            on_item_click=on_item_click
            on_axis_click=on_axis_click
            on_legend_click=on_legend_click
            features=features
            keyboard_navigation=keyboard_navigation
            prefer_line_x_strict=prefer_line_x_strict
            zoom=zoom
            on_zoom_change=on_zoom_change
            embed_mode=embed_mode
            overlay_mount=overlay_mount
        >
            {children.map(|c| c())}
        </ChartRoot>
    }
}
