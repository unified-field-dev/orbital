use leptos::prelude::*;

use crate::overlay::OverlayDismiss;
use crate::Handler;

#[derive(Clone)]
pub struct MenuInjection<V> {
    pub has_icon: RwSignal<bool>,
    pub on_select: Handler<V>,
}

impl<V: Clone + Send + Sync + 'static> MenuInjection<V> {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }
}

#[component]
pub fn BaseMenuItem<V>(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    value: V,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView
where
    V: Clone + Send + Sync + 'static,
{
    let MenuInjection {
        has_icon,
        on_select,
    } = MenuInjection::expect_context();

    if icon.with_untracked(|i| i.is_some()) {
        has_icon.set(true);
    }

    let on_click = move |_| {
        if disabled.get_untracked() {
            return;
        }
        on_select.run(value.clone());
        if let Some(dismiss) = use_context::<OverlayDismiss>() {
            dismiss.close.run(());
        }
    };

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-menu-item".to_string()];
                if disabled.get() {
                    parts.push("orbital-menu-item--disabled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="menuitem"
            tabindex="-1"
            aria-disabled=move || disabled.get().then_some("true")
            on:click=on_click
        >
            {move || icon.get().map(|_| view! { <span class="orbital-menu-item__icon" aria-hidden="true"></span> })}
            <span class="orbital-menu-item__label">{children()}</span>
        </div>
    }
}
