//! Chart context provider and consumption hooks.

use std::collections::HashMap;

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_motion::use_reduced_motion;
use orbital_theme::use_theme_options;

use crate::engine::{
    axis_categories, enabled_zoom_axes, filter_projected_by_band_window, nice_domain,
    project_chart_data, project_pie_data, project_pie_inline, project_scatter_data,
    project_scatter_series, resolve_linear_axis_domain, resolve_plot_inset,
    resolve_scatter_domains, resolve_y_domain, slice_categories, window_for_axis, y_axis_ticks,
    zoom_window_to_linear_domain, BandScale, LinearScale, ProjectedChartData, ProjectedSeries,
};
use crate::types::effective_skip_animation;
use crate::{compute_drawing_area, DrawingArea};
use crate::{
    AxisClickData, AxisDef, AxisPosition, ChartFeatures, ChartItemId, ChartMotion,
    ChartOrientation, ChartType, DomainLimit, GridConfig, HighlightScope, OrbitalChartPalette,
    OrbitalChartsTheme, PlotInset, ProjectedPieData, ProjectedScatterData, ScaleType, SeriesDef,
    ZoomConfig, ZoomFilterMode, ZoomWindow,
};

/// Chart geometry family driving projection and scale construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartKind {
    #[default]
    Cartesian,
    Pie,
    Scatter,
    Sparkline,
    Heatmap,
}

/// Scale handle returned by [`use_x_scale`] and [`use_y_scale`].
#[derive(Clone, Debug, PartialEq)]
pub enum ChartScale {
    /// Categorical band scale.
    Band(BandScale),
    /// Continuous linear scale.
    Linear(LinearScale),
}

impl ChartScale {
    /// Map a numeric domain value through a linear scale.
    pub fn scale_f64(&self, value: f64) -> f64 {
        match self {
            Self::Band(scale) => scale.scale(&value.to_string()).unwrap_or(0.0),
            Self::Linear(scale) => scale.scale(value),
        }
    }

    /// Map a category to its band center (band scales only).
    pub fn scale_category(&self, category: &str) -> Option<f64> {
        match self {
            Self::Band(scale) => scale.scale(category),
            Self::Linear(_) => None,
        }
    }

    /// Invert a pixel coordinate to a domain value (linear scales only).
    pub fn invert(&self, pixel: f64) -> Option<f64> {
        match self {
            Self::Linear(scale) => Some(scale.invert(pixel)),
            Self::Band(_) => None,
        }
    }

    /// Band width when this is a band scale.
    pub fn bandwidth(&self) -> Option<f64> {
        match self {
            Self::Band(scale) => Some(scale.bandwidth()),
            Self::Linear(_) => None,
        }
    }
}

/// Built scales keyed by axis id.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartScales {
    /// X-axis scales by axis id.
    pub x: HashMap<String, ChartScale>,
    /// Y-axis scales by axis id.
    pub y: HashMap<String, ChartScale>,
}

/// Precomputed tick values keyed by axis id (separate x/y maps).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartAxisTicks {
    /// X-axis tick values per axis id.
    pub x: HashMap<String, Vec<f64>>,
    /// Y-axis tick values per axis id.
    pub y: HashMap<String, Vec<f64>>,
}

/// Full chart context provided to composition children.
#[derive(Clone, Debug)]
pub struct ChartContext {
    /// Series definitions passed to the container.
    pub series: Vec<SeriesDef>,
    /// Resolved x-axis definitions.
    pub x_axes: Vec<AxisDef>,
    /// Resolved y-axis definitions.
    pub y_axes: Vec<AxisDef>,
    /// Projected tabular/inline cartesian data.
    pub projected: Option<ProjectedChartData>,
    /// Projected pie slice data.
    pub pie: Option<ProjectedPieData>,
    /// Projected scatter point data.
    pub scatter: Option<ProjectedScatterData>,
    /// Chart geometry family.
    pub chart_kind: ChartKind,
    /// Highlight and fade scope for interaction.
    pub highlight_scope: Option<HighlightScope>,
    /// Plot bounds after inset.
    pub drawing_area: DrawingArea,
    /// Background grid configuration.
    pub grid: Option<GridConfig>,
    /// Resolved palette.
    pub palette: OrbitalChartPalette,
    /// Animation configuration.
    pub motion: ChartMotion,
    /// Whether animations are skipped.
    pub skip_animation: bool,
    /// Chart width in pixels.
    pub width: f64,
    /// Chart height in pixels.
    pub height: f64,
    /// Plot inset applied to the chart.
    pub margin: PlotInset,
    /// Y-axis value domain (raw min/max before nice padding).
    pub y_domain: (f64, f64),
    /// Whether the chart has no displayable data.
    pub is_empty: bool,
    /// Whether the chart is loading.
    pub loading: bool,
    /// Projection error message, if any.
    pub projection_error: Option<String>,
    /// Bar/line layout orientation.
    pub orientation: ChartOrientation,
    /// Item click callback.
    pub on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Axis click callback.
    pub on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    /// Active capability flags.
    pub features: ChartFeatures,
    /// Whether keyboard navigation between marks is enabled.
    pub keyboard_navigation: bool,
    /// Chart theme extension (tooltip delay, fade opacity, etc.).
    pub charts_theme: OrbitalChartsTheme,
    /// Current zoom windows per axis.
    pub zoom_windows: Vec<ZoomWindow>,
    /// Full category counts before zoom slicing (for pointer math).
    pub zoom_full_category_counts: HashMap<String, usize>,
}

