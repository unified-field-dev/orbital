use leptos::{ev, prelude::*};

use crate::{icon::BaseIcon, Handler};

const DISMISS_ICON_PATH: &str = "m4.09 4.22.06-.07a.5.5 0 0 1 .63-.06l.07.06L10 9.29l5.15-5.14a.5.5 0 0 1 .63-.06l.07.06c.18.17.2.44.06.63l-.06.07L10.71 10l5.14 5.15c.18.17.2.44.06.63l-.06.07a.5.5 0 0 1-.63.06l-.07-.06L10 10.71l-5.15 5.14a.5.5 0 0 1-.63.06l-.07-.06a.5.5 0 0 1-.06-.63l.06-.07L9.29 10 4.15 4.85a.5.5 0 0 1-.06-.63l.06-.07-.06.07Z";

#[component]
pub fn BaseTagDismissIcon() -> impl IntoView {
    view! {
        <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
            <path d=DISMISS_ICON_PATH fill="currentColor" />
        </svg>
    }
}

#[component]
pub fn BaseTagDismissButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(optional, into)] on_click: Option<Handler<ev::MouseEvent>>,
) -> impl IntoView {
    view! {
        <button
            class=move || class.get().unwrap_or_else(|| "orbital-tag__dismiss".to_string())
            id=id
            aria-label=move || aria_label.clone()
            aria-labelledby=move || aria_labelledby.clone()
            on:click=move |event| {
                if let Some(on_click) = on_click.as_ref() {
                    on_click.run(event);
                }
            }
        >
            {move || {
                if let Some(icon) = icon.get() {
                    view! { <BaseIcon icon=icon width="1em" height="1em" /> }.into_any()
                } else {
                    view! { <BaseTagDismissIcon /> }.into_any()
                }
            }}
        </button>
    }
}
