use leptos::prelude::*;
use orbital_base_components::BaseDialogBody;

/// Layout wrapper grouping title, content, and actions inside [`DialogSurface`](super::surface::DialogSurface).
#[component]
pub fn DialogBody(
    /// Optional CSS class on the body wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Typically [`DialogTitle`](super::title::DialogTitle), [`DialogContent`](super::content::DialogContent), and [`DialogActions`](super::actions::DialogActions).
    children: Children,
) -> impl IntoView {
    view! {
        <BaseDialogBody class=class>
            {children()}
        </BaseDialogBody>
    }
}
