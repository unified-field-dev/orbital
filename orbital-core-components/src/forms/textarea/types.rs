use leptos::prelude::*;
use orbital_base_components::{FormBind, Handler, TextareaResize, TextareaRule, TextareaSize};

/// Value binding, identity, and validation for [`Textarea`](super::textarea::Textarea).
#[derive(Default)]
pub struct TextareaBind {
    /// Two-way string value (signal, store field, or plain initial value).
    pub value: FormBind<String>,
    /// Explicit `id` for the `<textarea>`; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<TextareaRule>,
}

impl TextareaBind {
    pub fn new(value: impl Into<FormBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<String>> for TextareaBind {
    fn from(value: RwSignal<String>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<String>> for TextareaBind {
    fn from(value: FormBind<String>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

/// Visual attributes for [`Textarea`](super::textarea::Textarea).
#[derive(Default)]
pub struct TextareaAppearance {
    pub placeholder: MaybeProp<String>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub rows: Signal<Option<u32>>,
    pub cols: Signal<Option<u32>>,
    pub resize: Signal<TextareaResize>,
    pub size: Signal<TextareaSize>,
}

impl TextareaAppearance {
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

    pub fn fixed() -> Self {
        Self {
            resize: Signal::from(TextareaResize::None),
            ..Default::default()
        }
    }
}

impl From<TextareaSize> for TextareaAppearance {
    fn from(size: TextareaSize) -> Self {
        Self {
            size: Signal::from(size),
            ..Default::default()
        }
    }
}

/// Focus, blur, and input-guard callbacks for [`Textarea`](super::textarea::Textarea).
#[derive(Default)]
pub struct TextareaEvents {
    /// Fired when the textarea receives focus.
    pub on_focus: Option<Handler<leptos::ev::FocusEvent>>,
    /// Fired when the textarea loses focus.
    pub on_blur: Option<Handler<leptos::ev::FocusEvent>>,
    /// When set, returns false to reject incoming input.
    pub allow_value: Option<Handler<String, bool>>,
}
