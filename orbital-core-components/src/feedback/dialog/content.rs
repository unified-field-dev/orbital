use leptos::prelude::*;
use orbital_base_components::BaseDialogContent;

/// Main content region inside [`DialogSurface`](super::surface::DialogSurface).
#[component]
pub fn DialogContent(
    /// Optional CSS class on the content block.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Body copy, form fields, or other primary dialog content.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseDialogContent class=class>
            {children()}
        </BaseDialogContent>
    }
}
