use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct FloatingActionsMenuInjection {
    pub open: RwSignal<bool>,
    pub persistent_tooltips: Signal<bool>,
}

impl FloatingActionsMenuInjection {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }
}
