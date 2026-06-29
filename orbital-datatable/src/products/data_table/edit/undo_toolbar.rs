use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, Toolbar};

use crate::types::DataTableTableState;

/// Undo/redo toolbar for previews and apps with [`DataTableFeatures::UNDO_REDO`].
#[component]
pub fn DataTableEditUndoToolbar(state: DataTableTableState) -> impl IntoView {
    let undo_disabled = Signal::derive(move || !state.can_undo());
    let redo_disabled = Signal::derive(move || !state.can_redo());

    view! {
        <Toolbar class="orbital-data-table__edit-toolbar">
            <Button
                appearance=ButtonAppearance::Subtle
                attr:data-testid="data-table-undo"
                disabled=undo_disabled
                on:click=move |_| state.undo_edit()
            >
                "Undo"
            </Button>
            <Button
                appearance=ButtonAppearance::Subtle
                attr:data-testid="data-table-redo"
                disabled=redo_disabled
                on:click=move |_| state.redo_edit()
            >
                "Redo"
            </Button>
        </Toolbar>
    }
}
