use leptos::prelude::*;
use orbital_base_components::BaseDialogTitle;

/// Dialog heading text.
#[component]
pub fn DialogTitle(
    /// Optional CSS class on the title element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Title text or inline elements.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseDialogTitle class=class>
            {children()}
        </BaseDialogTitle>
    }
}
