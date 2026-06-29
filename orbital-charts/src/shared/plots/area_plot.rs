//! Area plot layer — stacked/percent fill using the line engine.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::context::{is_series_visible, use_chart_context, use_x_scale, use_y_scale};
use crate::engine::{
    build_area_path, build_line_path, data_fingerprint, projected_for_plot_type,
    resolve_series_color, resolve_stack_config, stack_segment_bottom, stack_series, PlotPoint,
};
use crate::shared::marks::LineStroke;
use crate::shared::marks::{area_gradient_id, AreaFill};
use crate::{ChartScale, ChartType, CurveType, StackOffset, StackOrder};

/// Renders filled area paths for each series.
#[component]
pub fn AreaPlot(
    /// Stack offset strategy override for stacked/percent areas.
    #[prop(default = None)]
    stack_offset: Option<StackOffset>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let skip = ctx.skip_animation;
    let highlight_scope = ctx.highlight_scope;
    let projected = ctx.projected.clone();
    let series_defs = ctx.series.clone();
    let palette = ctx.palette.clone();
    let x_scale = use_x_scale("x");
    let y_scale = use_y_scale("y");

    view! {
        {move || {
            let Some(data) = projected.as_ref() else {
                return ().into_any();
            };

            let plot_data = projected_for_plot_type(data, &series_defs, ChartType::Area);
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
                let (resolved_offset, resolved_order) =
                    resolve_stack_config(&group, &series_defs, ChartType::Area);
                (
                    stack_offset.unwrap_or(resolved_offset),
                    resolved_order,
                )
            } else {
                (stack_offset.unwrap_or(StackOffset::None), StackOrder::None)
            };

            let visible_series: Vec<_> = plot_data
                .series
                .iter()
                .filter(|s| is_series_visible(&s.id))
                .cloned()
                .collect();

            let raw_series = visible_series.clone();
            let render_series: Vec<_> = if has_stack {
                stack_series(&visible_series, &stack_groups, offset, order)
            } else {
                visible_series
            };

            render_series
                .iter()
                .enumerate()
                .map(|(idx, s)| {
                    let def = series_defs
                        .iter()
                        .find(|d| d.id == s.id)
                        .cloned()
                        .unwrap_or_default();
                    let color = resolve_series_color(idx, &def, &palette);
                    let curve = def.curve.unwrap_or(CurveType::Linear);
                    let connect_nulls = def.connect_nulls.unwrap_or(false);
                    let use_gradient = def.area.unwrap_or(true);

                    let bottom_data: Vec<f64> = if has_stack {
                        (0..s.data.len())
                            .map(|row| {
                                stack_segment_bottom(
                                    &raw_series,
                                    &render_series,
                                    &stack_groups,
                                    &series_defs,
                                    &s.id,
                                    row,
                                    offset,
                                    order,
                                )
                            })
                            .collect()
                    } else {
                        vec![0.0; s.data.len()]
                    };

                    let top_points: Vec<PlotPoint> = plot_data
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

                    let bottom_points: Vec<PlotPoint> = plot_data
                        .categories
                        .iter()
                        .enumerate()
                        .map(|(i, cat)| {
                            let x = category_x(&x_scale, cat, i, plot_data.categories.len());
                            let y_val = bottom_data.get(i).copied();
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

                    let top_line = build_line_path(&top_points, curve, connect_nulls);
                    let baseline_y = scale_y(&y_scale, 0.0);
                    let mut area_d = build_area_path(&top_line, baseline_y);

                    if has_stack && !bottom_points.is_empty() {
                        let bottom_line = build_line_path(&bottom_points, curve, connect_nulls);
                        if !bottom_line.d.is_empty() && !top_line.d.is_empty() {
                            area_d = stacked_area_path(&top_line, &bottom_line);
                        }
                    }

                    let d: Signal<String> = Signal::from(area_d);
                    let grad_id = area_gradient_id(&s.id);
                    let series_color: Signal<String> = Signal::from(color.clone());
                    let stroke: Signal<String> = Signal::from(color);
                    let line_d: Signal<String> = Signal::from(top_line.d.clone());
                    let key: Signal<String> =
                        Signal::from(format!("{}-{}", s.id, data_fingerprint(&s.data)));

                    let marker_triples: Vec<(f64, f64, usize)> = top_line
                        .markers
                        .iter()
                        .enumerate()
                        .map(|(i, (x, y))| (*x, *y, i))
                        .collect();
                    let scope = def.highlight_scope.or(highlight_scope);

                    view! {
                        <AreaFill
                            d=d
                            series_color=series_color
                            gradient_id=grad_id
                            use_gradient=use_gradient
                        />
                        <LineStroke
                            d=line_d
                            stroke=stroke
                            markers=marker_triples
                            series_id=s.id.clone()
                            show_markers=def.show_markers.unwrap_or(false)
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

fn scale_y(scale: &ChartScale, value: f64) -> f64 {
    match scale {
        ChartScale::Linear(l) => l.scale(value),
        _ => value,
    }
}

fn stacked_area_path(top: &crate::engine::LinePath, bottom: &crate::engine::LinePath) -> String {
    if top.markers.is_empty() {
        return String::new();
    }
    let mut d = top.d.clone();
    for (x, y) in bottom.markers.iter().rev() {
        d.push_str(&format!(" L {x} {y}"));
    }
    if let Some((x, y)) = top.markers.first() {
        d.push_str(&format!(" L {x} {y} Z"));
    }
    d
}
