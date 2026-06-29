use leptos::{html, prelude::*};
use orbital_style::inject_style;

use super::overlay_layer_root_styles::overlay_layer_root_styles;

/// Scoped portal mount target with an isolated stacking context for embedded overlays.
///
/// Renders children inline inside a positioned shell. Pass the same `node_ref` to
/// [`ThemedPortal`](crate::ThemedPortal) or anchored overlays (`mount` on [`Popover`](crate::Popover)
/// / [`Tooltip`](crate::Tooltip)) so interactive chrome portals into this layer instead of
/// `document.body`.
#[component]
pub fn OverlayLayerRoot(
    /// Mount target for descendant portals. When omitted, an internal ref is allocated.
    #[prop(optional)]
    node_ref: Option<NodeRef<html::Div>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-overlay-layer-root", overlay_layer_root_styles());

    let layer_ref = node_ref.unwrap_or_default();
    let root_class = Signal::derive(move || {
        let mut classes = vec!["orbital-overlay-layer-root".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    });

    view! {
        <div class=root_class node_ref=layer_ref>
            {children()}
        </div>
    }
}
