use leptos::prelude::*;
use orbital_base_components::{FormBind, SwitchRule};

/// Value binding, identity, and validation for [`Switch`](super::switch::Switch).
#[derive(Default)]
pub struct SwitchBind {
    /// Two-way checked state (signal, store field, or plain initial value).
    pub checked: FormBind<bool>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Value submitted when the switch is on.
    pub value: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<SwitchRule>,
    /// When true, the switch cannot be toggled.
    pub disabled: Signal<bool>,
}

impl SwitchBind {
    pub fn new(checked: impl Into<FormBind<bool>>) -> Self {
        Self {
            checked: checked.into(),
            ..Default::default()
        }
    }

    pub fn disabled() -> Self {
        Self {
            disabled: Signal::from(true),
            ..Default::default()
        }
    }
}

impl From<RwSignal<bool>> for SwitchBind {
    fn from(checked: RwSignal<bool>) -> Self {
        Self::new(checked)
    }
}

impl From<FormBind<bool>> for SwitchBind {
    fn from(checked: FormBind<bool>) -> Self {
        Self {
            checked,
            ..Default::default()
        }
    }
}

/// Label text for [`Switch`](super::switch::Switch).
#[derive(Default)]
pub struct SwitchLabel {
    /// Visible label associated with the switch control.
    pub label: MaybeProp<String>,
}

impl SwitchLabel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: MaybeProp::from(label.into()),
        }
    }
}

impl From<String> for SwitchLabel {
    fn from(label: String) -> Self {
        Self::new(label)
    }
}

impl From<&str> for SwitchLabel {
    fn from(label: &str) -> Self {
        Self::new(label)
    }
}
