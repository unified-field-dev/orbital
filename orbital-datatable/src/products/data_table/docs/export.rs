use leptos::prelude::*;
use orbital_macros::component_doc;

/// Export filtered rows to CSV or XLSX and copy/paste cell ranges via the clipboard API.
///
/// CSV export is always available from the toolbar menu; enable `EXCEL_EXPORT` and `CLIPBOARD` on `features` for extended IO.
///
/// # When to use
///
/// - Download current view as CSV or Excel
/// - Spreadsheet-style copy/paste between cells
///
/// # Usage
///
/// 1. Open the toolbar Export menu for CSV/print.
/// 2. Enable `EXCEL_EXPORT` for xlsx download.
/// 3. Enable `CELL_SELECTION | CLIPBOARD` for Ctrl+C / Ctrl+V on ranges.
///
/// # Best Practices
///
/// ## Do's
///
/// * Export respects current sort, filter, and visible columns
/// * Use [`DataTableHandle::export_csv`] for programmatic export
///
/// ## Don'ts
///
/// * Do not export hidden columns when users expect WYSIWYG downloads
///
///
/// # Examples
///
///
/// ## CSV export
/// Toolbar export menu downloads filtered rows as CSV and supports print.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-export-preview">
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Excel export
/// Optional xlsx download behind `EXCEL_EXPORT`.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("score".into(), "95".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-excel-export-preview">
///         <DataTable
///             features=DataTableFeatures::EXCEL_EXPORT
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("score", "Score")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Clipboard copy/paste
/// Copy cell ranges with Ctrl+C and paste into editable cells with Ctrl+V (`CLIPBOARD` + `CELL_SELECTION`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-clipboard-preview">
///         <DataTable
///             features=DataTableFeatures::CELL_SELECTION | DataTableFeatures::CLIPBOARD
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name").with_editable(true),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group = "Selection & IO",
    group_priority = 70,
    preview_slug = "data-table-export",
    preview_label = "Export & Clipboard",
    preview_icon = icondata::AiExportOutlined,
)]
#[component]
pub fn DataTableExportDoc() -> impl IntoView {
    view! { () }
}
