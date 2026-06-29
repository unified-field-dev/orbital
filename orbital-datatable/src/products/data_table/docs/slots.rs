use leptos::prelude::*;
use orbital_core_components::{Toolbar, ToolbarButton};
use orbital_macros::component_doc;

use crate::{
    DataTable, DataTableColumnDef, DataTableEmptyView, DataTableFooterSlot, DataTableRowModel,
    DataTableToolbarSlot, PagingMode,
};

/// Leptos slot regions for DataTable composition.
///
/// # When to use
///
/// - Replacing the default toolbar, footer, or loading/empty overlays.
/// - Custom expandable row detail panels via [`DataTableRowDetail`].
///
/// # Usage
///
/// Nest slot children on [`DataTable`]: [`DataTableToolbarSlot`], [`DataTableFooterSlot`],
/// [`DataTableEmptyView`], [`DataTableNoResultsView`], [`DataTableLoadingView`], and
/// [`DataTableRowDetail`]. Wire side effects through [`DataTableEvents`] and read table
/// state with [`use_data_table_table_state`].
///
/// # Examples
///
/// ## Custom toolbar, footer, and empty state
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{
///     DataTable, DataTableColumnDef, DataTableEmptyView, DataTableFooterSlot,
///     DataTableRowModel, DataTableToolbarSlot, PagingMode,
/// };
/// use orbital_core_components::{Toolbar, ToolbarButton};
/// let empty: RwSignal<Vec<DataTableRowModel>> = RwSignal::new(vec![]);
/// view! {
///     <div data-testid="data-table-slots-preview">
///         <DataTable
///             paging=PagingMode::None
///             max_height=200.0
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=empty
///         >
///             <DataTableToolbarSlot slot>
///                 <div data-testid="custom-toolbar">
///                     <Toolbar><ToolbarButton>"Custom toolbar"</ToolbarButton></Toolbar>
///                 </div>
///             </DataTableToolbarSlot>
///             <DataTableFooterSlot slot>
///                 <div data-testid="custom-footer">"Custom footer"</div>
///             </DataTableFooterSlot>
///             <DataTableEmptyView slot>
///                 <div data-testid="custom-empty">"No data yet"</div>
///             </DataTableEmptyView>
///         </DataTable>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    preview_slug = "data-table-slots",
    preview_label = "DataTable Slots",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn DataTableSlotsDoc() -> impl IntoView {
    let empty: RwSignal<Vec<DataTableRowModel>> = RwSignal::new(vec![]);

    view! {
        <div data-testid="data-table-slots-preview">
            <DataTable
                paging=PagingMode::None
                max_height=200.0
                columns=vec![DataTableColumnDef::new("name", "Name")]
                items=empty
            >
                <DataTableToolbarSlot slot>
                    <div data-testid="custom-toolbar">
                        <Toolbar>
                            <ToolbarButton>"Custom toolbar"</ToolbarButton>
                        </Toolbar>
                    </div>
                </DataTableToolbarSlot>
                <DataTableFooterSlot slot>
                    <div data-testid="custom-footer">"Custom footer"</div>
                </DataTableFooterSlot>
                <DataTableEmptyView slot>
                    <div data-testid="custom-empty">"No data yet"</div>
                </DataTableEmptyView>
            </DataTable>
        </div>
    }
}
