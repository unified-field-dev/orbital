use leptos::{ev, leptos_dom::helpers::WindowListenerHandle, prelude::*};

use crate::core::{
    column_drag_ghost_at_pointer, move_column_drag_ghost, use_data_table_context, ColumnDragGhost,
};
use crate::types::DataTableTableState;

#[cfg(not(feature = "hydrate"))]
pub fn column_field_at_client_point(_x: f32, _y: f32) -> Option<String> {
    None
}

#[cfg(not(feature = "hydrate"))]
pub fn picker_field_at_client_point(_x: f32, _y: f32) -> Option<String> {
    None
}

#[cfg(feature = "hydrate")]
pub fn column_field_at_client_point(x: f32, y: f32) -> Option<String> {
    let document = window().document()?;
    let el = document.element_from_point(x, y)?;
    let mut current = Some(el);
    while let Some(node) = current {
        for attribute in ["data-field", "data-drag-field"] {
            if let Some(field) = node.get_attribute(attribute) {
                if !field.is_empty() {
                    return Some(field);
                }
            }
        }
        current = node.parent_element();
    }
    None
}

#[cfg(feature = "hydrate")]
pub fn picker_field_at_client_point(x: f32, y: f32) -> Option<String> {
    let document = window().document()?;
    let el = document.element_from_point(x, y)?;
    let mut current = Some(el);
    while let Some(node) = current {
        for attribute in ["data-column-picker-field", "data-drag-field"] {
            if let Some(field) = node.get_attribute(attribute) {
                if !field.is_empty() {
                    return Some(field);
                }
            }
        }
        current = node.parent_element();
    }
    None
}

/// Follower label rendered at the table root while a column is dragged.
#[component]
pub fn DataTableColumnDragGhost() -> impl IntoView {
    let ctx = use_data_table_context();

    view! {
        {move || {
            ctx.column_drag_ghost.get().map(|ghost| {
                let style = format!(
                    "left: {:.0}px; top: {:.0}px; width: {:.0}px;",
                    ghost.x, ghost.y, ghost.width_px
                );
                view! {
                    <div
                        class="orbital-data-table__column-drag-ghost"
                        style=style
                        data-testid="data-table-column-drag-ghost"
                    >
                        {ghost.label}
                    </div>
                }
            })
        }}
    }
}

/// Shared pointer-drag completion for column reorder handles.
#[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
pub fn attach_pointer_reorder_end(
    listeners: StoredValue<Vec<WindowListenerHandle>>,
    dragging: RwSignal<bool>,
    drag_field: RwSignal<Option<String>>,
    ghost: RwSignal<Option<ColumnDragGhost>>,
    on_reorder: impl Fn(String, String) + Clone + Send + Sync + 'static,
    resolve_target: fn(f32, f32) -> Option<String>,
) {
    listeners.update_value(|handles| {
        for handle in handles.drain(..) {
            handle.remove();
        }
    });

    let on_pointer_move = window_event_listener(ev::pointermove, move |ev: ev::PointerEvent| {
        if drag_field.get().is_none() {
            return;
        }
        ghost.update(|current| {
            if let Some(active) = current.as_mut() {
                move_column_drag_ghost(active, ev.client_x() as f32, ev.client_y() as f32);
            }
        });
    });

    let on_drag = window_event_listener(ev::drag, move |ev: ev::DragEvent| {
        if drag_field.get().is_none() {
            return;
        }
        ghost.update(|current| {
            if let Some(active) = current.as_mut() {
                move_column_drag_ghost(active, ev.client_x() as f32, ev.client_y() as f32);
            }
        });
    });

    let on_pointer_up = window_event_listener(ev::pointerup, move |ev: ev::PointerEvent| {
        if drag_field.get().is_none() {
            return;
        }
        #[cfg(feature = "hydrate")]
        {
            let source = drag_field.get().unwrap_or_default();
            if let Some(target) = resolve_target(ev.client_x() as f32, ev.client_y() as f32) {
                if source != target {
                    on_reorder(source, target);
                }
            }
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = ev;
        dragging.set(false);
        drag_field.set(None);
        ghost.set(None);
        listeners.update_value(|handles| {
            for handle in handles.drain(..) {
                handle.remove();
            }
        });
    });

    listeners.update_value(|handles| {
        handles.push(on_pointer_move);
        handles.push(on_drag);
        handles.push(on_pointer_up);
    });
}

pub fn begin_column_drag_ghost(
    ghost: RwSignal<Option<ColumnDragGhost>>,
    label: String,
    width_px: f32,
    x: f32,
    y: f32,
) {
    ghost.set(Some(column_drag_ghost_at_pointer(label, width_px, x, y)));
}

pub fn clear_column_drag_ghost(ghost: RwSignal<Option<ColumnDragGhost>>) {
    ghost.set(None);
}

pub fn apply_column_reorder(state: DataTableTableState, source: &str, target: &str) {
    if source != target {
        state.reorder_column(source, target, true);
    }
}
