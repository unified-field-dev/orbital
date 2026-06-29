use leptos::{html, prelude::*};
use orbital_base_components::arrow_style;

/// Popover-style arrow that inherits the surface background color.
///
/// `arrow_ref` must match the [`AnchorArrow::node_ref`] passed to [`AnchoredSurface`], so positioning can center the arrow on the anchor target.
pub fn spotlight_arrow(arrow_ref: NodeRef<html::Div>) -> AnyView {
    view! {
        <div
            class="orbital-popover-surface__angle"
            style=arrow_style()
            node_ref=arrow_ref
        ></div>
    }
    .into_any()
}
