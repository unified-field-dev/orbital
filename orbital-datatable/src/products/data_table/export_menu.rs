use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, Menu, MenuItem, MenuTrigger};

use crate::engine::ExportRowScope;
use crate::io::{download_bytes, print_html};
use crate::types::DataTableTableState;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExportAction {
    Csv,
    Print,
    Excel,
}

/// Toolbar export menu (CSV, print, optional Excel).
#[component]
pub fn DataTableExportMenu(state: DataTableTableState) -> impl IntoView {
    let excel_enabled = state.excel_export_enabled();

    let on_select = Callback::new(move |action: ExportAction| {
        let scope = ExportRowScope::AllMatching;
        match action {
            ExportAction::Csv => {
                let csv = state.export_csv(scope);
                download_bytes("export.csv", csv.as_bytes(), "text/csv;charset=utf-8");
            }
            ExportAction::Print => {
                let html = state.export_print_html(scope);
                print_html(&html);
            }
            ExportAction::Excel => {
                if let Ok(bytes) = state.export_xlsx(scope) {
                    download_bytes(
                        "export.xlsx",
                        &bytes,
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    );
                }
            }
        }
    });

    view! {
        <Menu on_select=move |action: ExportAction| on_select.run(action)>
            <MenuTrigger slot>
                <Button
                    appearance=ButtonAppearance::Subtle
                    attr:data-testid="data-table-export-menu"
                >
                    "Export"
                </Button>
            </MenuTrigger>
            <MenuItem value=ExportAction::Csv>"Download CSV"</MenuItem>
            <MenuItem value=ExportAction::Print>"Print"</MenuItem>
            <Show when=move || excel_enabled>
                <MenuItem value=ExportAction::Excel>"Download Excel"</MenuItem>
            </Show>
        </Menu>
    }
}
