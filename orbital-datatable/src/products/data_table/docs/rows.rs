use leptos::prelude::*;
use orbital_macros::component_doc;

/// Row identity, sizing, spanning, detail panels, reorder, and pinning.
///
/// Wrap dataset records as [`DataTableRowModel`] and optionally override ids with `get_row_id`.
///
/// # When to use
///
/// - Stable row keys from business fields (SKU, order id)
/// - Expandable detail content below a row
/// - Sticky summary rows at top or bottom while scrolling
///
/// # Usage
///
/// 1. Map records to [`DataTableRowModel`] via `items` or [`DataTableSource::Client`].
/// 2. Set `get_row_id` when keys differ from [`DataRecord::id`](crate::DataRecord).
/// 3. Enable `ROW_DETAIL`, `ROW_REORDER`, or `ROW_PINNING` on `features` as needed.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `auto_row_height` with multi-line `cell_view` content
/// * Pin at most one or two summary rows to avoid viewport clutter
///
/// ## Don'ts
///
/// * Do not rely on row index as id — use stable business keys
///
/// # Row reference
///
/// | Type / prop | Description |
/// |-------------|-------------|
/// | [`DataTableRowModel`] | Wraps a [`DataRecord`] with optional [`DataTableRowKind`] for tree/group rows |
/// | `get_row_id` | Override stable row id used for selection, pinning, and scroll |
/// | `auto_row_height` | Size rows to multi-line cell content |
/// | `ROW_DETAIL` | Expandable detail panel below a row |
/// | `ROW_REORDER` | Drag rows to reorder (client data) |
/// | `ROW_PINNING` | Sticky top/bottom rows via [`PinnedRowsState`] |
///
///
/// # Examples
///
///
/// ## Custom row id
/// `get_row_id` resolves stable row keys from record fields instead of `DataRecord.id`.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, GetRowId};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("rec-1", HashMap::from([
///         ("sku".into(), DataValue::Text("SKU-42".into())),
///         ("name".into(), DataValue::Text("Widget".into())),
///     ]))),
///     DataTableRowModel::new(DataRecord::new("rec-2", HashMap::from([
///         ("sku".into(), DataValue::Text("SKU-99".into())),
///         ("name".into(), DataValue::Text("Gadget".into())),
///     ]))),
/// ]);
/// let get_row_id = GetRowId::new(|(record,)| {
///     record.get("sku").map(|v| v.display_string()).unwrap_or_else(|| record.id.clone())
/// });
/// view! {
///     <div data-testid="data-table-get-row-id-preview">
///         <DataTable
///             get_row_id=get_row_id
///             columns=vec![
///                 DataTableColumnDef::new("sku", "SKU"),
///                 DataTableColumnDef::new("name", "Name"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Auto row height
/// Rows grow with multi-line cell content while density still controls padding.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel};
/// use orbital_core_components::{Flex, FlexGap, ThemeDensityStepper};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([
///         ("notes".into(), DataValue::Text("Line one.\nLine two.\nLine three.".into())),
///     ]))),
/// ]);
/// let notes_view = Arc::new(|record: orbital_data::DataRecord| {
///     let text = record.get("notes").map(|v| v.display_string()).unwrap_or_default();
///     view! { <span style="white-space: pre-line">{text}</span> }.into_any()
/// });
/// view! {
///     <div data-testid="data-table-row-sizing-preview">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <ThemeDensityStepper />
///             <DataTable
///                 auto_row_height=true
///                 columns=vec![
///                     DataTableColumnDef::new("notes", "Notes").with_cell_view(notes_view),
///                 ]
///                 items=items
///             />
///         </Flex>
///     </div>
/// }
/// ```
///
///
/// ## Row spanning
/// Consecutive equal values in a column merge into one cell (`row_span_merge`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableRowModel, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([("qty".into(), DataValue::Number(2.0)), ("item".into(), DataValue::Text("A".into()))]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([("qty".into(), DataValue::Number(2.0)), ("item".into(), DataValue::Text("B".into()))]))),
///     DataTableRowModel::new(DataRecord::new("3", HashMap::from([("qty".into(), DataValue::Number(5.0)), ("item".into(), DataValue::Text("C".into()))]))),
///     DataTableRowModel::new(DataRecord::new("4", HashMap::from([("qty".into(), DataValue::Number(5.0)), ("item".into(), DataValue::Text("D".into()))]))),
///     DataTableRowModel::new(DataRecord::new("5", HashMap::from([("qty".into(), DataValue::Number(5.0)), ("item".into(), DataValue::Text("E".into()))]))),
///     DataTableRowModel::new(DataRecord::new("6", HashMap::from([("qty".into(), DataValue::Number(1.0)), ("item".into(), DataValue::Text("F".into()))]))),
/// ]);
/// view! {
///     <div data-testid="data-table-row-span-preview">
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("qty", "Qty").with_col_type(ColumnType::Number).with_row_span_merge(true),
///                 DataTableColumnDef::new("item", "Item"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Row detail panel
/// Expand a row to reveal a detail slot (`ROW_DETAIL` flag).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowDetail, DataTableRowModel, RowDetailView};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([
///         ("name".into(), DataValue::Text("Order 1001".into())),
///         ("notes".into(), DataValue::Text("Ships next week.".into())),
///     ]))),
/// ]);
/// let detail: RowDetailView = Arc::new(|record: orbital_data::DataRecord| {
///     let notes = record.get("notes").map(|v| v.display_string()).unwrap_or_default();
///     view! { <p>{notes}</p> }.into_any()
/// });
/// view! {
///     <div data-testid="data-table-row-detail-preview">
///         <DataTable
///             features=DataTableFeatures::ROW_DETAIL
///             columns=vec![DataTableColumnDef::new("name", "Order")]
///             items=items
///         >
///             <DataTableRowDetail slot render=detail />
///         </DataTable>
///     </div>
/// }
/// ```
///
///
/// ## Row reorder
/// Drag rows to reorder when `ROW_REORDER` is enabled.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Alpha".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Beta".into())])),
///     DataTableRowModel::from_text_cells("3", HashMap::from([("name".into(), "Gamma".into())])),
///     DataTableRowModel::from_text_cells("4", HashMap::from([("name".into(), "Delta".into())])),
///     DataTableRowModel::from_text_cells("5", HashMap::from([("name".into(), "Epsilon".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-row-reorder-preview">
///         <DataTable
///             features=DataTableFeatures::ROW_REORDER
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Row pinning
/// Sticky top and bottom rows stay visible while scrolling (`ROW_PINNING`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableInitialState, DataTableRowModel, PagingMode, PinnedRowsState};
/// let items = RwSignal::new((1..=16).map(|i| {
///     let label = if i == 1 { "Summary (pinned top)".into() }
///         else if i == 16 { "Totals (pinned bottom)".into() }
///         else { format!("Row {i}") };
///     DataTableRowModel::from_text_cells(i.to_string(), HashMap::from([("name".into(), label)]))
/// }).collect());
/// view! {
///     <div data-testid="data-table-row-pinning-preview">
///         <DataTable
///             features=DataTableFeatures::ROW_PINNING
///             paging=PagingMode::None
///             max_height=280.0
///             initial_state=DataTableInitialState {
///                 pinned_rows: PinnedRowsState {
///                     top: vec!["1".into()],
///                     bottom: vec!["16".into()],
///                 },
///                 ..Default::default()
///             }
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 30,
    preview_slug = "data-table-rows",
    preview_label = "Rows",
    preview_icon = icondata::AiMenuOutlined,
)]
#[component]
pub fn DataTableRowsDoc() -> impl IntoView {
    view! { () }
}