/// Resolved chart state including built scales.
#[derive(Clone, Debug)]
pub struct ChartState {
    /// Shared chart context.
    pub context: ChartContext,
    /// Built axis scales.
    pub scales: ChartScales,
    /// Y-axis tick values per axis id.
    pub y_ticks: HashMap<String, Vec<f64>>,
    /// X-axis tick values per axis id.
    pub x_ticks: HashMap<String, Vec<f64>>,
    /// Per-axis value domains for scatter charts.
    pub axis_domains: HashMap<String, (f64, f64)>,
}

/// Build chart state from container props.
pub fn build_chart_state(
    dataset: Option<&Dataset>,
    binding: Option<&ChartFieldBinding>,
    series: Option<&[SeriesDef]>,
    x_axis: Option<&[AxisDef]>,
    y_axis: Option<&[AxisDef]>,
    width: f64,
    height: f64,
    margin: PlotInset,
    grid: Option<GridConfig>,
    palette: Option<&OrbitalChartPalette>,
    skip_animation: Option<bool>,
    motion: Option<&ChartMotion>,
    prefers_reduced_motion: bool,
    loading: bool,
    orientation: ChartOrientation,
    chart_kind: ChartKind,
    highlight_scope: Option<HighlightScope>,
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    features: ChartFeatures,
    keyboard_navigation: bool,
    charts_theme: OrbitalChartsTheme,
    prefer_line_x_strict: bool,
    zoom_windows: &[ZoomWindow],
) -> ChartState {
    let skip = effective_skip_animation(skip_animation, motion, prefers_reduced_motion);
    let motion = motion.cloned().unwrap_or_default();
    let palette = match palette {
        Some(p) if !p.colors.is_empty() => p.clone(),
        _ => OrbitalChartPalette::from_theme(None),
    };

    let (mut projected, pie, scatter, projection_error, is_empty) =
        resolve_all_projected(chart_kind, dataset, binding, series, x_axis, y_axis);

    let resolved_series =
        resolve_series_defs(series, projected.as_ref(), pie.as_ref(), scatter.as_ref());

    let (x_axes, y_axes) = resolve_axes(
        chart_kind,
        x_axis,
        y_axis,
        &resolved_series,
        prefer_line_x_strict,
        projected.as_ref(),
        scatter.as_ref(),
        orientation,
    );

    let mut x_axes = x_axes;
    let mut y_axes = y_axes;

    let full_y_domain = projected
        .as_ref()
        .map(|p| resolve_y_domain(p, &resolved_series))
        .unwrap_or((0.0, 1.0));

    let mut y_domain = full_y_domain;

    let (x_domains, y_domains) = scatter
        .as_ref()
        .map(|s| resolve_scatter_domains(s, &x_axes, &y_axes))
        .unwrap_or_default();

    let mut axis_domains = HashMap::new();
    axis_domains.extend(x_domains.clone());
    axis_domains.extend(y_domains.clone());

    if axis_domains.is_empty() {
        for axis in &y_axes {
            if matches!(
                axis.scale_type,
                ScaleType::Linear | ScaleType::Log | ScaleType::Sqrt
            ) {
                axis_domains.insert(axis.id.clone(), resolve_linear_axis_domain(axis, y_domain));
            }
        }
        for axis in &x_axes {
            if axis.scale_type == ScaleType::Linear {
                axis_domains.insert(axis.id.clone(), resolve_linear_axis_domain(axis, y_domain));
            }
        }
    }

    let (zoom_full_category_counts, active_zoom_windows) = apply_cartesian_zoom(
        &mut x_axes,
        &mut y_axes,
        &mut projected,
        &mut axis_domains,
        &mut y_domain,
        full_y_domain,
        &resolved_series,
        zoom_windows,
        features,
        chart_kind,
    );

    let y_domain = match chart_kind {
        ChartKind::Scatter => scatter
            .as_ref()
            .and_then(|s| s.series.first())
            .map(|_| (0.0, 1.0))
            .unwrap_or((0.0, 1.0)),
        _ => y_domain,
    };

    let mut y_ticks = HashMap::new();
    for axis in &y_axes {
        if axis.scale_type == ScaleType::Linear {
            let domain = y_domains.get(&axis.id).copied().unwrap_or(y_domain);
            y_ticks.insert(axis.id.clone(), y_axis_ticks(axis, domain));
        }
    }

    let mut x_ticks = HashMap::new();
    for axis in &x_axes {
        if axis.scale_type == ScaleType::Linear {
            let domain = x_domains.get(&axis.id).copied().unwrap_or(y_domain);
            x_ticks.insert(axis.id.clone(), y_axis_ticks(axis, domain));
        }
    }

    let margin = resolve_plot_inset(
        margin,
        &x_axes,
        &y_axes,
        &x_ticks,
        &y_ticks,
        orientation,
        chart_kind,
    );
    let drawing_area = compute_drawing_area(width, height, margin);

    let scales = build_scales(
        chart_kind,
        &x_axes,
        &y_axes,
        &drawing_area,
        projected.as_ref(),
        scatter.as_ref(),
        y_domain,
        &axis_domains,
    );

    let context = ChartContext {
        series: resolved_series,
        x_axes,
        y_axes,
        projected,
        pie,
        scatter,
        chart_kind,
        highlight_scope,
        drawing_area,
        grid,
        palette,
        motion,
        skip_animation: skip,
        width,
        height,
        margin,
        y_domain,
        is_empty,
        loading,
        projection_error,
        orientation,
        on_item_click,
        on_axis_click,
        features,
        keyboard_navigation,
        charts_theme,
        zoom_windows: active_zoom_windows,
        zoom_full_category_counts,
    };

    ChartState {
        context,
        scales,
        y_ticks,
        x_ticks,
        axis_domains,
    }
}

