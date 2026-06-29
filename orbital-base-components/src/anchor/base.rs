use leptos::{context::Provider, html, prelude::*};

use super::{injection::AnchorInjection, scroll_spy::mount_anchor_scroll_spy, OffsetTarget};

/// Headless in-page anchor rail with scroll-spy active link tracking.
#[component]
pub fn BaseAnchor(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] offset_target: Option<OffsetTarget>,
    children: Children,
) -> impl IntoView {
    let anchor_ref = NodeRef::<html::Div>::new();
    let bar_ref = NodeRef::<html::Div>::new();
    let element_ids = RwSignal::new(Vec::<String>::new());
    let active_id = RwSignal::new(None::<String>);

    mount_anchor_scroll_spy(element_ids, active_id, offset_target);

    let root_class = move || {
        let mut parts = vec!["orbital-anchor".to_string()];
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    };

    let bar_class = move || {
        let mut parts = vec!["orbital-anchor-rail__bar".to_string()];
        if active_id.with(|id| id.is_some()) {
            parts.push("orbital-anchor-rail__bar--active".to_string());
        }
        parts.join(" ")
    };

    view! {
        <div class=root_class node_ref=anchor_ref>
            <div class="orbital-anchor-rail">
                <div class=bar_class node_ref=bar_ref></div>
            </div>
            <Provider value=AnchorInjection::new(anchor_ref, bar_ref, element_ids, active_id)>
                {children()}
            </Provider>
        </div>
    }
}
