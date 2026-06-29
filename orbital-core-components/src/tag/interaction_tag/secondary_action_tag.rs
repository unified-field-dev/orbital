use leptos::{ev, prelude::*};
use orbital_base_components::{BaseSecondaryActionTag, Handler};

/// Secondary action button inside [`crate::InteractionTag`].
///
/// Typically used for dismiss/remove while [`InteractionTagPrimary`] handles the main click.
#[component]
pub fn SecondaryActionTag(
    /// Accessible name fragment combined with the primary label (e.g. `"Remove"`).
    #[prop(into)]
    aria_label: String,
    /// Optional CSS class on the secondary button.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional custom icon; defaults to dismiss X.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Callback when the secondary button is clicked.
    #[prop(optional, into)]
    on_click: Option<Handler<ev::MouseEvent>>,
) -> impl IntoView {
    view! {
        <BaseSecondaryActionTag
            aria_label=aria_label
            class=class
            icon=icon
            nostrip:on_click=on_click
        />
    }
}
