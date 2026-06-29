//! Browser scroll helpers for reply anchors.

#[cfg(feature = "hydrate")]
use leptos::prelude::{document, request_animation_frame};

/// Scroll a reply row into view inside the discussion scroll container.
#[cfg(feature = "hydrate")]
pub fn scroll_reply_into_view(reply_id: &str) {
    use wasm_bindgen::JsCast;

    let document = document();
    let selector = format!("[data-reply-id=\"{reply_id}\"]");
    let Ok(Some(element)) = document.query_selector(&selector) else {
        return;
    };
    let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    element.scroll_into_view();
}

/// Schedule scroll after the next paint so newly drilled-in rows exist in the DOM.
#[cfg(feature = "hydrate")]
pub fn schedule_scroll_reply_into_view(reply_id: String) {
    request_animation_frame(move || {
        request_animation_frame(move || {
            scroll_reply_into_view(&reply_id);
        });
    });
}