fn resolve_all_projected(
    chart_kind: ChartKind,
    dataset: Option<&Dataset>,
    binding: Option<&ChartFieldBinding>,
    series: Option<&[SeriesDef]>,
    x_axis: Option<&[AxisDef]>,
    y_axis: Option<&[AxisDef]>,
) -> (
    Option<ProjectedChartData>,
    Option<ProjectedPieData>,
    Option<ProjectedScatterData>,
    Option<String>,
    bool,
) {
    match chart_kind {
        ChartKind::Pie => {
            if let (Some(ds), Some(b)) = (dataset, binding) {
                return match project_pie_data(ds, b) {
                    Ok(pie) => {
                        let empty = pie.slices.is_empty();
                        (None, Some(pie), None, None, empty)
                    }
                    Err(err) => (None, None, None, Some(err.to_string()), true),
                };
            }
            if let Some(series_list) = series {
                if let Some(first) = series_list.first() {
                    if let Some(data) = &first.data {
                        let categories = x_axis
                            .and_then(|axes| axes.first())
                            .and_then(|a| a.data.clone())
                            .unwrap_or_default();
                        if !categories.is_empty() {
                            return match project_pie_inline(&categories, data, &first.id) {
                                Ok(pie) => (None, Some(pie), None, None, false),
                                Err(err) => (None, None, None, Some(err.to_string()), true),
                            };
                        }
                    }
                }
            }
            let empty = dataset.is_none() && series.is_none();
            (None, None, None, None, empty)
        }
        ChartKind::Scatter => {
            if let (Some(ds), Some(b)) = (dataset, binding) {
                return match project_scatter_data(ds, b) {
                    Ok(scatter) => {
                        let empty = scatter.series.is_empty()
                            || scatter.series.iter().all(|s| s.points.is_empty());
                        (None, None, Some(scatter), None, empty)
                    }
                    Err(err) => (None, None, None, Some(err.to_string()), true),
                };
            }
            if let Some(series_list) = series {
                return match project_scatter_series(series_list) {
                    Ok(scatter) => {
                        let empty = scatter.series.iter().all(|s| s.points.is_empty());
                        (None, None, Some(scatter), None, empty)
                    }
                    Err(err) => (None, None, None, Some(err.to_string()), true),
                };
            }
            let empty = dataset.is_none() && series.is_none();
            (None, None, None, None, empty)
        }
        ChartKind::Cartesian | ChartKind::Sparkline => {
            let (projected, err, empty) =
                resolve_projected(dataset, binding, series, x_axis, y_axis);
            (projected, None, None, err, empty)
        }
        ChartKind::Heatmap => (None, None, None, None, false),
    }
}

