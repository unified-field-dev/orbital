use leptos::prelude::*;
use orbital_base_components::BaseDialogActions;

/// Footer row for primary and secondary actions (typically [`crate::Button`]).
#[component]
pub fn DialogActions(
    /// Optional CSS class on the actions footer.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Action buttons; use `on_click` to set `open` to `false` when dismissing.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseDialogActions class=class>
            {children()}
        </BaseDialogActions>
    }
}
