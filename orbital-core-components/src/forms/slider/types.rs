use leptos::prelude::*;
use orbital_base_components::{FormBind, SliderRule};

/// Value binding, identity, and validation for [`Slider`](super::slider::Slider).
#[derive(Default)]
pub struct SliderBind {
    /// Two-way numeric value for the slider thumb position.
    pub value: FormBind<f64>,
    /// Explicit `id` for the native range input; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<SliderRule>,
}

impl SliderBind {
    pub fn new(value: impl Into<FormBind<f64>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<f64>> for SliderBind {
    fn from(value: RwSignal<f64>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<f64>> for SliderBind {
    fn from(value: FormBind<f64>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

/// Range, orientation, and rendering options for [`Slider`](super::slider::Slider).
pub struct SliderAppearance {
    /// Minimum value.
    pub min: Signal<f64>,
    /// Maximum value.
    pub max: Signal<f64>,
    /// Increment step. When omitted, any value in range is allowed.
    pub step: MaybeProp<f64>,
    /// Whether to draw stop marks along each step.
    pub show_stops: Signal<bool>,
    /// Vertical orientation with minimum at the bottom.
    pub vertical: Signal<bool>,
    /// Extra inline styles merged into the slider root.
    pub style: MaybeProp<String>,
}

impl Default for SliderAppearance {
    fn default() -> Self {
        Self {
            min: Signal::from(0.0),
            max: Signal::from(100.0),
            step: MaybeProp::default(),
            show_stops: Signal::from(true),
            vertical: Signal::from(false),
            style: MaybeProp::default(),
        }
    }
}
