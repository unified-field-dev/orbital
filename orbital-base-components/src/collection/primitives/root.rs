use leptos::{context::Provider, prelude::*};

use crate::collection::state::{CollectionState, CollectionStateInjection};

#[component]
pub fn BaseCollectionRoot(
    #[prop(into)] class: Signal<String>,
    #[prop(optional, default = "tree")] role: &'static str,
    #[prop(into)] collection_state: CollectionState,
    #[prop(optional, default = "orbital-collection")] base_class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec![base_class.to_string()];
                let extra = class.get();
                if !extra.is_empty() {
                    parts.push(extra);
                }
                parts.join(" ")
            }
            role=role
        >
            <Provider value=CollectionStateInjection(collection_state)>{children()}</Provider>
        </div>
    }
}
