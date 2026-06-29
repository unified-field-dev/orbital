use leptos::prelude::*;

/// Styled drop-zone shell rendered inside an upload trigger.
#[component]
pub fn BaseUploadDragger(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let root_class = move || {
        let mut parts = vec!["orbital-upload-dragger".to_string()];
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    };

    view! { <div class=root_class>{children()}</div> }
}
