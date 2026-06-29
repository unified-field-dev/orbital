use leptos::{html, prelude::*};

use super::positioning::AnchorArrow;

pub const ARROW_EDGE_LENGTH: f64 = 1.414 * 8.0;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayArrowMode {
    #[default]
    None,
    Tooltip,
    Popover,
}

impl OverlayArrowMode {
    pub fn enabled(&self) -> bool {
        !matches!(self, Self::None)
    }
}

pub fn arrow_style() -> String {
    let offset = -(ARROW_EDGE_LENGTH / 2.0);
    format!(
        "--orbital-positioning-arrow-height: {h}px; --orbital-positioning-arrow-offset: {o}px;",
        h = ARROW_EDGE_LENGTH,
        o = offset,
    )
}

pub fn build_anchor_arrow(arrow_ref: NodeRef<html::Div>) -> AnchorArrow {
    AnchorArrow {
        safe_width: 4.0,
        width: ARROW_EDGE_LENGTH / 2.0 + 1.0,
        height: ARROW_EDGE_LENGTH / 2.0 + 2.0,
        node_ref: arrow_ref,
    }
}
