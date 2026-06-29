use leptos::prelude::*;
use orbital_base_components::BaseToastTrigger;

/// Wraps a single interactive child; clicking dismisses the enclosing toast.
#[component]
pub fn ToastTrigger(children: Children) -> impl IntoView {
    view! {
        <BaseToastTrigger>
            {children()}
        </BaseToastTrigger>
    }
}
