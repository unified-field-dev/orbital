use std::collections::HashSet;

use leptos::prelude::*;

use crate::types::{DataTableFilter, DataTableSort, DataTableTableState, PaginationState};

/// Sync controlled model props with internal table signals.
pub fn wire_controlled_models(
    table_state: DataTableTableState,
    sort: Signal<Option<DataTableSort>>,
    filter: Signal<Option<DataTableFilter>>,
    pagination: Signal<Option<PaginationState>>,
    selection: Signal<Option<HashSet<String>>>,
) {
    Effect::new(move |_| {
        if let Some(external) = sort.get() {
            if table_state.sort.get_untracked() != external {
                table_state.sort.set(external);
                if !table_state.is_server() {
                    table_state.recompute_client_processed();
                }
                table_state.bump_render();
            }
        }
    });

    Effect::new(move |_| {
        if let Some(external) = filter.get() {
            if table_state.filter.get_untracked() != external {
                table_state.filter.set(external);
                table_state.reset_pagination();
                if !table_state.is_server() {
                    table_state.recompute_client_processed();
                }
                table_state.bump_render();
            }
        }
    });

    Effect::new(move |_| {
        if let Some(external) = pagination.get() {
            let current = table_state.current_pagination();
            if current != external {
                table_state.page.set(external.page);
                table_state.page_size.set(external.page_size as usize);
                if table_state.is_server() {
                    let size = external.page_size.max(1);
                    table_state
                        .server_offset
                        .set((external.page as u32).saturating_mul(size));
                }
                if !table_state.is_server() {
                    table_state.recompute_client_processed();
                }
                table_state.bump_render();
            }
        }
    });

    Effect::new(move |_| {
        if let Some(external) = selection.get() {
            if table_state.selected.get_untracked() != external {
                table_state.selected.set(external.clone());
                table_state
                    .selection_anchor
                    .set(external.iter().next().cloned());
                table_state.bump_render();
            }
        }
    });
}
