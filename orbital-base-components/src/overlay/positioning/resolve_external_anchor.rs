/// Resolve a DOM element by its `id` attribute for external anchor positioning.
#[cfg(not(feature = "ssr"))]
pub fn resolve_external_anchor(id: &str) -> Option<web_sys::Element> {
    if id.is_empty() {
        return None;
    }
    leptos::prelude::document().get_element_by_id(id)
}

/// SSR stub — no DOM available at render time.
#[cfg(feature = "ssr")]
pub fn resolve_external_anchor(_id: &str) -> Option<web_sys::Element> {
    None
}
