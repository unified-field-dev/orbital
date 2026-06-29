use leptos::{context::Provider, prelude::*};

use super::{TagAppearance, TagGroupInjection, TagSize};
use crate::Handler;

#[component]
pub fn BaseTagGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: Signal<TagSize>,
    #[prop(optional, into)] appearance: Signal<TagAppearance>,
    #[prop(optional, into)] on_dismiss: Option<Handler<String>>,
    #[prop(optional, into)] dismissible: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-tag-group".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            <Provider value=TagGroupInjection {
                size,
                appearance,
                on_dismiss,
                dismissible,
            }>{children()}</Provider>
        </div>
    }
}