fn band_axis_categories(x_axis: Option<&[AxisDef]>, y_axis: Option<&[AxisDef]>) -> Vec<String> {
    let from_axes = |axes: &[AxisDef]| {
        axes.iter()
            .find(|a| matches!(a.scale_type, ScaleType::Band | ScaleType::Point))
            .and_then(|a| a.data.clone())
    };
    x_axis
        .and_then(from_axes)
        .or_else(|| y_axis.and_then(from_axes))
        .unwrap_or_default()
}

fn resolve_projected(
    dataset: Option<&Dataset>,
    binding: Option<&ChartFieldBinding>,
    series: Option<&[SeriesDef]>,
    x_axis: Option<&[AxisDef]>,
    y_axis: Option<&[AxisDef]>,
) -> (Option<ProjectedChartData>, Option<String>, bool) {
    if let (Some(ds), Some(b)) = (dataset, binding) {
        return match project_chart_data(ds, b) {
            Ok(data) => {
                let empty = data.series.is_empty() || data.categories.is_empty();
                (Some(data), None, empty)
            }
            Err(err) => (None, Some(err.to_string()), true),
        };
    }

    if let Some(series_list) = series {
        let categories = band_axis_categories(x_axis, y_axis);
        if !categories.is_empty() && !series_list.is_empty() {
            let projected_series: Vec<ProjectedSeries> = series_list
                .iter()
                .filter_map(|s| {
                    let data = s.data.clone()?;
                    Some(ProjectedSeries {
                        id: s.id.clone(),
                        label: s.label.clone().unwrap_or_else(|| s.id.clone()),
                        data,
                    })
                })
                .collect();
            if !projected_series.is_empty() {
                let empty = false;
                return (
                    Some(ProjectedChartData {
                        categories,
                        series: projected_series,
                    }),
                    None,
                    empty,
                );
            }
        }
    }

    let empty = dataset.is_none() && binding.is_none() && series.is_none();
    (None, None, empty)
}

fn resolve_series_defs(
    series: Option<&[SeriesDef]>,
    projected: Option<&ProjectedChartData>,
    pie: Option<&ProjectedPieData>,
    scatter: Option<&ProjectedScatterData>,
) -> Vec<SeriesDef> {
    if let Some(list) = series {
        if !list.is_empty() {
            return list.to_vec();
        }
    }
    if let Some(p) = pie {
        return vec![SeriesDef {
            id: p.series_id.clone(),
            label: Some(p.series_id.clone()),
            chart_type: Some(ChartType::Pie),
            ..Default::default()
        }];
    }
    if let Some(s) = scatter {
        return s
            .series
            .iter()
            .map(|ser| SeriesDef {
                id: ser.series_id.clone(),
                label: Some(ser.label.clone()),
                chart_type: Some(ChartType::Scatter),
                color: ser.color.clone(),
                marker_size: Some(ser.marker_size),
                x_axis_id: Some(ser.x_axis_id.clone()),
                y_axis_id: Some(ser.y_axis_id.clone()),
                ..Default::default()
            })
            .collect();
    }
    projected
        .map(|p| {
            p.series
                .iter()
                .map(|s| SeriesDef {
                    id: s.id.clone(),
                    label: Some(s.label.clone()),
                    ..Default::default()
                })
                .collect()
        })
        .unwrap_or_default()
}

fn is_line_cartesian(series: &[SeriesDef], prefer_line_x_strict: bool) -> bool {
    if series.is_empty() {
        return prefer_line_x_strict;
    }
    series.iter().all(|s| match s.chart_type {
        None => prefer_line_x_strict,
        Some(ChartType::Line) | Some(ChartType::Area) => true,
        _ => false,
    })
}

