use leptos::prelude::*;
use orbital_core_components::{
    Button, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface, DialogTitle,
};

use crate::types::DataTableTableState;

/// Modal shown when `on_row_update` rejects a commit.
#[component]
pub fn DataTableEditErrorDialog(state: DataTableTableState) -> impl IntoView {
    let open = RwSignal::new(false);

    Effect::new(move || {
        open.set(state.edit_error_dialog.get().is_some());
    });

    view! {
        <Dialog open=open>
            <DialogSurface attr:aria-label="Edit rejected">
                <DialogBody>
                    <div data-testid="data-table-edit-error-dialog">
                        <DialogTitle>"Edit rejected"</DialogTitle>
                        <DialogContent>
                            {move || state.edit_error_dialog.get().unwrap_or_default()}
                        </DialogContent>
                        <DialogActions>
                            <Button on:click=move |_| {
                                state.edit_error_dialog.set(None);
                                open.set(false);
                            }>
                                "Close"
                            </Button>
                        </DialogActions>
                    </div>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
