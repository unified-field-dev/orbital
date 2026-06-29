use leptos::prelude::*;

use super::BaseButton;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CompoundButtonIconPosition {
    #[default]
    Before,
    After,
}

#[component]
pub fn BaseCompoundButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] appearance: MaybeProp<String>,
    #[prop(optional, into)] shape: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] disabled_focusable: Signal<bool>,
    #[prop(optional, into)] secondary_content: MaybeProp<String>,
    #[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    #[prop(optional)] icon_before: Option<Children>,
    #[prop(optional)] icon_after: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <BaseButton
            class=class
            appearance=appearance
            shape=shape
            size=size
            disabled=disabled
            disabled_focusable=disabled_focusable
            nostrip:on_click=on_click
        >
            <span class="orbital-compound-button__content">
                {icon_before.map(|c| c())}
                <span class="orbital-compound-button__text">
                    <span class="orbital-compound-button__primary">{children()}</span>
                    {move || secondary_content.get().map(|text| view! {
                        <span class="orbital-compound-button__secondary">{text}</span>
                    })}
                </span>
                {icon_after.map(|c| c())}
            </span>
        </BaseButton>
    }
}
