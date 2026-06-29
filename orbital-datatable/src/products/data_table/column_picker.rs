use leptos::{ev, leptos_dom::helpers::WindowListenerHandle, prelude::*};
use orbital_core_components::{
    Button, ButtonAppearance, Checkbox, Popover, PopoverPosition, PopoverTrigger,
    PopoverTriggerType,
};

use super::column_reorder::{
    apply_column_reorder, attach_pointer_reorder_end, begin_column_drag_ghost,
    clear_column_drag_ghost, picker_field_at_client_point,
};
use crate::core::use_data_table_context;
use crate::engine::ordered_column_defs;
use crate::types::{DataTableColumnDef, DataTableFeatures, DataTableTableState};

/// Column visibility picker panel (Popover + Checkbox list).
#[component]
pub fn DataTableColumnPicker(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    let allow_hide = ctx.header_chrome.get_value().column_hide;
    let reorder_enabled = state.features.contains(DataTableFeatures::COLUMN_REORDER);

    view! {
        <Popover trigger_type=PopoverTriggerType::Click position=PopoverPosition::BottomEnd>
            <PopoverTrigger slot>
                <Button
                    appearance=ButtonAppearance::Subtle
                    attr:data-testid="data-table-column-picker-trigger"
                >
                    "Columns"
                </Button>
            </PopoverTrigger>
            <div
                class="orbital-data-table__column-picker-panel"
                data-testid="data-table-column-picker-panel"
            >
                <div class="orbital-data-table__column-picker-title">
                    {if reorder_enabled { "Show and reorder columns" } else { "Show columns" }}
                </div>
                <div class="orbital-data-table__column-picker-list">
                    <For
                        each=move || {
                            let _ = state.column_order.get();
                            ordered_column_defs(
                                &state.columns.get_value(),
                                &state.column_order.get(),
                            )
                            .into_iter()
                            .filter(|col| if allow_hide { col.hideable } else { !col.hideable })
                            .collect::<Vec<_>>()
                        }
                        key=|col| col.field.clone()
                        children=move |column| {
                            view! {
                                <DataTableColumnPickerRow
                                    state=state
                                    column=column
                                    reorder_enabled=reorder_enabled
                                />
                            }
                        }
                    />
                </div>
            </div>
        </Popover>
    }
}

#[component]
fn DataTableColumnPickerRow(
    state: DataTableTableState,
    column: DataTableColumnDef,
    reorder_enabled: bool,
) -> impl IntoView {
    let ctx = use_data_table_context();
    let field = column.field.clone();
    let label = column.header_name.clone();
    let testid = format!("data-table-column-picker-{field}");
    let field_for_change = field.clone();
    let drop_field = field.clone();
    let can_reorder = reorder_enabled && column.reorderable;
    let checked = RwSignal::new(true);
    let dragging = RwSignal::new(false);
    let drag_field = RwSignal::new(None::<String>);
    let reorder_listeners = StoredValue::new(Vec::<WindowListenerHandle>::new());

    Effect::new({
        let field = field.clone();
        move |_| {
            let visible = !state
                .column_visibility
                .with(|map| matches!(map.get(&field), Some(false)));
            checked.set(visible);
        }
    });

    let drag_handle_testid = format!("data-table-column-picker-drag-{field}");

    view! {
        <div
            class="orbital-data-table__column-picker-row"
            data-column-picker-field=field.clone()
            on:dragover=move |ev: leptos::ev::DragEvent| {
                if can_reorder {
                    ev.prevent_default();
                }
            }
            on:drop=move |ev: leptos::ev::DragEvent| {
                if !can_reorder {
                    return;
                }
                ev.prevent_default();
                let source = ev
                    .data_transfer()
                    .and_then(|dt| dt.get_data("text/plain").ok())
                    .filter(|s| !s.is_empty())
                    .or_else(|| drag_field.get());
                if let Some(source) = source {
                    apply_column_reorder(state, &source, &drop_field);
                }
                dragging.set(false);
                drag_field.set(None);
                clear_column_drag_ghost(ctx.column_drag_ghost);
            }
        >
            {can_reorder.then(|| {
                let drag_start_field = field.clone();
                let pointer_field = field.clone();
                let label_for_drag_start = label.clone();
                let label_for_pointer = label.clone();
                view! {
                    <span
                        class=move || {
                            let mut classes =
                                vec!["orbital-data-table__column-picker-drag-handle".to_string()];
                            if dragging.get() {
                                classes.push("orbital-data-table__header--dragging".to_string());
                            }
                            classes.join(" ")
                        }
                        draggable=true
                        data-drag-field=drag_start_field.clone()
                        data-testid=drag_handle_testid.clone()
                        attr:aria-label="Drag to reorder column"
                        on:dragstart=move |ev: leptos::ev::DragEvent| {
                            dragging.set(true);
                            drag_field.set(Some(drag_start_field.clone()));
                            begin_column_drag_ghost(
                                ctx.column_drag_ghost,
                                label_for_drag_start.clone(),
                                160.0,
                                ev.client_x() as f32,
                                ev.client_y() as f32,
                            );
                            if let Some(dt) = ev.data_transfer() {
                                let _ = dt.set_data("text/plain", &drag_start_field);
                            }
                        }
                        on:dragend=move |_| {
                            dragging.set(false);
                            drag_field.set(None);
                            clear_column_drag_ghost(ctx.column_drag_ghost);
                        }
                        on:pointerdown=move |ev: ev::PointerEvent| {
                            ev.prevent_default();
                            dragging.set(true);
                            drag_field.set(Some(pointer_field.clone()));
                            begin_column_drag_ghost(
                                ctx.column_drag_ghost,
                                label_for_pointer.clone(),
                                160.0,
                                ev.client_x() as f32,
                                ev.client_y() as f32,
                            );
                            let state = state;
                            attach_pointer_reorder_end(
                                reorder_listeners,
                                dragging,
                                drag_field,
                                ctx.column_drag_ghost,
                                move |source, target| {
                                    apply_column_reorder(state, &source, &target);
                                },
                                picker_field_at_client_point,
                            );
                        }
                    >
                        "⠿"
                    </span>
                }
            })}
            <Checkbox
                checked=checked
                label=label
                disabled=Signal::from(false)
                on_change=Callback::new(move |visible: bool| {
                    state.set_column_visible(&field_for_change, visible);
                })
                attr:data-testid=testid
            />
        </div>
    }
}
