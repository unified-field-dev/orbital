use leptos::prelude::*;
use orbital_base_components::{OptionBind, RatingColor, RatingSize};

#[derive(Clone)]
pub(crate) struct RatingInjection {
    pub value: OptionBind<f32>,
    pub hovered_value: RwSignal<Option<f32>>,
    pub name: Signal<String>,
    pub step: Signal<f32>,
    pub size: Signal<RatingSize>,
    pub color: Signal<RatingColor>,
    pub interactive: bool,
}

impl RatingInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
