use leptos::prelude::*;
use orbital_base_components::BaseToastFooter;

/// Footer actions inside [`Toast`](super::toast::Toast).
#[component]
pub fn ToastFooter(
    /// Optional CSS class on the footer element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <BaseToastFooter class=class>
            {children()}
        </BaseToastFooter>
    }
}
