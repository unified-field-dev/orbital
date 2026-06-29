//! Overlay layer context for portal mounts and viewport-relative tooltip positioning.

use leptos::{html, prelude::*};

use crate::{ChartEmbedMode, OverlayMount};

/// Overlay mount and root refs provided by [`ChartRoot`](crate::ChartRoot).
#[derive(Clone)]
pub struct ChartOverlayContext {
    /// Portal mount target inside [`ChartOverlayLayer`](crate::ChartOverlayLayer).
    pub layer_ref: NodeRef<html::Div>,
    /// Chart root element for `getBoundingClientRect` → fixed tooltip coords.
    pub root_ref: NodeRef<html::Div>,
    pub embed_mode: ChartEmbedMode,
    pub overlay_mount: OverlayMount,
}

impl ChartOverlayContext {
    /// Resolve the DOM element to portal overlay chrome into (client only).
    #[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
    pub fn resolve_portal_mount(&self) -> Option<web_sys::Element> {
        if let OverlayMount::HostElement { id } = &self.overlay_mount {
            if let Some(host) = resolve_host_element(id) {
                return Some(host);
            }
        }
        self.layer_ref.get().map(|el| el.into())
    }
}

pub fn provide_overlay_context(ctx: ChartOverlayContext) {
    provide_context(ctx);
}

pub fn use_overlay_context() -> ChartOverlayContext {
    expect_context::<ChartOverlayContext>()
}

#[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
fn resolve_host_element(id: &str) -> Option<web_sys::Element> {
    use leptos::wasm_bindgen::JsCast;
    let document = leptos::prelude::document();
    if let Some(el) = document.get_element_by_id(id) {
        return Some(el);
    }
    let selector = format!("[data-orbital-chart-host=\"{id}\"]");
    document
        .query_selector(&selector)
        .ok()
        .flatten()
        .map(|el| el.unchecked_into())
}
