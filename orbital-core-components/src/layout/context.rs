use leptos::prelude::*;

/// Shared open/closed state for [`Layout`](crate::Layout) sidebars.
#[derive(Clone, Copy)]
pub struct LayoutSidebarOpen(pub RwSignal<bool>);

impl LayoutSidebarOpen {
    pub fn provide(open: RwSignal<bool>) {
        provide_context(Self(open));
    }

    pub fn expect_context() -> Self {
        expect_context()
    }
}
