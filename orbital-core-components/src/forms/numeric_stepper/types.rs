use leptos::prelude::*;
use orbital_base_components::{FormBind, NumericStepperRule};

/// Value binding, identity, and validation for [`NumericStepper`](super::numeric_stepper::NumericStepper).
#[derive(Default)]
pub struct NumericStepperBind {
    /// Two-way numeric value.
    pub value: FormBind<i32>,
    /// Explicit `id` for the input; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<NumericStepperRule<i32>>,
}

impl NumericStepperBind {
    pub fn new(value: impl Into<FormBind<i32>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<i32>> for NumericStepperBind {
    fn from(value: RwSignal<i32>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<i32>> for NumericStepperBind {
    fn from(value: FormBind<i32>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum NumericStepperSize {
    Small,
    #[default]
    Medium,
}

impl NumericStepperSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
        }
    }
}

/// Range and visual options for [`NumericStepper`](super::numeric_stepper::NumericStepper).
pub struct NumericStepperAppearance {
    /// Minimum allowed value.
    pub min: Signal<i32>,
    /// Maximum allowed value.
    pub max: Signal<i32>,
    /// Increment/decrement amount for +/- buttons.
    pub step: Signal<i32>,
    /// Placeholder text displayed when empty.
    pub placeholder: MaybeProp<String>,
    /// When true, interaction is disabled.
    pub disabled: Signal<bool>,
    /// Visual size variant.
    pub size: Signal<NumericStepperSize>,
}

impl Default for NumericStepperAppearance {
    fn default() -> Self {
        Self {
            min: Signal::from(i32::MIN),
            max: Signal::from(i32::MAX),
            step: Signal::from(1),
            placeholder: MaybeProp::default(),
            disabled: Signal::from(false),
            size: Signal::from(NumericStepperSize::Medium),
        }
    }
}
