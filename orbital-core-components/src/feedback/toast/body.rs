use leptos::prelude::*;
use orbital_base_components::BaseToastBody;

/// Primary message copy inside [`Toast`](super::toast::Toast).
#[component]
pub fn ToastBody(
    /// Optional CSS class on the body element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <BaseToastBody class=class>
            {children()}
        </BaseToastBody>
    }
}
