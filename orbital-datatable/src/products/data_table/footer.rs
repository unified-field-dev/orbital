use leptos::prelude::*;
use orbital_core_components::{Field, Select, SelectSize};

use super::pagination_bar::DataTablePaginationBar;
use crate::core::{use_data_table_context, DataTableTableStateProvider};
use crate::types::{DataTableFooterSlot, DataTableTableState, PaginationDisplayFormat};

fn effective_page_size_options(options: &Option<Vec<u32>>, current: usize) -> Vec<u32> {
    let Some(opts) = options else {
        return Vec::new();
    };
    if opts.is_empty() {
        return Vec::new();
    }
    let current_u32 = current as u32;
    if opts.contains(&current_u32) {
        return opts.clone();
    }
    let mut merged = opts.clone();
    merged.push(current_u32);
    merged.sort_unstable();
    merged.dedup();
    merged
}

/// Footer page-size selector synced with [`DataTableTableState::page_size`].
#[component]
fn DataTablePageSizeSelect(state: DataTableTableState, options: Vec<u32>) -> impl IntoView {
    let selected_size = RwSignal::new(state.page_size.get().to_string());

    Effect::new(move |_| {
        selected_size.set(state.page_size.get().to_string());
    });

    Effect::new(move |_| {
        let parsed = selected_size.get().parse::<u32>().unwrap_or(1).max(1) as usize;
        if state.page_size.get() != parsed {
            state.page_size.set(parsed);
            state.reset_pagination();
            if !state.is_server() {
                state.recompute_client_processed();
            }
            state.notify_pagination();
        }
    });

    view! {
        <Field label="Rows per page">
            <Select
                bind=selected_size
                attr:data-testid="data-table-page-size"
                appearance=SelectSize::Small
            >
                {options
                    .into_iter()
                    .map(|size| {
                        view! {
                            <option value=size.to_string()>{size.to_string()}</option>
                        }
                    })
                    .collect_view()}
            </Select>
        </Field>
    }
}

/// Footer with row count and pagination bar.
#[component]
pub fn DataTableFooter(
    state: DataTableTableState,
    #[prop(optional)] footer_slot: Option<DataTableFooterSlot>,
) -> impl IntoView {
    if let Some(slot) = footer_slot {
        return view! {
            <DataTableTableStateProvider state=state>
                {(slot.children)()}
            </DataTableTableStateProvider>
        }
        .into_any();
    }

    let ctx = use_data_table_context();

    view! {
        {move || {
            let locale = ctx.locale.get_value();
            let pagination_display = ctx.pagination_display.get_value();
            let page_size_options = ctx.page_size_options.get_value();
            let use_range =
                state.show_pagination() && pagination_display == PaginationDisplayFormat::Locale;
            let (from, to, total, estimated) = state.pagination_range_bounds();
            let label = if use_range {
                locale.format_pagination_range(from, to, total, estimated)
            } else {
                locale.format_footer_rows(state.total_rows.get())
            };
            let size_options = effective_page_size_options(&page_size_options, state.page_size.get());
            let show_page_size = state.show_pagination() && !size_options.is_empty();

            view! {
                <div class="orbital-data-table__footer" data-testid="data-table-footer">
                    <div class="orbital-data-table__footer-start">
                        <span aria-live="polite" data-testid="data-table-pagination-range">
                            {label}
                        </span>
                        <Show when=move || show_page_size>
                            <DataTablePageSizeSelect state=state options=size_options.clone() />
                        </Show>
                    </div>
                    <Show when=move || state.show_pagination()>
                        <div class="orbital-data-table__footer-end">
                            <DataTablePaginationBar state=state />
                        </div>
                    </Show>
                </div>
            }
            .into_any()
        }}
    }
    .into_any()
}
