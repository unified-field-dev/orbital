use leptos::prelude::*;
use orbital_base_components::BackdropMode;

/// Visibility and behavior for [`Backdrop`].
#[derive(Clone, Copy)]
pub struct BackdropConfig {
    /// When false, the scrim is not rendered.
    pub open: Signal<bool>,
    /// Full viewport dim or spotlight cutout around an anchor.
    pub mode: BackdropMode,
}

impl BackdropConfig {
    pub fn new(open: impl Into<Signal<bool>>) -> Self {
        Self {
            open: open.into(),
            mode: BackdropMode::Full,
        }
    }

    pub fn with_mode(mut self, mode: BackdropMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Default for BackdropConfig {
    fn default() -> Self {
        Self {
            open: Signal::from(false),
            mode: BackdropMode::Full,
        }
    }
}
