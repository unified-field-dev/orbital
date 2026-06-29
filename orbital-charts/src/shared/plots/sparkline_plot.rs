//! Sparkline plot layer — minimal line or bar marks.

use leptos::prelude::*;

use crate::context::{
    set_hovered_item, use_chart_context, use_highlighted_item, use_x_scale, use_y_scale,
    ChartInteractionContext, ChartTooltipContext,
};
use crate::engine::{
    build_area_path, build_line_path, resolve_series_color, BarGeometry, PlotPoint,
};
use crate::{
    ChartItemId, ChartScale, CurveType, FadeMode, HighlightMode, HighlightScope, SparklinePlotType,
    TooltipTrigger,
};

fn scale_y(scale: &ChartScale, value: f64) -> f64 {
    match scale {
        ChartScale::Linear(l) => l.scale(value),
        _ => value,
    }
}

/// Renders sparkline line or bar marks without axis chrome.
#[component]
pub fn SparklinePlot(
    /// Line or bar plot type.
    #[prop(default = SparklinePlotType::Line)]
    plot_type: SparklinePlotType,
    /// Fill area under line (line mode only).
    #[prop(default = false)]
    area: bool,
    /// Line curve interpolation.
    #[prop(default = CurveType::Linear)]
    curve: CurveType,
    /// Optional series color override.
    #[prop(default = None)]
    color: Option<String>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let interaction = expect_context::<ChartInteractionContext>();
    let projected = ctx.projected.clone();
    let series_defs = ctx.series.clone();
    let palette = ctx.palette.clone();
    let x_scale = use_x_scale("x");
    let y_scale = use_y_scale("y");
    let plot_height = ctx.drawing_area.plot_height;
    let baseline_y = scale_y(&y_scale, 0.0);
    let stroke_width = (plot_height / 28.0 * 1.5).clamp(1.5, 3.0);
    let highlight_scope = ctx.highlight_scope.unwrap_or(HighlightScope {
        highlight: HighlightMode::None,
        fade: FadeMode::Global,
    });
    let show_markers = move || {
        ctx.highlight_scope.is_some()
            || use_context::<ChartTooltipContext>()
                .map(|tooltip| tooltip.trigger.get())
                .is_some_and(|trigger| !matches!(trigger, TooltipTrigger::None))
    };

    let highlighted = use_highlighted_item();

    view! {
        {move || {
            let Some(data) = projected.as_ref() else {
                return ().into_any();
            };
            let markers_enabled = show_markers();
            let Some(series) = data.series.first() else {
                return ().into_any();
            };
            let series_id = series_defs
                .first()
                .map(|s| s.id.clone())
                .unwrap_or_else(|| "sparkline".into());
            let def = series_defs.first().cloned().unwrap_or_default();
            let stroke_color = color.clone().unwrap_or_else(|| resolve_series_color(0, &def, &palette));

            match plot_type {
                SparklinePlotType::Bar => {
                    let bar_width = x_scale.bandwidth().unwrap_or(4.0) * 0.85;
                    let mut bars = Vec::new();
                    let marks = data
                        .categories
                        .iter()
                        .enumerate()
                        .map(|(i, cat)| {
                            let x_center = category_x(&x_scale, cat, i, data.categories.len());
                            let value = series.data.get(i).copied().unwrap_or(0.0);
                            let y_top = scale_y(&y_scale, value);
                            let bar_h = (baseline_y - y_top).max(0.0);
                            let bar_x = x_center - bar_width / 2.0;
                            bars.push(BarGeometry {
                                x: bar_x,
                                y: y_top,
                                width: bar_width,
                                height: bar_h,
                                series_id: series_id.clone(),
                                data_index: i,
                                value,
                            });
                            let item_id = ChartItemId {
                                series_id: series_id.clone(),
                                data_index: i,
                            };
                            let item_id_for_class = item_id.clone();
                            let mark_class = move || {
                                let mut classes = vec!["orb-sparkline-bar".to_string()];
                                let active = highlighted.get();
                                let is_active = active.as_ref() == Some(&item_id_for_class);
                                if highlight_scope.highlight != HighlightMode::None && active.is_some() {
                                    if is_active {
                                        classes.push("orb-sparkline-bar-highlighted".into());
                                    } else if highlight_scope.fade == FadeMode::Global {
                                        classes.push("orb-sparkline-bar-faded".into());
                                    }
                                }
                                classes.join(" ")
                            };
                            view! {
                                    <rect
                                        class=mark_class
                                        x=bar_x
                                        y=y_top
                                        width=bar_width
                                        height=bar_h
                                        fill=stroke_color.clone()
                                        style="pointer-events: all;"
                                        on:mouseenter=move |_| {
                                            interaction.pointer_plot.set(Some((
                                                ctx.drawing_area.left + x_center,
                                                ctx.drawing_area.top + y_top + bar_h / 2.0,
                                            )));
                                            set_hovered_item(Some(item_id.clone()));
                                        }
                                        on:mouseleave=move |_| {
                                            interaction.pointer_plot.set(None);
                                            set_hovered_item(None);
                                        }
                                    />
                            }
                        })
                        .collect_view();
                    interaction.plot_bars.set(bars);
                    marks.into_any()
                }
                SparklinePlotType::Line => {
                    let points: Vec<PlotPoint> = data
                        .categories
                        .iter()
                        .enumerate()
                        .map(|(i, cat)| {
                            let x = category_x(&x_scale, cat, i, data.categories.len());
                            let y_val = series.data.get(i).copied();
                            PlotPoint {
                                x,
                                y: y_val.map(|v| scale_y(&y_scale, v)),
                            }
                        })
                        .collect();

                    let line = build_line_path(&points, curve, false);
                    let path_d = line.d.clone();

                    let area_path = if area {
                        Some(build_area_path(&line, baseline_y))
                    } else {
                        None
                    };

                    let mut markers = Vec::new();
                    for (i, pt) in points.iter().enumerate() {
                        if let Some(y) = pt.y {
                            markers.push((pt.x, y, series_id.clone(), i));
                        }
                    }
                    interaction.plot_line_markers.set(markers);

                    let marker_views = markers_enabled.then(|| {
                        points
                            .iter()
                            .enumerate()
                            .filter_map(|(i, pt)| {
                                let y = pt.y?;
                                let item_id = ChartItemId {
                                    series_id: series_id.clone(),
                                    data_index: i,
                                };
                                let item_id_for_class = item_id.clone();
                                let cx = pt.x;
                                let cy = y;
                                let point_class = move || {
                                    let mut classes = vec!["orb-sparkline-point".to_string()];
                                    let active = highlighted.get();
                                    let is_active = active.as_ref() == Some(&item_id_for_class);
                                    if highlight_scope.highlight != HighlightMode::None && active.is_some() {
                                        if is_active {
                                            classes.push("orb-sparkline-point-highlighted".into());
                                        } else if highlight_scope.fade == FadeMode::Global {
                                            classes.push("orb-sparkline-point-faded".into());
                                        }
                                    }
                                    classes.join(" ")
                                };
                                Some(view! {
                                    <circle
                                        class=point_class
                                        cx=cx
                                        cy=cy
                                        r=stroke_width * 1.5
                                        fill=stroke_color.clone()
                                        style="pointer-events: all;"
                                        on:mouseenter=move |_| {
                                            interaction.pointer_plot.set(Some((
                                                ctx.drawing_area.left + cx,
                                                ctx.drawing_area.top + cy,
                                            )));
                                            set_hovered_item(Some(item_id.clone()));
                                        }
                                        on:mouseleave=move |_| {
                                            interaction.pointer_plot.set(None);
                                            set_hovered_item(None);
                                        }
                                    />
                                })
                            })
                            .collect_view()
                    });

                    let line_item_id = ChartItemId {
                        series_id: series_id.clone(),
                        data_index: 0,
                    };
                    let line_class = move || {
                        let mut classes = vec!["orb-sparkline-line".to_string()];
                        let active = highlighted.get();
                        if highlight_scope.highlight != HighlightMode::None
                            && active.is_some()
                            && active.as_ref() != Some(&line_item_id)
                            && highlight_scope.fade == FadeMode::Global
                        {
                            classes.push("orb-sparkline-line-faded".into());
                        }
                        classes.join(" ")
                    };

                    let first_hover = points.iter().find_map(|pt| {
                        pt.y.map(|y| (pt.x, y))
                    });
                    let series_for_line_hover = series_id.clone();

                    view! {
                        {area_path.map(|area_d| {
                            view! {
                                <path
                                    class="orb-sparkline-area"
                                    d=area_d
                                    fill=stroke_color.clone()
                                    fill-opacity="0.2"
                                />
                            }
                        })}
                        {markers_enabled.then(|| {
                            first_hover.map(|(hx, hy)| {
                                view! {
                                    <path
                                        class="orb-sparkline-line-hit"
                                        d=path_d.clone()
                                        stroke="transparent"
                                        stroke-width=(stroke_width * 10.0).max(12.0)
                                        fill="none"
                                        style="pointer-events: all;"
                                        on:mouseenter=move |_| {
                                            interaction.pointer_plot.set(Some((
                                                ctx.drawing_area.left + hx,
                                                ctx.drawing_area.top + hy,
                                            )));
                                            set_hovered_item(Some(ChartItemId {
                                                series_id: series_for_line_hover.clone(),
                                                data_index: 0,
                                            }));
                                        }
                                        on:mouseleave=move |_| {
                                            interaction.pointer_plot.set(None);
                                            set_hovered_item(None);
                                        }
                                    />
                                }
                            })
                        })}
                        <path
                            class=line_class
                            d=path_d
                            stroke=stroke_color.clone()
                            stroke-width=stroke_width
                            fill="none"
                        />
                        {marker_views}
                    }
                    .into_any()
                }
            }
        }}
    }
}

fn category_x(scale: &ChartScale, category: &str, index: usize, count: usize) -> f64 {
    match scale {
        ChartScale::Band(b) => b.scale(category).unwrap_or_else(|| {
            if count > 1 {
                b.step() * index as f64 + b.bandwidth() / 2.0
            } else {
                b.bandwidth() / 2.0
            }
        }),
        ChartScale::Linear(l) => l.scale(index as f64),
    }
}
