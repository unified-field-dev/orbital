use leptos::prelude::*;

#[component]
pub fn BaseDivider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] vertical: Signal<bool>,
    #[prop(optional, into)] labeled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-divider".to_string()];
                if vertical.get() {
                    parts.push("orbital-divider--vertical".to_string());
                }
                if !labeled.get() {
                    parts.push("orbital-divider--unlabeled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            aria-orientation=move || if vertical.get() { "vertical" } else { "horizontal" }
            role="separator"
        >
            {children.map(|c| view! {
                <div class="orbital-divider__wrapper">{c()}</div>
            })}
        </div>
    }
}
