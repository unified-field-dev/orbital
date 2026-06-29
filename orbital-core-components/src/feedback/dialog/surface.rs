use leptos::prelude::*;
use orbital_base_components::BaseDialogSurface;

/// Animated modal panel (`role="dialog"`, `aria-modal="true"`).
///
/// Required child of [`Dialog`](super::dialog::Dialog). Wraps [`DialogBody`] and its title/content/actions slots.
#[component]
pub fn DialogSurface(
    /// Optional CSS class on the surface panel.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Panel contents — typically [`DialogBody`].
    children: Children,
) -> impl IntoView {
    view! {
        <BaseDialogSurface class=class>
            {children()}
        </BaseDialogSurface>
    }
}
