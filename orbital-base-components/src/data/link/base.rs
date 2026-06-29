use leptos::{either::EitherOf3, prelude::*};

#[component]
pub fn BaseLink(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] span: bool,
    #[prop(optional, into)] inline: Signal<bool>,
    #[prop(optional)] href: Option<Signal<String>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] disabled_focusable: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let link_disabled = Memo::new(move |_| disabled.get() || disabled_focusable.get());
    let merged_class = move || {
        let mut parts = vec!["orbital-link".to_string()];
        if inline.get() {
            parts.push("orbital-link--inline".to_string());
        }
        if link_disabled.get() {
            parts.push("orbital-link--disabled".to_string());
            parts.push("orbital-link--disabled-focusable".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    let tabindex = Memo::new(move |_| {
        if disabled_focusable.get() {
            Some("0")
        } else if disabled.get() {
            Some("-1")
        } else {
            None
        }
    });

    if let Some(href) = href {
        EitherOf3::A(view! {
            <a
                role="link"
                class=merged_class
                href=href
                tabindex=tabindex
                aria-disabled=move || link_disabled.get().then_some("true")
            >
                {children()}
            </a>
        })
    } else if span {
        EitherOf3::B(view! { <span class=merged_class>{children()}</span> })
    } else {
        EitherOf3::C(view! {
            <button
                class=merged_class
                disabled=move || disabled.get().then_some("")
                aria-disabled=move || link_disabled.get().then_some("true")
            >
                {children()}
            </button>
        })
    }
}
