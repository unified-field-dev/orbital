use leptos::prelude::*;
use orbital_base_components::{BaseBreadcrumbButton, BaseBreadcrumbDivider, BaseBreadcrumbItem};
use orbital_style::inject_style;

use super::styles::breadcrumb_styles;

static BREADCRUMB_STYLES: std::sync::Once = std::sync::Once::new();

fn ensure_breadcrumb_styles() {
    BREADCRUMB_STYLES.call_once(|| inject_style("orbital-breadcrumb", breadcrumb_styles()));
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_breadcrumb_styles();
    view! {
        <BaseBreadcrumbItem class=class>
            {children()}
        </BaseBreadcrumbItem>
    }
}

#[component]
pub fn BreadcrumbButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    ensure_breadcrumb_styles();
    view! {
        <BaseBreadcrumbButton class=class current=current>
            {children()}
        </BaseBreadcrumbButton>
    }
}

#[component]
pub fn BreadcrumbDivider(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    ensure_breadcrumb_styles();
    view! { <BaseBreadcrumbDivider class=class /> }
}
