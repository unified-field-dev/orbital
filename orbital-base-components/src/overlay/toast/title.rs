use leptos::prelude::*;

#[component]
pub fn BaseToastTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-toast-title", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseToastBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-toast-body", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseToastFooter(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-toast-footer", class.get())>
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
