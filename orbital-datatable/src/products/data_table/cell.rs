use leptos::prelude::*;
use orbital_core_components::{TableCell, TableCellLayout};

use super::cell_selection_handlers::{
    cell_range_classes, handle_cell_mouse_down, handle_cell_mouse_enter,
};
use super::column_styles::{column_cell_classes, column_combined_style};
use super::edit::data_table_default_editor_view;
use crate::engine::{format_display, resolve_value, ResolvedColumn};
use crate::types::{DataTableColumnDef, DataTableRowModel, DataTableTableState, EditMode};

#[inline]
fn is_cell_editing(
    state: DataTableTableState,
    row_id: &str,
    field: &str,
    column: &DataTableColumnDef,
) -> bool {
    state.editing_enabled()
        && column.is_editable()
        && state.edit_session.is_editing_field(row_id, field)
}

fn begin_cell_edit(state: DataTableTableState, row_id: &str, field: &str, edit_mode: EditMode) {
    match edit_mode {
        EditMode::Cell => state.start_cell_edit(row_id, field),
        EditMode::Row => state.start_row_edit(row_id),
    }
}

/// Body cell for [`super::row::DataTableRow`].
#[component]
pub fn DataTableCell(
    state: DataTableTableState,
    row: DataTableRowModel,
    resolved: ResolvedColumn,
    row_id: String,
    col_index: usize,
    #[prop(default = None)] colspan: Option<u32>,
    #[prop(default = None)] rowspan: Option<u32>,
    #[prop(default = None)] extra_style: Option<String>,
) -> impl IntoView {
    let column = resolved.def.clone();
    let field = column.field.clone();
    let cell_testid = format!("data-table-cell-{row_id}-{field}");
    let cell_class = column_cell_classes(&resolved);
    let mut cell_style = column_combined_style(&resolved);
    if let Some(extra) = extra_style {
        if !extra.is_empty() {
            if !cell_style.is_empty() {
                cell_style.push(' ');
            }
            cell_style.push_str(&extra);
        }
    }

    let editable_class = format!("{cell_class} orbital-data-table__cell--editable");
    let editing_class = format!("{cell_class} orbital-data-table__cell--editing");
    let editable = column.is_editable() && state.editing_enabled();

    let cell_class_signal = {
        let cell_class = cell_class.clone();
        let row_id_for_range = row_id.clone();
        let field_for_range = field.clone();
        let row_id_for_edit = row_id.clone();
        let field_for_edit = field.clone();
        let column_for_edit = column.clone();
        let record_for_class = row.record.clone();
        let cell_class_cb = column.cell_class;
        Signal::derive(move || {
            let _ = state.render_key.get();
            let _ = state.edit_session.session.get();
            let _ = state.cell_selection.get();
            let _ = state.focus_cell.get();
            let mut classes =
                if is_cell_editing(state, &row_id_for_edit, &field_for_edit, &column_for_edit) {
                    editing_class.clone()
                } else if editable {
                    editable_class.clone()
                } else {
                    cell_class.clone()
                };
            if let Some(cb) = &cell_class_cb {
                let extra = cb.run((record_for_class.clone(),));
                if !extra.is_empty() {
                    classes.push(' ');
                    classes.push_str(&extra);
                }
            }
            let range_classes = cell_range_classes(state, &row_id_for_range, &field_for_range);
            if !range_classes.is_empty() {
                classes.push(' ');
                classes.push_str(&range_classes);
            }
            classes
        })
    };

    let tabindex_signal = {
        let row_id_tab = row_id.clone();
        let field_tab = field.clone();
        Signal::derive(move || {
            let focused = state.focus_cell.get();
            if focused
                .as_ref()
                .is_some_and(|c| c.row_id == row_id_tab && c.field == field_tab)
            {
                0
            } else {
                -1
            }
        })
    };

    let value = resolve_value(&column, &row);
    let display = format_display(&column, &value);
    let cell_view = column.cell_view.clone();
    let record = row.record.clone();
    let column_for_editor = column.clone();

    let row_id_mousedown = row_id.clone();
    let field_mousedown = field.clone();
    let row_id_enter = row_id.clone();
    let field_enter = field.clone();
    let row_id_focus = row_id.clone();
    let field_focus = field.clone();
    let row_id_click = row_id.clone();
    let field_click = field.clone();
    let row_id_dblclick = row_id.clone();
    let field_dblclick = field.clone();
    let column_dblclick = column.clone();
    let row_id_keydown = row_id.clone();
    let field_keydown = field.clone();
    let column_keydown = column.clone();
    let row_id_show = row_id.clone();
    let field_show = field.clone();
    let column_show = column.clone();

    let aria_col_index = col_index + 1;

    view! {
        <TableCell
            class=cell_class_signal
            style=cell_style.clone()
            colspan=colspan
            rowspan=rowspan
            attr:data-testid=cell_testid
            on:click=move |ev: leptos::ev::MouseEvent| {
                if let Some(events) = state.events.get_value() {
                    events.notify_cell_click(&row_id_click, &field_click);
                }
                let _ = ev;
            }
            on:mousedown=move |ev: leptos::ev::MouseEvent| {
                if state.cell_selection_enabled() {
                    ev.prevent_default();
                    handle_cell_mouse_down(
                        state,
                        row_id_mousedown.clone(),
                        field_mousedown.clone(),
                        ev.shift_key(),
                    );
                }
            }
            on:mouseenter=move |_ev: leptos::ev::MouseEvent| {
                handle_cell_mouse_enter(state, row_id_enter.clone(), field_enter.clone());
            }
            on:dblclick=move |ev: leptos::ev::MouseEvent| {
                if is_cell_editing(state, &row_id_dblclick, &field_dblclick, &column_dblclick) {
                    ev.stop_propagation();
                    return;
                }
                if !state.editing_enabled() || !column_dblclick.is_editable() {
                    return;
                }
                ev.prevent_default();
                ev.stop_propagation();
                begin_cell_edit(state, &row_id_dblclick, &field_dblclick, state.edit_mode);
            }
            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                if is_cell_editing(state, &row_id_keydown, &field_keydown, &column_keydown) {
                    return;
                }
                if column_keydown.is_editable()
                    && state.editing_enabled()
                    && ev.key() == "Enter"
                {
                    ev.prevent_default();
                    begin_cell_edit(state, &row_id_keydown, &field_keydown, state.edit_mode);
                }
            }
        >
            <TableCellLayout>
                <span
                    class="orbital-data-table__cell-focus"
                    attr:aria-colindex=aria_col_index.to_string()
                    data-table-role="gridcell"
                    tabindex=move || tabindex_signal.get().to_string()
                    on:focus=move |_| {
                        state.set_focus_cell(Some(crate::types::CellCoord::new(
                            row_id_focus.clone(),
                            field_focus.clone(),
                        )));
                    }
                >
                <Show
                    when=move || {
                        let _ = state.edit_session.session.get();
                        is_cell_editing(state, &row_id_show, &field_show, &column_show)
                    }
                    fallback=move || {
                        if let Some(cell_view) = cell_view.clone() {
                            cell_view(record.clone()).into_any()
                        } else {
                            view! { <span>{display.clone()}</span> }.into_any()
                        }
                    }
                >
                    {data_table_default_editor_view(
                        state,
                        row_id.clone(),
                        field.clone(),
                        column_for_editor.clone(),
                        row.clone(),
                    )}
                </Show>
                </span>
            </TableCellLayout>
        </TableCell>
    }
}

/// Back-compat alias for call sites that still invoke the cell renderer as a function.
pub fn data_table_cell_view(
    state: DataTableTableState,
    row: DataTableRowModel,
    resolved: ResolvedColumn,
    row_id: String,
    col_index: usize,
    colspan: Option<u32>,
    rowspan: Option<u32>,
    extra_style: Option<String>,
) -> impl IntoView {
    view! {
        <DataTableCell
            state=state
            row=row
            resolved=resolved
            row_id=row_id
            col_index=col_index
            colspan=colspan
            rowspan=rowspan
            extra_style=extra_style
        />
    }
}
