use leptos::prelude::*;
use orbital_base_components::BaseAnchorLink;

/// Anchor link item registered with the parent [`Anchor`](crate::Anchor) rail.
#[component]
pub fn AnchorLink(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Visible link label.
    #[prop(into)]
    title: Signal<String>,
    /// Target href (`#section-id` for in-page anchors).
    #[prop(into)]
    href: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <BaseAnchorLink class=class title=title href=href>
            {children.map(|children| children())}
        </BaseAnchorLink>
    }
}
