use leptos::prelude::*;

/// Headless button-group wrapper for adjacent button layouts.
#[component(transparent)]
pub fn BaseButtonGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] vertical: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-button-group".to_string()];
                if vertical {
                    parts.push("orbital-button-group--vertical".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="group"
        >
            {children()}
        </div>
    }
}
