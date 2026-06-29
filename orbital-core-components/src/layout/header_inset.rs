use leptos::prelude::*;
use orbital_base_components::BaseLayoutHeaderInset;
use orbital_macros::component_doc;

/// Scrollable spacer matching overlay app bar height.
///
/// Rendered automatically by [`LayoutMain`](crate::LayoutMain) when a Fixed or Sticky [`AppBar`](crate::AppBar) provides [`AppBarInset`](orbital_base_components::AppBarInset).
#[component_doc(category = "Shell", preview = "manual")]
#[component]
pub fn LayoutHeaderInset(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = 56)] height_px: u16,
) -> impl IntoView {
    view! {
        <BaseLayoutHeaderInset class=class height_px=height_px />
    }
}
