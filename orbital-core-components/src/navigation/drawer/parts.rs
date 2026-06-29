use leptos::prelude::*;
use orbital_base_components::{
    BaseDrawerBody, BaseDrawerHeader, BaseDrawerHeaderTitle, DrawerHeaderTitleAction,
};
use orbital_style::inject_style;

use super::styles::drawer_styles;

static DRAWER_STYLES: std::sync::Once = std::sync::Once::new();

fn ensure_drawer_styles() {
    DRAWER_STYLES.call_once(|| inject_style("orbital-drawer", drawer_styles()));
}

#[component]
pub fn DrawerHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_drawer_styles();
    view! {
        <BaseDrawerHeader class=class>
            {children()}
        </BaseDrawerHeader>
    }
}

#[component]
pub fn DrawerHeaderTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] drawer_header_title_action: Option<DrawerHeaderTitleAction>,
    children: Children,
) -> impl IntoView {
    ensure_drawer_styles();
    view! {
        <BaseDrawerHeaderTitle class=class nostrip:drawer_header_title_action=drawer_header_title_action>
            {children()}
        </BaseDrawerHeaderTitle>
    }
}

#[component]
pub fn DrawerBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_drawer_styles();
    view! {
        <BaseDrawerBody class=class>
            {children()}
        </BaseDrawerBody>
    }
}
