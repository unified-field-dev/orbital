use leptos::prelude::*;
use orbital_base_components::BaseInteractionTagPrimary;

/// Primary action button inside [`crate::InteractionTag`].
#[component]
pub fn InteractionTagPrimary(
    /// Optional CSS class on the primary button.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// When true, layout adjusts for an adjacent [`crate::SecondaryActionTag`].
    #[prop(optional, into)]
    has_secondary_action: Signal<bool>,
    /// Optional leading icon from the icondata catalog.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Visible label for the chip.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseInteractionTagPrimary
            class=class
            has_secondary_action=has_secondary_action
            icon=icon
        >
            {children()}
        </BaseInteractionTagPrimary>
    }
}
