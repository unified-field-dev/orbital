use leptos::prelude::*;

use crate::core::DataTableContext;
use crate::engine::{move_focus, CellMoveDirection};
use crate::types::{CellCoord, DataTableTableState};

/// Handle WAI-ARIA grid keyboard navigation and cell-selection shortcuts.
pub fn handle_grid_keydown(
    state: DataTableTableState,
    ctx: DataTableContext,
    ev: leptos::ev::KeyboardEvent,
) {
    if state
        .edit_session
        .session
        .with(|s| !matches!(s, crate::types::EditSession::Idle))
    {
        return;
    }

    let key = ev.key();
    let shift = ev.shift_key();
    let ctrl_or_meta = ev.ctrl_key() || ev.meta_key();

    let row_ids = state.visible_row_ids();
    let fields = state.visible_data_fields();

    if !row_ids.is_empty() && !fields.is_empty() {
        match key.as_str() {
            "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
                ev.prevent_default();
                let direction = match key.as_str() {
                    "ArrowUp" => CellMoveDirection::Up,
                    "ArrowDown" => CellMoveDirection::Down,
                    "ArrowLeft" => CellMoveDirection::Left,
                    _ => CellMoveDirection::Right,
                };

                if state.cell_selection_enabled() {
                    state.cell_selection.update(|sel| {
                        move_focus(sel, direction, &row_ids, &fields, shift);
                    });
                    if let Some(focus) = state.cell_selection.get().focus.clone() {
                        state.set_focus_cell(Some(focus));
                    }
                } else {
                    let mut focus = state
                        .focus_cell
                        .get()
                        .unwrap_or_else(|| CellCoord::new(row_ids[0].clone(), fields[0].clone()));
                    let row_idx = row_ids
                        .iter()
                        .position(|id| id == &focus.row_id)
                        .unwrap_or(0);
                    let col_idx = fields.iter().position(|f| f == &focus.field).unwrap_or(0);
                    let (new_row, new_col) = match direction {
                        CellMoveDirection::Up => (row_idx.saturating_sub(1), col_idx),
                        CellMoveDirection::Down => ((row_idx + 1).min(row_ids.len() - 1), col_idx),
                        CellMoveDirection::Left => (row_idx, col_idx.saturating_sub(1)),
                        CellMoveDirection::Right => (row_idx, (col_idx + 1).min(fields.len() - 1)),
                    };
                    focus = CellCoord::new(row_ids[new_row].clone(), fields[new_col].clone());
                    state.set_focus_cell(Some(focus));
                    state.scroll_to_row_id(&row_ids[new_row]);
                }
                state.bump_render();
            }
            "Home" => {
                ev.prevent_default();
                if ctrl_or_meta {
                    if let (Some(row_id), Some(field)) = (row_ids.first(), fields.first()) {
                        state.set_focus_cell(Some(CellCoord::new(row_id.clone(), field.clone())));
                        state.scroll_to_row_id(row_id);
                    }
                } else if let Some(focus) = state.focus_cell.get() {
                    if let Some(field) = fields.first() {
                        state.set_focus_cell(Some(CellCoord::new(focus.row_id, field.clone())));
                    }
                }
                state.bump_render();
            }
            "End" => {
                ev.prevent_default();
                if ctrl_or_meta {
                    if let (Some(row_id), Some(field)) = (row_ids.last(), fields.last()) {
                        state.set_focus_cell(Some(CellCoord::new(row_id.clone(), field.clone())));
                        state.scroll_to_row_id(row_id);
                    }
                } else if let Some(focus) = state.focus_cell.get() {
                    if let Some(field) = fields.last() {
                        state.set_focus_cell(Some(CellCoord::new(focus.row_id, field.clone())));
                    }
                }
                state.bump_render();
            }
            "PageUp" | "PageDown" => {
                ev.prevent_default();
                let page_rows = 10usize;
                let focus = state
                    .focus_cell
                    .get()
                    .unwrap_or_else(|| CellCoord::new(row_ids[0].clone(), fields[0].clone()));
                let row_idx = row_ids
                    .iter()
                    .position(|id| id == &focus.row_id)
                    .unwrap_or(0);
                let new_idx = if key == "PageUp" {
                    row_idx.saturating_sub(page_rows)
                } else {
                    (row_idx + page_rows).min(row_ids.len().saturating_sub(1))
                };
                let row_id = row_ids[new_idx].clone();
                state.set_focus_cell(Some(CellCoord::new(row_id.clone(), focus.field)));
                state.scroll_to_row_id(&row_id);
                state.bump_render();
            }
            " " if ctx.selection_mode.get().is_some() => {
                if let Some(focus) = state.focus_cell.get() {
                    let multiselect = ctx.selection_mode.get()
                        == Some(crate::types::DataTableSelectionMode::Multiselect);
                    state.toggle_row_selection(&focus.row_id, multiselect);
                }
            }
            "Enter" => {
                if let Some(focus) = state.focus_cell.get() {
                    if let Some(events) = ctx.events.get_value() {
                        events.notify_row_click(&focus.row_id);
                    }
                }
            }
            "Escape" => {
                state.clear_cell_selection();
                state.set_focus_cell(None);
            }
            _ => {}
        }
    }

    super::cell_selection_handlers::handle_clipboard_keys(state, ev, ctrl_or_meta, &key);
}
