use leptos::prelude::*;
use orbital_base_components::{AppBarInset, BaseLayoutMain};

use super::overlay::LayoutOverlayScroll;
use crate::ScrollArea;

/// Primary scrollable content column for [`Layout`](crate::Layout).
#[component]
pub fn LayoutMainShell(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let inset = use_context::<AppBarInset>();
    let overlay = use_context::<LayoutOverlayScroll>();

    let use_inset_scrollport =
        matches!((inset, overlay), (Some(_), Some(mode)) if mode.main_inset_scroll);

    if use_inset_scrollport {
        view! {
            <BaseLayoutMain class=class>
                <div
                    class="orbital-layout__main-chrome"
                    role="presentation"
                    aria-hidden="true"
                ></div>
                <ScrollArea class="orbital-layout__main-scroll">
                    {children()}
                </ScrollArea>
            </BaseLayoutMain>
        }
        .into_any()
    } else if matches!((inset, overlay), (Some(_), Some(_))) {
        view! {
            <BaseLayoutMain class=class>
                {children()}
            </BaseLayoutMain>
        }
        .into_any()
    } else {
        view! {
            <BaseLayoutMain class=class>
                <ScrollArea class="orbital-layout__main-scroll">
                    {children()}
                </ScrollArea>
            </BaseLayoutMain>
        }
        .into_any()
    }
}
