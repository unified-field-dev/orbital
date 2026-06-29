use leptos::prelude::*;

use crate::form::types::{LabelSize, LabelWeight};

/// Headless native `<label>` — `for`, required marker, and modifier classes only.
#[component(transparent)]
pub fn BaseLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] weight: MaybeProp<String>,
    #[prop(optional, into)] label_size: Signal<LabelSize>,
    #[prop(optional, into)] label_weight: Signal<LabelWeight>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] attr_for: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let _ = (label_size, label_weight, disabled);
    view! {
        <label
            class=move || {
                let mut parts = Vec::new();
                if let Some(c) = class.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = size.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = weight.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                parts.join(" ")
            }
            for=move || attr_for.get()
        >
            {children()}
            {move || required.get().then(|| view! { <span aria-hidden="true">"*"</span> })}
        </label>
    }
}
