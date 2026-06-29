use leptos::prelude::*;

use crate::overlay::FeedbackIntent;

use super::intent_icon::message_bar_intent_icon;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MessageBarLayout {
    #[default]
    Singleline,
    Multiline,
}

impl MessageBarLayout {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Singleline => "singleline",
            Self::Multiline => "multiline",
        }
    }
}

#[component]
pub fn BaseMessageBar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = MessageBarLayout::Singleline)] layout: MessageBarLayout,
    #[prop(optional, into)] intent: Signal<FeedbackIntent>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-message-bar".to_string(),
                    format!("orbital-message-bar--{}", intent.get().as_str()),
                    format!("orbital-message-bar--{}", layout.as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="group"
        >
            <div class="orbital-message-bar__icon">
                {move || message_bar_intent_icon(intent.get())}
            </div>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseMessageBarTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-message-bar-title", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseMessageBarBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-message-bar-body", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseMessageBarActions(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-message-bar-actions", class.get())>
            {children()}
        </div>
    }
}

fn format_class(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    }
}