fn resolve_axes(
    chart_kind: ChartKind,
    x_axis: Option<&[AxisDef]>,
    y_axis: Option<&[AxisDef]>,
    series: &[SeriesDef],
    prefer_line_x_strict: bool,
    projected: Option<&ProjectedChartData>,
    _scatter: Option<&ProjectedScatterData>,
    orientation: ChartOrientation,
) -> (Vec<AxisDef>, Vec<AxisDef>) {
    if chart_kind == ChartKind::Pie {
        return (Vec::new(), Vec::new());
    }

    if let (Some(x), Some(y)) = (x_axis.as_ref(), y_axis.as_ref()) {
        return (x.to_vec(), y.to_vec());
    }

    if chart_kind == ChartKind::Scatter {
        let (inferred_x, inferred_y) = (
            vec![AxisDef {
                id: "x".into(),
                scale_type: ScaleType::Linear,
                position: AxisPosition::Bottom,
                domain_limit: Some(DomainLimit::Nice),
                ..Default::default()
            }],
            vec![AxisDef {
                id: "y".into(),
                scale_type: ScaleType::Linear,
                position: AxisPosition::Left,
                domain_limit: Some(DomainLimit::Nice),
                ..Default::default()
            }],
        );
        return (
            x_axis.map(|a| a.to_vec()).unwrap_or(inferred_x),
            y_axis.map(|a| a.to_vec()).unwrap_or(inferred_y),
        );
    }

    let categories = projected.map(|p| p.categories.clone()).unwrap_or_default();
    let line_x_strict = is_line_cartesian(series, prefer_line_x_strict);
    let x_domain_limit = line_x_strict.then_some(DomainLimit::Strict);

    let (inferred_x, inferred_y) = match orientation {
        ChartOrientation::Vertical => (
            vec![AxisDef {
                id: "x".into(),
                scale_type: ScaleType::Band,
                data: Some(categories.clone()),
                position: AxisPosition::Bottom,
                domain_limit: x_domain_limit,
                ..Default::default()
            }],
            vec![AxisDef {
                id: "y".into(),
                scale_type: ScaleType::Linear,
                position: AxisPosition::Left,
                domain_limit: Some(DomainLimit::Nice),
                ..Default::default()
            }],
        ),
        ChartOrientation::Horizontal => (
            vec![AxisDef {
                id: "x".into(),
                scale_type: ScaleType::Linear,
                position: AxisPosition::Bottom,
                domain_limit: Some(if line_x_strict {
                    DomainLimit::Strict
                } else {
                    DomainLimit::Nice
                }),
                ..Default::default()
            }],
            vec![AxisDef {
                id: "y".into(),
                scale_type: ScaleType::Band,
                data: Some(categories),
                position: AxisPosition::Left,
                ..Default::default()
            }],
        ),
    };

    (
        x_axis.map(|a| a.to_vec()).unwrap_or(inferred_x),
        y_axis.map(|a| a.to_vec()).unwrap_or(inferred_y),
    )
}

fn build_scales(
    chart_kind: ChartKind,
    x_axes: &[AxisDef],
    y_axes: &[AxisDef],
    drawing_area: &DrawingArea,
    projected: Option<&ProjectedChartData>,
    scatter: Option<&ProjectedScatterData>,
    y_domain: (f64, f64),
    axis_domains: &HashMap<String, (f64, f64)>,
) -> ChartScales {
    let mut x = HashMap::new();
    for axis in x_axes {
        let scale = match axis.scale_type {
            ScaleType::Band | ScaleType::Point => {
                let categories = axis_categories(axis, projected);
                let padding = axis.category_gap_ratio.unwrap_or(0.1);
                ChartScale::Band(BandScale::new(
                    categories,
                    (0.0, drawing_area.plot_width),
                    padding,
                ))
            }
            ScaleType::Linear => {
                let domain = axis_domains.get(&axis.id).copied().unwrap_or_else(|| {
                    let limit = axis.domain_limit.unwrap_or(DomainLimit::Nice);
                    nice_domain(y_domain.0, y_domain.1, limit)
                });
                ChartScale::Linear(LinearScale::new(domain, (0.0, drawing_area.plot_width)))
            }
            _ => continue,
        };
        x.insert(axis.id.clone(), scale);
    }

    let mut y = HashMap::new();
    for axis in y_axes {
        let domain = axis_domains.get(&axis.id).copied().unwrap_or_else(|| {
            let limit = axis.domain_limit.unwrap_or(DomainLimit::Nice);
            nice_domain(y_domain.0, y_domain.1, limit)
        });
        let scale = match axis.scale_type {
            ScaleType::Linear | ScaleType::Log | ScaleType::Sqrt => ChartScale::Linear(
                LinearScale::new((domain.0, domain.1), (drawing_area.plot_height, 0.0)),
            ),
            ScaleType::Band | ScaleType::Point => {
                let categories = axis_categories(axis, projected);
                ChartScale::Band(BandScale::new(
                    categories,
                    (drawing_area.plot_height, 0.0),
                    axis.category_gap_ratio.unwrap_or(0.1),
                ))
            }
            _ => ChartScale::Linear(LinearScale::new(
                (domain.0, domain.1),
                (drawing_area.plot_height, 0.0),
            )),
        };
        y.insert(axis.id.clone(), scale);
    }

    let _ = (chart_kind, scatter);
    ChartScales { x, y }
}

