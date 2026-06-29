use leptos::prelude::*;

/// Backdrop and keyboard dismiss behavior for [`Dialog`](super::dialog::Dialog).
#[derive(Clone, Copy)]
pub struct DialogDismissConfig {
    /// When true, clicking the backdrop sets `open` to `false`.
    pub mask_closeable: Signal<bool>,
    /// When true, pressing Esc sets `open` to `false`.
    pub close_on_esc: bool,
}

impl Default for DialogDismissConfig {
    fn default() -> Self {
        Self {
            mask_closeable: Signal::from(true),
            close_on_esc: true,
        }
    }
}
