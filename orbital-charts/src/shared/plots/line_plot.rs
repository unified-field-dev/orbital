//! Line plot layer.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::context::{
    is_series_visible, use_chart_context, use_hidden_series, use_x_scale, use_y_scale,
    ChartInteractionContext,
};
use crate::engine::{
    build_line_path, data_fingerprint, projected_for_plot_type, resolve_series_color,
    resolve_stack_config, stack_series, PlotPoint,
};
use crate::shared::marks::LineStroke;
use crate::{ChartScale, ChartType, CurveType, SeriesDef, StackOffset};

/// Skip per-point markers above this count (path still renders).
const LAZY_MARKER_THRESHOLD: usize = 500;

fn scale_y(scale: &ChartScale, value: f64) -> f64 {
    match scale {
        ChartScale::Linear(l) => l.scale(value),
        _ => value,
    }
}

fn collect_line_markers(
    projected: &crate::engine::ProjectedChartData,
    series_defs: &[SeriesDef],
    x_scale: &ChartScale,
    y_scale: &ChartScale,
) -> Vec<(f64, f64, String, usize)> {
    let plot_data = projected_for_plot_type(projected, series_defs, ChartType::Line);
    if plot_data.series.is_empty() {
        return Vec::new();
    }

    let stack_groups: HashMap<String, String> = series_defs
        .iter()
        .filter_map(|s| s.stack_group.as_ref().map(|g| (s.id.clone(), g.clone())))
        .collect();
    let has_stack = plot_data
        .series
        .iter()
        .any(|s| stack_groups.contains_key(&s.id));

    let (offset, order) = if has_stack {
        let group = stack_groups
            .values()
            .next()
            .cloned()
            .unwrap_or_else(|| "stack".into());
        resolve_stack_config(&group, series_defs, ChartType::Line)
    } else {
        (StackOffset::None, crate::StackOrder::None)
    };

    let render_series = if has_stack {
        stack_series(&plot_data.series, &stack_groups, offset, order)
    } else {
        plot_data.series.clone()
    };

    let total_points = render_series.iter().map(|s| s.data.len()).sum::<usize>();
    if total_points > LAZY_MARKER_THRESHOLD {
        return Vec::new();
    }

    let mut line_markers = Vec::new();
    for s in render_series.iter().filter(|s| is_series_visible(&s.id)) {
        for (i, cat) in plot_data.categories.iter().enumerate() {
            let x = category_x(x_scale, cat, i, plot_data.categories.len());
            let y_val = s.data.get(i).copied();
            if let Some(y) = y_val.filter(|v| !v.is_nan()).map(|v| scale_y(y_scale, v)) {
                line_markers.push((x, y, s.id.clone(), i));
            }
        }
    }
    line_markers
}

/// Renders line strokes and optional markers for each series.
#[component]
pub fn LinePlot() -> impl IntoView {
    let ctx = use_chart_context();
    let interaction = expect_context::<ChartInteractionContext>();
    let hidden = use_hidden_series();
    let skip = ctx.skip_animation;
    let highlight_scope = ctx.highlight_scope;
    let projected = ctx.projected.clone();
    let series_defs = ctx.series.clone();
    let palette = ctx.palette.clone();
    let x_scale = use_x_scale("x");
    let y_scale = use_y_scale("y");

    let projected_for_markers = projected.clone();
    let series_defs_for_markers = series_defs.clone();
    let x_scale_for_markers = x_scale.clone();
    let y_scale_for_markers = y_scale.clone();
    let line_markers = Memo::new(move |_| {
        let _ = hidden.get();
        let Some(data) = projected_for_markers.as_ref() else {
            return Vec::new();
        };
        collect_line_markers(
            data,
            &series_defs_for_markers,
            &x_scale_for_markers,
            &y_scale_for_markers,
        )
    });

    Effect::new(move |_| {
        interaction.plot_line_markers.set(line_markers.get());
    });

    view! {
        {move || {
            let Some(data) = projected.as_ref() else {
                return ().into_any();
            };

            let plot_data = projected_for_plot_type(data, &series_defs, ChartType::Line);
            if plot_data.series.is_empty() {
                return ().into_any();
            }

            let stack_groups: HashMap<String, String> = series_defs
                .iter()
                .filter_map(|s| s.stack_group.as_ref().map(|g| (s.id.clone(), g.clone())))
                .collect();
            let has_stack = plot_data
                .series
                .iter()
                .any(|s| stack_groups.contains_key(&s.id));

            let (offset, order) = if has_stack {
                let group = stack_groups
                    .values()
                    .next()
                    .cloned()
                    .unwrap_or_else(|| "stack".into());
                resolve_stack_config(&group, &series_defs, ChartType::Line)
            } else {
                (StackOffset::None, crate::StackOrder::None)
            };

            let render_series = if has_stack {
                stack_series(&plot_data.series, &stack_groups, offset, order)
            } else {
                plot_data.series.clone()
            };

            let total_points = render_series.iter().map(|s| s.data.len()).sum::<usize>();
            let lazy_markers = total_points > LAZY_MARKER_THRESHOLD;

            render_series
                .iter()
                .enumerate()
                .filter(|(_, s)| is_series_visible(&s.id))
                .map(|(idx, s)| {
                    let def = series_defs
                        .iter()
                        .find(|d| d.id == s.id)
                        .cloned()
                        .unwrap_or_default();
                    let color = resolve_series_color(idx, &def, &palette);
                    let curve = def.curve.unwrap_or(CurveType::Linear);
                    let connect_nulls = def.connect_nulls.unwrap_or(false);
                    let show_markers = def.show_markers.unwrap_or(false) && !lazy_markers;
                    let scope = def.highlight_scope.or(highlight_scope);

                    let points: Vec<PlotPoint> = plot_data
                        .categories
                        .iter()
                        .enumerate()
                        .map(|(i, cat)| {
                            let x = category_x(&x_scale, cat, i, plot_data.categories.len());
                            let y_val = s.data.get(i).copied();
                            PlotPoint {
                                x,
                                y: y_val.and_then(|v| {
                                    if v.is_nan() {
                                        None
                                    } else {
                                        Some(scale_y(&y_scale, v))
                                    }
                                }),
                            }
                        })
                        .collect();

                    let line = build_line_path(&points, curve, connect_nulls);
                    let marker_triples: Vec<(f64, f64, usize)> = line
                        .markers
                        .iter()
                        .enumerate()
                        .map(|(i, (x, y))| (*x, *y, i))
                        .collect();
                    let d: Signal<String> = Signal::from(line.d.clone());
                    let stroke: Signal<String> = Signal::from(color.clone());
                    let key: Signal<String> =
                        Signal::from(format!("{}-{}", s.id, data_fingerprint(&s.data)));

                    view! {
                        <LineStroke
                            d=d
                            stroke=stroke
                            markers=marker_triples
                            series_id=s.id.clone()
                            show_markers=show_markers
                            skip_animation=skip
                            draw_key=key
                            highlight_scope=scope
                        />
                    }
                })
                .collect_view()
                .into_any()
        }}
    }
}

fn category_x(scale: &ChartScale, category: &str, index: usize, count: usize) -> f64 {
    match scale {
        ChartScale::Band(b) => b.scale(category).unwrap_or_else(|| {
            if count > 1 {
                (b.step() * index as f64) + b.bandwidth() / 2.0
            } else {
                b.bandwidth() / 2.0
            }
        }),
        ChartScale::Linear(l) => l.scale(index as f64),
    }
}
