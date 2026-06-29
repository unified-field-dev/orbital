use leptos::prelude::*;
use orbital_base_components::BaseMessageBarActions;

/// Footer slot for action buttons inside [`MessageBar`](super::message_bar::MessageBar).
#[component]
pub fn MessageBarActions(
    /// Optional CSS class on the actions footer.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Action buttons (typically small [`crate::Button`] children).
    children: Children,
) -> impl IntoView {
    view! {
        <BaseMessageBarActions class=class>
            {children()}
        </BaseMessageBarActions>
    }
}
