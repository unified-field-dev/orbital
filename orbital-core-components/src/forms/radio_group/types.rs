use leptos::prelude::*;
use orbital_base_components::{OptionBind, RadioGroupRule};

/// Layout direction for [`RadioGroup`](super::radio_group::RadioGroup) options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RadioGroupLayout {
    /// Stack options vertically (default).
    #[default]
    Vertical,
    /// Arrange options in a horizontal row.
    Horizontal,
}

/// Value binding, identity, and validation for [`RadioGroup`](super::radio_group::RadioGroup).
#[derive(Default)]
pub struct RadioGroupBind {
    /// Two-way selected value (signal, store field, or plain initial value).
    pub value: OptionBind<String>,
    /// Explicit `id` for the radiogroup root; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name shared by all radios in this group.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<RadioGroupRule>,
}

impl RadioGroupBind {
    pub fn new(value: impl Into<OptionBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<String>>> for RadioGroupBind {
    fn from(value: RwSignal<Option<String>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<String>> for RadioGroupBind {
    fn from(value: OptionBind<String>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}
