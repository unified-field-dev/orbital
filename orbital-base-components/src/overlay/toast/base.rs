use leptos::prelude::*;

use super::super::feedback_intent::FeedbackIntent;

#[component]
pub fn BaseToast(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] intent: Signal<FeedbackIntent>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-toast".to_string(),
                    format!("orbital-toast--{}", intent.get().as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="status"
        >
            {children()}
        </div>
    }
}
