//! Session / local remember helpers for coachmarks (`hydrate` + wasm only).

/// Returns `true` if the user dismissed this coachmark for the browser session.
pub fn is_session_dismissed(key: &str) -> bool {
    #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
    {
        use leptos::web_sys::window;
        if let Some(w) = window() {
            if let Ok(Some(storage)) = w.session_storage() {
                if let Ok(Some(v)) = storage.get_item(key) {
                    return v == "1";
                }
            }
        }
    }
    let _ = key;
    false
}

/// Persist session dismissal (tab-scoped).
pub fn set_session_dismissed(key: &str) {
    #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
    {
        use leptos::web_sys::window;
        if let Some(w) = window() {
            if let Ok(Some(storage)) = w.session_storage() {
                let _ = storage.set_item(key, "1");
            }
        }
    }
    let _ = key;
}
