use leptos::prelude::*;
use orbital_base_components::{AutoCompleteSize, FormBind, Handler};

/// Value and identity bindings for [`AutoComplete`](super::auto_complete::AutoComplete).
#[derive(Default)]
pub struct AutoCompleteBind {
    /// Two-way input value.
    pub value: FormBind<String>,
    /// Explicit `id` for the input; auto-linked inside Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
}

impl AutoCompleteBind {
    pub fn new(value: impl Into<FormBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<String>> for AutoCompleteBind {
    fn from(value: RwSignal<String>) -> Self {
        Self::new(value)
    }
}

/// Visual and behavior attributes for [`AutoComplete`](super::auto_complete::AutoComplete).
#[derive(Default)]
pub struct AutoCompleteAppearance {
    pub placeholder: MaybeProp<String>,
    pub disabled: Signal<bool>,
    pub size: Signal<AutoCompleteSize>,
    pub clear_after_select: Signal<bool>,
    pub blur_after_select: Signal<bool>,
}

impl AutoCompleteAppearance {
    pub fn with_placeholder(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: MaybeProp::from(placeholder.into()),
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

impl From<AutoCompleteSize> for AutoCompleteAppearance {
    fn from(size: AutoCompleteSize) -> Self {
        Self {
            size: Signal::from(size),
            ..Default::default()
        }
    }
}

/// Event callbacks for [`AutoComplete`](super::auto_complete::AutoComplete).
#[derive(Default)]
pub struct AutoCompleteEvents {
    pub on_select: Option<Handler<String>>,
}