/// Provide chart context and scales to child components.
#[component]
pub fn ChartContextProvider(state: ChartState, children: Children) -> impl IntoView {
    provide_context(state.context.clone());
    provide_context(state.scales.clone());
    provide_context(ChartAxisTicks {
        x: state.x_ticks.clone(),
        y: state.y_ticks.clone(),
    });
    provide_context(state.axis_domains.clone());
    children()
}

/// Access the full chart context.
pub fn use_chart_context() -> ChartContext {
    expect_context::<ChartContext>()
}

/// Access the plot drawing area bounds.
pub fn use_drawing_area() -> DrawingArea {
    use_chart_context().drawing_area
}

/// Access an x-axis scale by id (defaults to `"x"`).
pub fn use_x_scale(axis_id: impl Into<String>) -> ChartScale {
    let id = axis_id.into();
    let scales = expect_context::<ChartScales>();
    scales
        .x
        .get(&id)
        .or_else(|| scales.x.values().next())
        .cloned()
        .unwrap_or_else(|| ChartScale::Linear(LinearScale::new((0.0, 1.0), (0.0, 100.0))))
}

/// Access a y-axis scale by id (defaults to `"y"`).
pub fn use_y_scale(axis_id: impl Into<String>) -> ChartScale {
    let id = axis_id.into();
    let scales = expect_context::<ChartScales>();
    scales
        .y
        .get(&id)
        .or_else(|| scales.y.values().next())
        .cloned()
        .unwrap_or_else(|| ChartScale::Linear(LinearScale::new((0.0, 1.0), (100.0, 0.0))))
}

/// Y-axis tick values for an axis id.
pub fn use_y_ticks(axis_id: impl Into<String>) -> Vec<f64> {
    let id = axis_id.into();
    expect_context::<ChartAxisTicks>()
        .y
        .get(&id)
        .cloned()
        .unwrap_or_default()
}

/// X-axis tick values for an axis id.
pub fn use_x_ticks(axis_id: impl Into<String>) -> Vec<f64> {
    let id = axis_id.into();
    expect_context::<ChartAxisTicks>()
        .x
        .get(&id)
        .cloned()
        .unwrap_or_default()
}

/// Apply zoom windows to axes, projected data, and domains.
fn apply_cartesian_zoom(
    x_axes: &mut Vec<AxisDef>,
    y_axes: &mut Vec<AxisDef>,
    projected: &mut Option<ProjectedChartData>,
    axis_domains: &mut HashMap<String, (f64, f64)>,
    y_domain: &mut (f64, f64),
    full_y_domain: (f64, f64),
    series_defs: &[SeriesDef],
    zoom_windows: &[ZoomWindow],
    features: ChartFeatures,
    chart_kind: ChartKind,
) -> (HashMap<String, usize>, Vec<ZoomWindow>) {
    if !matches!(chart_kind, ChartKind::Cartesian | ChartKind::Sparkline) {
        return (HashMap::new(), zoom_windows.to_vec());
    }

    let enabled = enabled_zoom_axes(features, x_axes, y_axes);
    if enabled.is_empty() {
        return (HashMap::new(), zoom_windows.to_vec());
    }

    let mut full_counts = HashMap::new();
    let mut active_windows = zoom_windows.to_vec();

    let snapshots: Vec<(String, ScaleType, ZoomConfig, Option<Vec<String>>)> = enabled
        .into_iter()
        .map(|(axis, config)| (axis.id.clone(), axis.scale_type, config, axis.data.clone()))
        .collect();

    for (axis_id, scale_type, config, axis_data) in snapshots {
        let window = window_for_axis(zoom_windows, &axis_id);
        if window.start <= 0.0 && window.end >= 100.0 {
            continue;
        }

        active_windows.retain(|w| w.axis_id != axis_id);
        active_windows.push(window.clone());

        match scale_type {
            ScaleType::Band | ScaleType::Point => {
                let full_cats = axis_data.unwrap_or_else(|| {
                    let fallback = x_axes
                        .iter()
                        .find(|a| a.id == axis_id)
                        .or_else(|| y_axes.iter().find(|a| a.id == axis_id))
                        .cloned()
                        .unwrap_or_default();
                    axis_categories(&fallback, projected.as_ref())
                });
                full_counts.insert(axis_id.clone(), full_cats.len());

                if let Some(data) = projected.as_mut() {
                    *data = filter_projected_by_band_window(data, &window);
                    if config.filter_mode() == ZoomFilterMode::Discard {
                        *y_domain = resolve_y_domain(data, series_defs);
                    } else {
                        *y_domain = full_y_domain;
                    }
                }

                let sliced = slice_categories(&full_cats, &window);
                if let Some(axis_mut) = x_axes.iter_mut().find(|a| a.id == axis_id) {
                    axis_mut.data = Some(sliced.clone());
                }
                if let Some(axis_mut) = y_axes.iter_mut().find(|a| a.id == axis_id) {
                    axis_mut.data = Some(sliced);
                }
            }
            ScaleType::Linear => {
                if let Some(full) = axis_domains.get(&axis_id).copied() {
                    let zoomed = zoom_window_to_linear_domain(full, &window);
                    axis_domains.insert(axis_id.clone(), zoomed);
                }
            }
            _ => {}
        }
    }

    (full_counts, active_windows)
}

