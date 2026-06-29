use leptos::prelude::*;
use orbital_base_components::{BaseInlineDrawer, DrawerPosition, DrawerSize, OpenBind};
use orbital_style::inject_style;

use super::styles::drawer_styles;

#[component]
pub fn InlineDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] open: OpenBind,
    #[prop(default = DrawerPosition::Left.into(), into)] position: Signal<DrawerPosition>,
    #[prop(default = DrawerSize::Small.into(), into)] size: Signal<DrawerSize>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-drawer", drawer_styles());

    view! {
        <BaseInlineDrawer class=class open=open position=position size=size>
            {children()}
        </BaseInlineDrawer>
    }
}
