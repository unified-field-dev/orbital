/// Overlay scroll behavior when [`Layout`](crate::Layout) uses `overlay_header=true`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LayoutOverlayScroll {
    /// When true, main uses an inner scrollport below opaque fixed chrome. When false (default), the shell uses pinned sticky chrome and window scroll.
    pub main_inset_scroll: bool,
}
