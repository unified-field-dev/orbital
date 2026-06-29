use leptos::prelude::*;
use orbital_base_components::{FormBind, Handler, InputRule, InputSize, InputType};

/// Value binding, identity, and validation for [`Input`](super::input::Input).
#[derive(Default)]
pub struct InputBind {
    /// Two-way string value (signal, store field, or plain initial value).
    pub value: FormBind<String>,
    /// Explicit `id` for the `<input>`; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<InputRule>,
}

impl InputBind {
    pub fn new(value: impl Into<FormBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<String>> for InputBind {
    fn from(value: RwSignal<String>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<String>> for InputBind {
    fn from(value: FormBind<String>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

/// Visual and native input attributes for [`Input`](super::input::Input).
#[derive(Default)]
pub struct InputAppearance {
    /// When true, focuses the input when it mounts.
    pub autofocus: Signal<bool>,
    /// HTML input type (text, email, password, search, etc.).
    pub input_type: Signal<InputType>,
    /// Placeholder text shown when the value is empty.
    pub placeholder: MaybeProp<String>,
    /// When true, the input cannot be edited or focused for editing.
    pub disabled: Signal<bool>,
    /// When true, the value is visible but not editable.
    pub readonly: Signal<bool>,
    /// Native `size` attribute width (character columns), not Orbital visual size.
    pub input_size: Signal<Option<i32>>,
    /// Orbital visual size (small, medium, large).
    pub size: Signal<InputSize>,
    /// Native `autocomplete` attribute value.
    pub autocomplete: MaybeProp<String>,
    /// Inline style string applied to the native `<input>` element.
    pub input_style: MaybeProp<String>,
    /// Transforms raw user input before assigning to the value (runs on change); return `None` to reject.
    pub parser: Option<Handler<String, Option<String>>>,
    /// Formats the stored value for display in the input.
    pub format: Option<Handler<String, String>>,
}

impl InputAppearance {
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

    pub fn autofocus(placeholder: impl Into<String>) -> Self {
        Self {
            autofocus: Signal::from(true),
            placeholder: MaybeProp::from(placeholder.into()),
            ..Default::default()
        }
    }

    pub fn email(placeholder: impl Into<String>) -> Self {
        Self {
            input_type: Signal::from(InputType::Email),
            placeholder: MaybeProp::from(placeholder.into()),
            ..Default::default()
        }
    }
}

impl From<InputSize> for InputAppearance {
    fn from(size: InputSize) -> Self {
        Self {
            size: Signal::from(size),
            ..Default::default()
        }
    }
}

impl From<InputType> for InputAppearance {
    fn from(input_type: InputType) -> Self {
        Self {
            input_type: Signal::from(input_type),
            ..Default::default()
        }
    }
}

/// Focus, blur, and input-guard callbacks for [`Input`](super::input::Input).
#[derive(Default)]
pub struct InputEvents {
    /// Fired when the input receives focus.
    pub on_focus: Option<Handler<leptos::ev::FocusEvent>>,
    /// Fired when the input loses focus.
    pub on_blur: Option<Handler<leptos::ev::FocusEvent>>,
    /// When set, returns false to reject incoming keystrokes.
    pub allow_value: Option<Handler<String, bool>>,
}
