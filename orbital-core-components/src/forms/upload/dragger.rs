use leptos::prelude::*;
use orbital_base_components::BaseUploadDragger;

/// Styled drop-zone shell rendered inside an [`Upload`](crate::Upload) trigger.
#[component]
pub fn UploadDragger(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <BaseUploadDragger class=class>{children()}</BaseUploadDragger> }
}
