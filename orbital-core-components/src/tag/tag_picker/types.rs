use leptos::{html, prelude::*};
use orbital_base_components::{ActiveDescendantController, FormBind, Handler};
use std::collections::HashMap;

#[slot]
pub struct TagPickerControl {
    pub children: Children,
}

#[derive(Clone)]
pub(crate) struct TagPickerControlInjection(pub ActiveDescendantController);

impl TagPickerControlInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Clone)]
pub(crate) struct TagPickerInjection {
    pub size: Signal<TagPickerSize>,
    pub input_ref: NodeRef<html::Input>,
    pub(super) selected_options: FormBind<Vec<String>>,
    pub options: StoredValue<HashMap<String, (String, String, Signal<bool>)>>,
    pub(super) is_show_listbox: RwSignal<bool>,
    pub(super) listbox_hidden_callback: StoredValue<Vec<Handler<()>>>,
}

impl TagPickerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    /// value: (value, text, disabled)
    pub fn insert_option(&self, id: String, value: (String, String, Signal<bool>)) {
        self.options.update_value(|options| {
            options.insert(id, value);
        });
    }

    pub fn remove_option(&self, id: &str) {
        self.options.update_value(|options| {
            options.remove(id);
        });
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_options
            .get()
            .iter()
            .any(|option| option == value)
    }

    pub fn select_option(&self, value: &str) {
        self.selected_options.update(|options| {
            if let Some(index) = options.iter().position(|v| v == value) {
                options.remove(index);
            } else {
                options.push(value.to_string());
                if let Some(input_el) = self.input_ref.get_untracked() {
                    input_el.set_value("");
                }
            }
        });
        self.is_show_listbox.set(false);
    }

    pub fn remove_selected_option(&self, value: String) {
        if self.is_show_listbox.get_untracked() {
            let selected_options = self.selected_options.clone();
            self.listbox_hidden_callback.update_value(|list| {
                list.push(Handler::new(move || {
                    selected_options.update(|options| {
                        if let Some(index) = options.iter().position(|v| v == &value) {
                            options.remove(index);
                        }
                    });
                }));
            });
        } else {
            self.selected_options.update(|options| {
                if let Some(index) = options.iter().position(|v| v == &value) {
                    options.remove(index);
                }
            });
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TagPickerSize {
    Medium,
    #[default]
    Large,
    ExtraLarge,
}

impl TagPickerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
        }
    }
}

/// Selected option keys for [`TagPicker`](super::tag_picker::TagPicker).
#[derive(Default)]
pub struct TagPickerBind {
    pub selected_options: FormBind<Vec<String>>,
}

impl TagPickerBind {
    pub fn new(selected_options: impl Into<FormBind<Vec<String>>>) -> Self {
        Self {
            selected_options: selected_options.into(),
        }
    }
}

impl From<FormBind<Vec<String>>> for TagPickerBind {
    fn from(selected_options: FormBind<Vec<String>>) -> Self {
        Self { selected_options }
    }
}