/// Build chart state reactively inside a component.
pub fn use_chart_state(
    dataset: Option<Dataset>,
    binding: Option<ChartFieldBinding>,
    series: Option<Vec<SeriesDef>>,
    x_axis: Option<Vec<AxisDef>>,
    y_axis: Option<Vec<AxisDef>>,
    width: f64,
    height: f64,
    margin: PlotInset,
    grid: Option<GridConfig>,
    palette: Option<OrbitalChartPalette>,
    skip_animation: Option<bool>,
    motion: Option<ChartMotion>,
    loading: bool,
    orientation: ChartOrientation,
    chart_kind: ChartKind,
    highlight_scope: Option<HighlightScope>,
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    features: ChartFeatures,
    keyboard_navigation: bool,
    charts_theme: OrbitalChartsTheme,
    prefer_line_x_strict: bool,
    zoom_windows: RwSignal<Vec<ZoomWindow>>,
) -> Signal<ChartState> {
    let prefers_reduced = use_reduced_motion();
    let theme_options = use_theme_options();
    Signal::derive(move || {
        let resolved_palette = palette
            .clone()
            .filter(|p| !p.colors.is_empty())
            .unwrap_or_else(|| OrbitalChartPalette::from_theme(theme_options.get().brand.as_ref()));
        let windows = zoom_windows.get();
        build_chart_state(
            dataset.as_ref(),
            binding.as_ref(),
            series.as_deref(),
            x_axis.as_deref(),
            y_axis.as_deref(),
            width,
            height,
            margin,
            grid,
            Some(&resolved_palette),
            skip_animation,
            motion.as_ref(),
            prefers_reduced.get(),
            loading,
            orientation,
            chart_kind,
            highlight_scope,
            on_item_click,
            on_axis_click,
            features,
            keyboard_navigation,
            charts_theme.clone(),
            prefer_line_x_strict,
            &windows,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::{ChartFieldBinding, DataRecord, DataSchema, DataValue, Dataset};
    use std::collections::HashMap;

    fn fixture_dataset() -> Dataset {
        let schema = DataSchema::from_text_fields([
            ("quarter".into(), "Quarter".into()),
            ("revenue".into(), "Revenue".into()),
        ]);
        let records = vec![DataRecord::new(
            "1",
            HashMap::from([
                ("quarter".into(), DataValue::Category("Q1".into())),
                ("revenue".into(), DataValue::Number(100.0)),
            ]),
        )];
        Dataset::from_records(schema, records)
    }

    #[test]
    fn build_chart_state_from_dataset() {
        let dataset = fixture_dataset();
        let binding = ChartFieldBinding::new("quarter", vec!["revenue".into()]);
        let state = build_chart_state(
            Some(&dataset),
            Some(&binding),
            None,
            None,
            None,
            400.0,
            300.0,
            PlotInset::uniform(40.0),
            None,
            None,
            None,
            None,
            false,
            false,
            ChartOrientation::Vertical,
            ChartKind::Cartesian,
            None,
            None,
            None,
            ChartFeatures::ANIMATION,
            true,
            OrbitalChartsTheme::default(),
            false,
            &[],
        );
        assert!(state.context.projected.is_some());
        assert!(state.scales.x.contains_key("x"));
        assert!(state.scales.y.contains_key("y"));
    }

    #[test]
    fn inferred_line_x_axis_uses_strict_domain() {
        let series = vec![SeriesDef {
            id: "revenue".into(),
            data: Some(vec![10.0, 20.0, 30.0]),
            chart_type: Some(ChartType::Line),
            ..Default::default()
        }];
        let state = build_chart_state(
            None,
            None,
            Some(&series),
            None,
            None,
            400.0,
            300.0,
            PlotInset::uniform(40.0),
            None,
            None,
            None,
            None,
            false,
            false,
            ChartOrientation::Vertical,
            ChartKind::Cartesian,
            None,
            None,
            None,
            ChartFeatures::ANIMATION,
            true,
            OrbitalChartsTheme::default(),
            true,
            &[],
        );
        assert_eq!(
            state.context.x_axes[0].domain_limit,
            Some(DomainLimit::Strict)
        );
    }

    #[test]
    fn explicit_x_axis_skips_strict_inference() {
        let series = vec![SeriesDef {
            id: "revenue".into(),
            data: Some(vec![10.0, 20.0, 30.0]),
            chart_type: Some(ChartType::Line),
            ..Default::default()
        }];
        let x_axis = vec![AxisDef {
            id: "x".into(),
            scale_type: ScaleType::Band,
            data: Some(vec!["A".into(), "B".into(), "C".into()]),
            ..Default::default()
        }];
        let state = build_chart_state(
            None,
            None,
            Some(&series),
            Some(&x_axis),
            None,
            400.0,
            300.0,
            PlotInset::uniform(40.0),
            None,
            None,
            None,
            None,
            false,
            false,
            ChartOrientation::Vertical,
            ChartKind::Cartesian,
            None,
            None,
            None,
            ChartFeatures::ANIMATION,
            true,
            OrbitalChartsTheme::default(),
            true,
            &[],
        );
        assert_eq!(state.context.x_axes[0].domain_limit, None);
    }

    #[test]
    fn bar_series_skips_strict_x_domain() {
        let series = vec![SeriesDef {
            id: "revenue".into(),
            data: Some(vec![10.0, 20.0, 30.0]),
            chart_type: Some(ChartType::Bar),
            ..Default::default()
        }];
        let state = build_chart_state(
            None,
            None,
            Some(&series),
            None,
            None,
            400.0,
            300.0,
            PlotInset::uniform(40.0),
            None,
            None,
            None,
            None,
            false,
            false,
            ChartOrientation::Vertical,
            ChartKind::Cartesian,
            None,
            None,
            None,
            ChartFeatures::ANIMATION,
            true,
            OrbitalChartsTheme::default(),
            false,
            &[],
        );
        assert_eq!(state.context.x_axes[0].domain_limit, None);
    }

    #[test]
    fn horizontal_inferred_line_x_axis_uses_strict_domain() {
        let series = vec![SeriesDef {
            id: "value".into(),
            data: Some(vec![2.0, 50.0, 92.0, 45.0]),
            chart_type: Some(ChartType::Line),
            ..Default::default()
        }];
        let y_axis = vec![AxisDef {
            id: "y".into(),
            scale_type: ScaleType::Band,
            data: Some(vec!["A".into(), "B".into(), "C".into(), "D".into()]),
            ..Default::default()
        }];
        let state = build_chart_state(
            None,
            None,
            Some(&series),
            None,
            Some(&y_axis),
            400.0,
            300.0,
            PlotInset::uniform(40.0),
            None,
            None,
            None,
            None,
            false,
            false,
            ChartOrientation::Horizontal,
            ChartKind::Cartesian,
            None,
            None,
            None,
            ChartFeatures::ANIMATION,
            true,
            OrbitalChartsTheme::default(),
            true,
            &[],
        );
        assert_eq!(
            state.context.x_axes[0].domain_limit,
            Some(DomainLimit::Strict)
        );
        assert!(!state.axis_domains.get("x").unwrap().0.is_nan());
    }

    #[test]
    fn chart_scale_invert_linear() {
        let scale = ChartScale::Linear(LinearScale::new((0.0, 100.0), (200.0, 0.0)));
        assert_eq!(scale.scale_f64(50.0), 100.0);
        assert_eq!(scale.invert(100.0), Some(50.0));
    }
}
