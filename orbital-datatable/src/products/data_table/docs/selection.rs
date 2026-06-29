use leptos::prelude::*;
use orbital_macros::component_doc;

/// Row multiselect with shift-click ranges and spreadsheet-style cell range selection.
///
/// Set `selection_mode` for row checkboxes; enable `CELL_SELECTION` on `features` for cell ranges.
///
/// # When to use
///
/// - Bulk actions on selected rows
/// - Copying rectangular cell regions
///
/// # Usage
///
/// 1. Set `selection_mode=DataTableSelectionMode::Multiselect` for row selection.
/// 2. Enable `DataTableFeatures::CELL_SELECTION` for cell ranges.
/// 3. Bind `selection` signal or listen to `on_selection_change` for controlled mode.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use shift-click for contiguous row ranges in long lists
/// * Pair cell selection with `CLIPBOARD` for copy/paste workflows on the [Export](/data-table-export) page
///
/// ## Don'ts
///
/// * Do not enable cell selection on mobile-only layouts without keyboard alternatives
///
/// # Selection reference
///
/// | Type / prop | Description |
/// |-------------|-------------|
/// | `selection_mode` | `Single` or `Multiselect` row checkboxes |
/// | `selection` | Controlled set of selected row ids |
/// | `CELL_SELECTION` | Spreadsheet-style rectangular cell ranges |
/// | [`CellSelection`] | Anchor and focus coords for cell range |
///
///
/// # Examples
///
///
/// ## Shift range selection
/// Multiselect with shift-click to select a contiguous row range.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, DataTableSelectionMode};
/// let items = RwSignal::new((1..=8).map(|i| {
///     DataTableRowModel::from_text_cells(
///         i.to_string(),
///         HashMap::from([("name".into(), format!("Row {i}"))]),
///     )
/// }).collect());
/// view! {
///     <div data-testid="data-table-range-select-preview">
///         <DataTable
///             selection_mode=DataTableSelectionMode::Multiselect
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Cell range selection
/// Spreadsheet-style cell ranges with shift-click and arrow keys (`CELL_SELECTION`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())])),
///     DataTableRowModel::from_text_cells("3", HashMap::from([("name".into(), "Bob".into()), ("role".into(), "Member".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-range-selection-preview">
///         <DataTable
///             features=DataTableFeatures::CELL_SELECTION
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role")]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group = "Selection & IO",
    group_priority = 70,
    preview_slug = "data-table-selection",
    preview_label = "Selection",
    preview_icon = icondata::AiCheckSquareOutlined,
)]
#[component]
pub fn DataTableSelectionDoc() -> impl IntoView {
    view! { () }
}
