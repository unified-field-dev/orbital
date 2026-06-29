use leptos::prelude::*;

use crate::{BadgeAppearance, BadgeColor, BadgeSize, BaseBadge};

#[component]
pub fn BaseCounterBadge(
    #[prop(optional, into)] class: MaybeProp<String>,
    count: u32,
    #[prop(default = 99)] overflow_count: u32,
    #[prop(optional, into)] appearance: Signal<BadgeAppearance>,
    #[prop(optional, into)] color: Signal<BadgeColor>,
    #[prop(optional, into)] size: Signal<BadgeSize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let display = move || {
        if count > overflow_count {
            format!("{overflow_count}+")
        } else {
            count.to_string()
        }
    };

    view! {
        <span
            class=move || {
                let mut parts = vec!["orbital-counter-badge".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children.map(|c| c())}
            <span class="orbital-counter-badge__pill">
                <BaseBadge appearance=appearance color=color size=size>
                    {display}
                </BaseBadge>
            </span>
        </span>
    }
}
