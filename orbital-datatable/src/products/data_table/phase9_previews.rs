use leptos::prelude::*;
use orbital_core_components::ButtonGroup;

/// Shared wrapper for preview demo controls above a [`DataTable`].
#[component]
pub fn DataTablePreviewControls(children: Children) -> impl IntoView {
    view! {
        <div class="orbital-data-table__preview-controls">
            <ButtonGroup>{children()}</ButtonGroup>
        </div>
    }
}
