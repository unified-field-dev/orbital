use leptos::prelude::*;
use orbital_base_components::BaseMessageBarTitle;

/// Bold heading line inside [`MessageBar`](super::message_bar::MessageBar).
#[component]
pub fn MessageBarTitle(
    /// Optional CSS class on the title element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Title text or inline elements.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseMessageBarTitle class=class>
            {children()}
        </BaseMessageBarTitle>
    }
}
