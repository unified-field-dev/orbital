use std::rc::Rc;

use leptos::{ev, prelude::*};

/// Headless primary clickable card region — renders a `button` or anchor when `href` is set.
#[component]
pub fn BaseCardButtonArea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let use_link = href.get_untracked().map(|h| !h.is_empty()).unwrap_or(false);
    let child_view = children();
    let href_value = href.get_untracked().unwrap_or_default();
    let on_click = Rc::new(on_click);

    if use_link {
        let on_click = on_click.clone();
        view! {
            <a
                class=move || class.get().unwrap_or_default()
                href=href_value.clone()
                aria-disabled=move || disabled.get().then_some("true")
                on:click=move |e: ev::MouseEvent| {
                    if disabled.get_untracked() {
                        e.prevent_default();
                        return;
                    }
                    if let Some(handler) = on_click.as_ref() {
                        handler.run(e);
                    }
                }
            >
                {child_view}
            </a>
        }
        .into_any()
    } else {
        view! {
            <button
                type="button"
                class=move || class.get().unwrap_or_default()
                disabled=move || disabled.get().then_some("")
                aria-disabled=move || disabled.get().then_some("true")
                on:click=move |e: ev::MouseEvent| {
                    if disabled.get_untracked() {
                        e.prevent_default();
                        return;
                    }
                    if let Some(handler) = on_click.as_ref() {
                        handler.run(e);
                    }
                }
            >
                {child_view}
            </button>
        }
        .into_any()
    }
}
