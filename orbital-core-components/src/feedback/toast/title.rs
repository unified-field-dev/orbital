use leptos::prelude::*;
use orbital_base_components::BaseToastTitle;

/// Bold title line inside [`Toast`](super::toast::Toast).
#[component]
pub fn ToastTitle(
    /// Optional CSS class on the title row.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Title text or inline elements.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseToastTitle class=class>
            {children()}
        </BaseToastTitle>
    }
}
