use leptos::prelude::*;
use orbital_base_components::BaseLayoutSidebar;

/// Side navigation column for [`Layout`](crate::Layout).
#[component]
pub fn LayoutSidebarShell(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <BaseLayoutSidebar class=class>
            {children()}
        </BaseLayoutSidebar>
    }
}
