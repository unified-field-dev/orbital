use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::overlay::dom_events::get_scroll_parent_node;

/// Collect Orbital scrollport ancestors from `node` up to the document root.
fn collect_scroll_areas(node: &web_sys::Node) -> Vec<web_sys::Element> {
    let mut areas = Vec::new();
    let mut current = node.parent_node();
    while let Some(parent) = current {
        if parent.node_type() == web_sys::Node::ELEMENT_NODE {
            if let Ok(el) = parent.clone().dyn_into::<web_sys::Element>() {
                let class = el.class_name();
                if class.contains("orbital-scroll-area") {
                    areas.push(el);
                }
            }
        }
        current = parent.parent_node();
    }
    areas
}

/// Returns the catalog preview scrollport for `node`, matching Playwright `previewScrollport`.
pub fn scroll_parent_element(node: &web_sys::Node) -> web_sys::Element {
    let areas = collect_scroll_areas(node);
    if let Some(el) = areas
        .iter()
        .find(|el| el.class_name().contains("orbital-layout__page-scroll"))
    {
        return el.clone();
    }
    if let Some(el) = areas
        .iter()
        .find(|el| el.class_name().contains("orbital-layout__main-scroll"))
    {
        return el.clone();
    }
    if let Some(el) = areas.into_iter().next() {
        return el;
    }

    get_scroll_parent_node(node)
        .and_then(|parent| parent.dyn_into::<web_sys::Element>().ok())
        .unwrap_or_else(|| document().document_element().unwrap())
}

/// True when the scrollport is the page viewport (document element or body).
pub fn is_viewport_scroll_element(el: &web_sys::Element) -> bool {
    if document()
        .document_element()
        .is_some_and(|html| html == *el)
    {
        return true;
    }
    document().body().is_some_and(|body| {
        body.dyn_ref::<web_sys::Element>()
            .map(|body_el| body_el == el)
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::{is_viewport_scroll_element, scroll_parent_element};

    #[test]
    fn scroll_parent_helper_is_available() {
        let _ = scroll_parent_element as fn(&web_sys::Node) -> web_sys::Element;
        let _ = is_viewport_scroll_element as fn(&web_sys::Element) -> bool;
    }
}
