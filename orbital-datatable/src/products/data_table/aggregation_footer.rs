use leptos::prelude::*;
use orbital_core_components::{TableCell, TableCellLayout};

use crate::engine::format_display;
use crate::types::DataTableTableState;

/// Footer aggregate summary row rendered below the table body (inline, not a nested component).
pub fn aggregation_footer_row_view(state: DataTableTableState) -> AnyView {
    view! {
        {move || {
            let Some(footer) = state.footer_row.get() else {
                return view! {
                    <tr class="orbital-data-table__aggregation-footer" style="display: none;">
                        <td></td>
                    </tr>
                }
                .into_any();
            };
            let cols = state.column_layout.get().columns;
            view! {
                <tr
                    class="orbital-data-table__aggregation-footer"
                    data-testid="data-table-aggregation-footer"
                    data-table-role="row"
                >
                    {cols
                        .iter()
                        .map(|col| {
                            let value = footer.get(&col.def.field);
                            let text = value
                                .map(|v| format_display(&col.def, v))
                                .unwrap_or_default();
                            view! {
                                <TableCell>
                                    <TableCellLayout>{text}</TableCellLayout>
                                </TableCell>
                            }
                        })
                        .collect_view()}
                </tr>
            }
            .into_any()
        }}
    }
    .into_any()
}
