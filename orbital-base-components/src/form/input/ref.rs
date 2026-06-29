use leptos::{html, prelude::*};

#[derive(Clone)]
pub struct InputRef {
    pub(crate) input_ref: NodeRef<html::Input>,
}

impl InputRef {
    pub fn new(input_ref: NodeRef<html::Input>) -> Self {
        Self { input_ref }
    }

    pub fn focus(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.blur();
        }
    }

    pub fn select(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            input_el.select();
        }
    }
}
