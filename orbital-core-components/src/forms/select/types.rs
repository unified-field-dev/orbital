use leptos::prelude::*;
use orbital_base_components::{FormBind, SelectRule, SelectSize};

/// Value binding, identity, and validation for [`Select`](super::select::Select).
#[derive(Default)]
pub struct SelectBind {
    /// Two-way selected option value (signal, store field, or plain initial value).
    pub value: FormBind<String>,
    /// Explicit `id` for the `<select>`; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<SelectRule>,
}

impl SelectBind {
    pub fn new(value: impl Into<FormBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<String>> for SelectBind {
    fn from(value: RwSignal<String>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<String>> for SelectBind {
    fn from(value: FormBind<String>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

/// Visual and native select attributes for [`Select`](super::select::Select).
#[derive(Default)]
pub struct SelectAppearance {
    /// When true, the select cannot be changed.
    pub disabled: Signal<bool>,
    /// Orbital visual size (small, medium, large).
    pub size: Signal<SelectSize>,
    /// Initial DOM value on first mount; syncs into `value` when the signal is empty.
    pub default_value: Option<String>,
}

impl SelectAppearance {
    pub fn disabled() -> Self {
        Self {
            disabled: Signal::from(true),
            ..Default::default()
        }
    }
}

impl From<SelectSize> for SelectAppearance {
    fn from(size: SelectSize) -> Self {
        Self {
            size: Signal::from(size),
            ..Default::default()
        }
    }
}
