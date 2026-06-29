use leptos::prelude::*;
use orbital_core_components::{TableHeader, TableRow};

use super::header_cell::DataTableHeaderCell;
use super::header_filters::DataTableHeaderFilterRow;
use super::header_groups::{build_group_header_row, DataTableGroupHeaderRow};
use super::leading_columns::{leading_column_layout, leading_header_cells};
use crate::core::use_data_table_context;
use crate::types::{DataTableFeatures, DataTableTableState};

/// Header row(s) with optional selection column, groups, and sortable headers.
#[component]
pub fn DataTableHeader(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    let layout = Memo::new(move |_| leading_column_layout(state, ctx));

    view! {
        <TableHeader>
            {move || {
                build_group_header_row(state).map(|cells| {
                    view! {
                        <DataTableGroupHeaderRow
                            cells=cells
                            leading_count=layout.get().count()
                        />
                    }
                })
            }}
            <TableRow>
                {move || {
                    let layout = layout.get();
                    let rowspan = build_group_header_row(state).is_some().then_some(2u32);
                    leading_header_cells(layout, rowspan)
                }}
                <For
                    each=move || {
                        let _ = state.render_key.get();
                        state.column_layout.get().columns
                    }
                    key=|col| format!("{}:{:.0}", col.def.field, col.width_px)
                    children=move |col| {
                        view! { <DataTableHeaderCell state=state resolved=col /> }
                    }
                />
            </TableRow>
            {move || {
                state
                    .features
                    .contains(DataTableFeatures::HEADER_FILTERS)
                    .then(|| view! { <DataTableHeaderFilterRow state=state /> })
            }}
        </TableHeader>
    }
}
