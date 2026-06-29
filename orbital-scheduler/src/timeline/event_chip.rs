//! Horizontal timeline event bar rendering (SC-17, SC-20).

use leptos::prelude::*;
use orbital_base_components::ToUnixSeconds;

use crate::shared::interaction::{
    event_is_draggable, event_is_editable, event_is_resizable, scheduler_drag_active,
    scheduler_drag_session, use_scheduler_interaction,
};
use crate::timeline::drag::{begin_timeline_event_drag, timeline_drag_mode_from_pointer};
use crate::timeline::engine::TimelineEventLayoutRect;
use crate::{
    render_event_content, EventDialogMode, EventDialogRequest, EventRenderSurface, PlannedEvent,
};

/// Positioned event bar inside a timeline resource lane.
#[component]
pub fn TimelineEventChip(
    event: PlannedEvent,
    layout: TimelineEventLayoutRect,
    resource_id: Option<String>,
) -> impl IntoView {
    let ctx = use_scheduler_interaction();
    let test_id = format!("scheduler-event-{}", event.id);
    let start_unix = event.start.to_unix_seconds().to_string();
    let end_unix = event.end.to_unix_seconds().to_string();
    let event_id = event.id.clone();
    let event_color = event.color.clone();
    let event_id_for_drag = event_id.clone();
    let is_draggable_override = event.is_draggable;
    let is_resizable_override = event.is_resizable;
    let resource_id_for_render = resource_id.clone();

    let class_list = {
        let ctx = ctx.clone();
        let event_id = event_id_for_drag.clone();
        let event_for_class = event.clone();
        move || {
            let ctx = ctx.clone();
            let mut parts = vec!["orb-scheduler-event".to_string()];
            let can_drag =
                event_is_draggable(&event_for_class, ctx.are_events_draggable.get_untracked())
                    || event_is_resizable(
                        &event_for_class,
                        ctx.are_events_resizable.get_untracked(),
                    );
            if can_drag {
                parts.push("orb-scheduler-event--draggable".to_string());
            }
            let dragging = scheduler_drag_active(&ctx)
                && scheduler_drag_session(&ctx)
                    .map(|s| s.event_id == event_id)
                    .unwrap_or(false);
            if dragging {
                parts.push("orb-scheduler-event--dragging".to_string());
            }
            parts.join(" ")
        }
    };

    let style = move || {
        let mut parts = vec![
            format!("left: {}%", layout.left_pct),
            format!("width: {}%", layout.width_pct),
        ];
        if let Some(color) = &event_color {
            parts.push(format!(
                "background: color-mix(in srgb, {color} 18%, var(--orb-color-surface-canvas, #fff)); border-inline-start: 3px solid {color};"
            ));
        }
        parts.join("; ")
    };

    let on_pointer_down = {
        let event_for_drag = event.clone();
        let event_for_edit = event.clone();
        let resource_id = resource_id.clone();
        let ctx = ctx.clone();
        move |ev: leptos::ev::PointerEvent| {
            ev.stop_propagation();
            #[cfg(feature = "hydrate")]
            let (chip_height, chip_width, offset_x) = {
                use wasm_bindgen::JsCast;
                if let Some(el) = ev.current_target() {
                    if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                        let rect = html.get_bounding_client_rect();
                        let offset_x = ev.client_x() as f64 - rect.left();
                        (rect.height(), rect.width(), offset_x)
                    } else {
                        (24.0, 80.0, 0.0)
                    }
                } else {
                    (24.0, 80.0, 0.0)
                }
            };
            #[cfg(not(feature = "hydrate"))]
            let (chip_height, chip_width, offset_x) = (24.0_f64, 80.0_f64, 0.0_f64);

            let draggable =
                event_is_draggable(&event_for_drag, ctx.are_events_draggable.get_untracked())
                    && is_draggable_override.unwrap_or(true);
            let resizable =
                event_is_resizable(&event_for_drag, ctx.are_events_resizable.get_untracked())
                    && is_resizable_override.unwrap_or(true);

            if let Some(mode) =
                timeline_drag_mode_from_pointer(offset_x, chip_width, resizable, draggable)
            {
                begin_timeline_event_drag(
                    ctx.clone(),
                    ctx.drag_listeners,
                    event_for_drag.clone(),
                    resource_id.clone(),
                    mode,
                    ev.client_x() as f32,
                    ev.client_y() as f32,
                    chip_width as f32,
                    chip_height as f32,
                );
            } else if event_is_editable(&event_for_edit) {
                ctx.open_dialog.run(EventDialogRequest {
                    mode: EventDialogMode::Edit {
                        event_id: event_for_edit.id.clone(),
                    },
                });
            }
        }
    };

    view! {
        <div
            class=class_list
            data-testid=test_id
            data-event-id=event_id
            data-start-unix=start_unix
            data-end-unix=end_unix
            style=style
            on:pointerdown=on_pointer_down
        >
            {move || {
                let ctx = ctx.clone();
                ctx.renderers.with_value(|renderers| {
                    render_event_content(
                        renderers,
                        &event,
                        resource_id_for_render.as_deref(),
                        EventRenderSurface::TimelineBar,
                    )
                })
            }}
        </div>
    }
}
