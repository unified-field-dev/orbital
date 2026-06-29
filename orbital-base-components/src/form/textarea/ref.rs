use leptos::{html, prelude::*};

#[derive(Clone)]
pub struct TextareaRef {
    pub(crate) textarea_ref: NodeRef<html::Textarea>,
}

impl TextareaRef {
    pub fn new(textarea_ref: NodeRef<html::Textarea>) -> Self {
        Self { textarea_ref }
    }

    pub fn focus(&self) {
        if let Some(el) = self.textarea_ref.get_untracked() {
            _ = el.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(el) = self.textarea_ref.get_untracked() {
            _ = el.blur();
        }
    }
}
