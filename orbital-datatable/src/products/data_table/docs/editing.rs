use leptos::prelude::*;
use orbital_macros::component_doc;

/// Inline cell and row editing with validation, async commit, and optional undo history.
///
/// Mark columns editable on [`DataTableColumnDef`] and handle commits via [`DataTableEvents::on_row_update`].
///
/// # When to use
///
/// - Spreadsheet-style inline edits in admin grids
/// - Validated numeric or enum fields
/// - Reversible edit sessions with undo/redo toolbar
///
/// # Usage
///
/// 1. Set `editable(true)` on editable columns.
/// 2. Wire `on_row_update` in `events` for async validation.
/// 3. Enable `UNDO_REDO` on `features` and `show_undo_toolbar=true` for history controls.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`ColumnType`]-appropriate editors (Select for enums, Checkbox for booleans)
/// * Return `Err` from `on_row_update` to reject invalid commits
///
/// ## Don'ts
///
/// * Do not edit server-sourced rows without updating the fetcher contract
///
///
/// # Examples
///
///
/// ## Inline cell editing
/// Double-click an editable cell to edit; press Enter to commit or Escape to cancel.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("role".into(), "Admin".into()),
///         ("active".into(), "true".into()),
///     ])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([
///         ("name".into(), "Grace".into()),
///         ("role".into(), "Member".into()),
///         ("active".into(), "false".into()),
///     ])),
/// ]);
/// view! {
///     <div data-testid="data-table-editing-preview">
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name").with_editable(true),
///                 DataTableColumnDef::new("role", "Role")
///                     .with_col_type(ColumnType::SingleSelect)
///                     .with_editable(true)
///                     .with_edit_options(vec!["Admin".into(), "Member".into()]),
///                 DataTableColumnDef::new("active", "Active")
///                     .with_col_type(ColumnType::Boolean)
///                     .with_editable(true),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Edit validation
/// Field validators show inline errors; `on_row_update` can reject commits with a dialog.
/// <!-- preview -->
/// ```rust
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableEvents, DataTableRowModel, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([
///         ("name".into(), DataValue::Text("Ada".into())),
///         ("score".into(), DataValue::Number(50.0)),
///     ]))),
/// ]);
/// let events = DataTableEvents {
///     on_row_update: Some(Callback::new(|(mut record,): (DataRecord,)| {
///         if record.get("name").map(|v| v.display_string()).as_deref() == Some("Blocked") {
///             Err("Name 'Blocked' is not allowed".into())
///         } else {
///             Ok(record)
///         }
///     })),
///     ..Default::default()
/// };
/// view! {
///     <div data-testid="data-table-edit-validation-preview">
///         <DataTable
///             paging=PagingMode::None
///             data_table_events=events
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name").with_editable(true),
///                 DataTableColumnDef::new("score", "Score")
///                     .with_col_type(ColumnType::Number)
///                     .with_editable(true)
///                     .with_validate_value(Callback::new(|(value,): (DataValue,)| {
///                         if let DataValue::Number(n) = value {
///                             if n > 100.0 {
///                                 Err("Score must be 100 or less".into())
///                             } else {
///                                 Ok(value)
///                             }
///                         } else {
///                             Ok(value)
///                         }
///                     })),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Undo and redo
/// Edit history stack with toolbar controls (`UNDO_REDO`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Alpha".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Beta".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-undo-redo-preview">
///         <DataTable
///             features=DataTableFeatures::UNDO_REDO
///             show_undo_toolbar=true
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name").with_editable(true)]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 40,
    preview_slug = "data-table-editing",
    preview_label = "Editing",
    preview_icon = icondata::AiEditOutlined,
)]
#[component]
pub fn DataTableEditingDoc() -> impl IntoView {
    view! { () }
}
