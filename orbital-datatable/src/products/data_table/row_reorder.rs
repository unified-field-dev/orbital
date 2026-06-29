use leptos::{ev, prelude::*};

use crate::core::{
    move_row_drag_ghost, row_drag_ghost_at_pointer, use_data_table_context, RowDragGhost,
};
use crate::engine::{format_display, resolve_value, ResolvedColumn};
use crate::types::DataTableRowModel;

/// Follower row preview rendered at the table root while a row is dragged.
#[component]
pub fn DataTableRowDragGhost() -> impl IntoView {
    let ctx = use_data_table_context();

    view! {
        {move || {
            ctx.row_drag_ghost.get().map(|ghost| {
                let style = format!("left: {:.0}px; top: {:.0}px;", ghost.x, ghost.y);
                view! {
                    <div
                        class="orbital-data-table__row-drag-ghost"
                        style=style
                        data-testid="data-table-row-drag-ghost"
                    >
                        {ghost
                            .cells
                            .iter()
                            .zip(ghost.widths_px.iter())
                            .map(|(text, width)| {
                                let cell_style = format!("width: {:.0}px; min-width: {:.0}px;", width, width);
                                view! {
                                    <span class="orbital-data-table__row-drag-ghost-cell" style=cell_style>
                                        {text.clone()}
                                    </span>
                                }
                            })
                            .collect_view()}
                    </div>
                }
            })
        }}
    }
}

pub fn build_row_drag_ghost(
    row: &DataTableRowModel,
    columns: &[ResolvedColumn],
    x: f32,
    y: f32,
) -> RowDragGhost {
    let mut cells = Vec::with_capacity(columns.len());
    let mut widths_px = Vec::with_capacity(columns.len());
    for col in columns {
        let value = resolve_value(&col.def, row);
        cells.push(format_display(&col.def, &value));
        widths_px.push(col.width_px as f32);
    }
    row_drag_ghost_at_pointer(cells, widths_px, x, y)
}

pub fn begin_row_drag_ghost(
    ghost: RwSignal<Option<RowDragGhost>>,
    row: &DataTableRowModel,
    columns: &[ResolvedColumn],
    x: f32,
    y: f32,
) {
    ghost.set(Some(build_row_drag_ghost(row, columns, x, y)));
}

pub fn clear_row_drag_ghost(ghost: RwSignal<Option<RowDragGhost>>) {
    ghost.set(None);
}

pub fn move_row_drag_ghost_on_event(ghost: RwSignal<Option<RowDragGhost>>, ev: &ev::DragEvent) {
    if ghost.get().is_none() {
        return;
    }
    ghost.update(|current| {
        if let Some(active) = current.as_mut() {
            move_row_drag_ghost(active, ev.client_x() as f32, ev.client_y() as f32);
        }
    });
}
