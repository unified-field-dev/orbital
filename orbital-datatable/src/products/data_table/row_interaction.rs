/// Returns true when the click target should not trigger row-level activation.
pub fn click_target_is_interactive(ev: &leptos::ev::MouseEvent) -> bool {
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::JsCast;
        let Some(target) = ev.target() else {
            return false;
        };
        let Some(el) = target.dyn_ref::<web_sys::Element>() else {
            return false;
        };
        let tag = el.tag_name().to_ascii_lowercase();
        if matches!(
            tag.as_str(),
            "input" | "button" | "a" | "select" | "textarea" | "label"
        ) {
            return true;
        }
        el.closest("[data-skip-row-click]").ok().flatten().is_some()
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = ev;
        false
    }
}

/// Returns true for keyboard keys that activate a row/card button.
pub fn keyboard_activates_row(ev: &leptos::ev::KeyboardEvent) -> bool {
    matches!(ev.key().as_str(), "Enter" | " ")
}
