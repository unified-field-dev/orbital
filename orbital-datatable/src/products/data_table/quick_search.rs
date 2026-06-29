use leptos::prelude::*;
use orbital_core_components::{SearchBox, SearchBoxAppearance};

use crate::core::use_data_table_context;
use crate::types::DataTableTableState;

/// Quick search bound to table filter state (client and server sources).
#[component]
pub fn DataTableQuickSearch(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    let placeholder = ctx.locale.get_value().quick_search_placeholder.clone();

    view! {
        <div data-testid="data-table-quick-search">
            <SearchBox
                bind=state.quick_search
                appearance=SearchBoxAppearance::with_placeholder(placeholder)
            />
        </div>
    }
}
