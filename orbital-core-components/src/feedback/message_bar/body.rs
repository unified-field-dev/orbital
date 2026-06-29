use leptos::prelude::*;
use orbital_base_components::BaseMessageBarBody;

/// Primary message copy inside [`MessageBar`](super::message_bar::MessageBar).
#[component]
pub fn MessageBarBody(
    /// Optional CSS class on the body element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Message text or inline elements.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseMessageBarBody class=class>
            {children()}
        </BaseMessageBarBody>
    }
}
