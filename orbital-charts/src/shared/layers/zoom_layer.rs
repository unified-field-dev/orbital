//! Zoom and pan gesture layer for cartesian charts.

use leptos::ev::{MouseEvent, WheelEvent};
use leptos::prelude::*;

use crate::context::ChartScale;
use crate::context::{
    use_chart_context, use_chart_zoom, use_x_scale, use_y_scale, ChartZoomContext, ZoomDragState,
};
use crate::engine::{band_index_to_fraction, pan_window, zoom_at_pointer};
use crate::ChartOrientation;

/// Transparent overlay capturing wheel, drag, and pinch zoom gestures.
#[component]
pub fn ChartZoomLayer() -> impl IntoView {
    let Some(zoom_ctx) = use_chart_zoom() else {
        return ().into_any();
    };

    if !zoom_ctx.is_active() {
        return ().into_any();
    }

    let ctx = use_chart_context();
    let plot_w = ctx.drawing_area.plot_width;
    let plot_h = ctx.drawing_area.plot_height;
    let orientation = ctx.orientation;

    let category_axis_id = match orientation {
        ChartOrientation::Vertical => "x".to_string(),
        ChartOrientation::Horizontal => "y".to_string(),
    };

    let category_scale = match orientation {
        ChartOrientation::Vertical => use_x_scale(category_axis_id.clone()),
        ChartOrientation::Horizontal => use_y_scale(category_axis_id.clone()),
    };

    let primary_config = zoom_ctx
        .config_for(&category_axis_id)
        .cloned()
        .unwrap_or_default();

    let panning_enabled = primary_config.panning_enabled();
    let zoom_ctx_style = zoom_ctx.clone();
    let primary_config_style = primary_config.clone();
    let cursor_style = move || {
        if zoom_ctx_style.dragging.get().is_some() {
            "grabbing"
        } else if primary_config_style.panning_enabled() {
            "grab"
        } else {
            "crosshair"
        }
    };

    let zoom_ctx_wheel = zoom_ctx.clone();
    let zoom_ctx_down = zoom_ctx.clone();
    let zoom_ctx_move = zoom_ctx.clone();
    let zoom_ctx_up = zoom_ctx.clone();
    let category_axis_id_wheel = category_axis_id.clone();
    let category_axis_id_down = category_axis_id.clone();
    let ctx_for_wheel = ctx.clone();

    view! {
        <rect
            class="orb-zoom-layer"
            x=0.0
            y=0.0
            width=plot_w
            height=plot_h
            fill="transparent"
            style=move || format!("pointer-events: all; cursor: {};", cursor_style())
            on:wheel=move |ev: WheelEvent| {
                ev.prevent_default();
                handle_wheel(
                    &zoom_ctx_wheel,
                    &category_axis_id_wheel,
                    &category_scale,
                    &ctx_for_wheel,
                    ev.offset_x() as f64,
                    ev.delta_y(),
                );
            }
            on:mousedown=move |ev: MouseEvent| {
                if ev.button() != 0 || !panning_enabled {
                    return;
                }
                let axis_id = category_axis_id_down.clone();
                let windows = zoom_ctx_down.windows.get_untracked();
                if let Some(window) = windows.iter().find(|w| w.axis_id == axis_id) {
                    zoom_ctx_down.dragging.set(Some(ZoomDragState {
                        axis_id: axis_id.clone(),
                        start_x: ev.offset_x() as f64,
                        start_window: window.clone(),
                    }));
                }
            }
            on:mousemove=move |ev: MouseEvent| {
                let Some(drag) = zoom_ctx_move.dragging.get_untracked() else {
                    return;
                };
                if !panning_enabled {
                    return;
                }
                let dx = ev.offset_x() as f64 - drag.start_x;
                let delta_pct = -(dx / plot_w) * (drag.start_window.end - drag.start_window.start);
                if let Some(config) = zoom_ctx_move.config_for(&drag.axis_id) {
                    let next = pan_window(&drag.start_window, delta_pct, config);
                    zoom_ctx_move.update_axis_window(&drag.axis_id, next);
                }
            }
            on:mouseup=move |_| {
                zoom_ctx_up.dragging.set(None);
            }
            on:mouseleave=move |_| {
                zoom_ctx_up.dragging.set(None);
            }
            on:touchend=move |_| {
                zoom_ctx.dragging.set(None);
            }
        />
    }
    .into_any()
}

fn handle_wheel(
    zoom_ctx: &ChartZoomContext,
    axis_id: &str,
    category_scale: &ChartScale,
    ctx: &crate::context::ChartContext,
    offset_x: f64,
    delta_y: f64,
) {
    let Some(config) = zoom_ctx.config_for(axis_id) else {
        return;
    };
    let windows = zoom_ctx.windows.get_untracked();
    let Some(window) = windows.iter().find(|w| w.axis_id == axis_id) else {
        return;
    };

    let pointer_fraction = pointer_fraction_in_full_domain(category_scale, offset_x, axis_id, ctx);

    let zoom_in = delta_y < 0.0;
    let next = zoom_at_pointer(window, pointer_fraction, zoom_in, config);
    zoom_ctx.update_axis_window(axis_id, next);
}

fn pointer_fraction_in_full_domain(
    scale: &ChartScale,
    offset_x: f64,
    axis_id: &str,
    ctx: &crate::context::ChartContext,
) -> f64 {
    match scale {
        ChartScale::Band(band) => {
            let local_idx = band.index_at(offset_x).unwrap_or(0);
            let full_count = ctx
                .zoom_full_category_counts
                .get(axis_id)
                .copied()
                .unwrap_or(band.domain().len());
            if full_count == 0 {
                return 0.5;
            }
            let window = ctx
                .zoom_windows
                .iter()
                .find(|w| w.axis_id == axis_id)
                .cloned()
                .unwrap_or_else(|| crate::ZoomWindow::full(axis_id));
            let (start_idx, _) = crate::engine::zoom_window_to_band_indices(full_count, &window);
            let full_idx = start_idx + local_idx;
            band_index_to_fraction(full_idx, full_count)
        }
        ChartScale::Linear(linear) => {
            let domain_val = linear.invert(offset_x);
            let full_domain = ctx.y_domain;
            let span = full_domain.1 - full_domain.0;
            if span == 0.0 {
                0.5
            } else {
                ((domain_val - full_domain.0) / span).clamp(0.0, 1.0)
            }
        }
    }
}
