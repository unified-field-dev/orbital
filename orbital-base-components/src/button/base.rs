use leptos::{ev, html, prelude::*};

use super::r#ref::ButtonRef;
use super::types::ButtonType;
use crate::ComponentRef;

/// Headless native `<button>` — semantics and a11y only; styling via `class` and modifier strings.
#[component(transparent)]
pub fn BaseButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] appearance: MaybeProp<String>,
    #[prop(optional, into)] shape: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] button_type: MaybeProp<ButtonType>,
    #[prop(optional, into)] block: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] disabled_focusable: Signal<bool>,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] aria_pressed: MaybeProp<String>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
    #[prop(optional)] comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView {
    let _ = block;
    let btn_disabled = Memo::new(move |_| disabled.get());
    let aria_disabled = move || {
        if loading.get() || disabled_focusable.get() {
            Some("true")
        } else {
            None
        }
    };
    let aria_busy = move || loading.get().then_some("true");

    let button_ref = NodeRef::<html::Button>::new();
    comp_ref.load(ButtonRef { button_ref });

    let on_click = move |e: ev::MouseEvent| {
        if btn_disabled.get_untracked() || loading.get_untracked() {
            return;
        }
        let Some(on_click) = on_click else {
            return;
        };
        on_click.run(e);
    };

    view! {
        <button
            class=move || {
                let mut parts = Vec::new();
                if let Some(c) = class.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = appearance.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = shape.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = size.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                parts.join(" ")
            }
            type=move || button_type.get().map(|t| t.as_str())
            disabled=move || {
                if disabled_focusable.get() {
                    None
                } else {
                    disabled.get().then_some("")
                }
            }
            tabindex=move || disabled_focusable.get().then_some(0)
            aria-disabled=aria_disabled
            aria-busy=aria_busy
            aria-pressed=move || aria_pressed.get()
            on:click=on_click
            node_ref=button_ref
        >
            {children()}
        </button>
    }
}
