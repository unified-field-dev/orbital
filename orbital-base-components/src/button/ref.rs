use leptos::{html, prelude::*};

#[derive(Clone)]
pub struct ButtonRef {
    pub(crate) button_ref: NodeRef<html::Button>,
}

impl ButtonRef {
    /// Click the button element.
    pub fn click(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            button_el.click();
        }
    }

    /// Focus the button element.
    pub fn focus(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.focus();
        }
    }
}
