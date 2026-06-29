use leptos::prelude::*;

use super::base::DialogInjection;

#[component]
pub fn BaseDialogSurface(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let _dialog = DialogInjection::expect_context();

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-dialog-surface".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="dialog"
            aria-modal="true"
        >
            {children()}
        </div>
    }
}
