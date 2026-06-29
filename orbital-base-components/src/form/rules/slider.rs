use std::ops::Deref;

use leptos::prelude::*;

use super::{FieldValidationState, Rule};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SliderRuleTrigger {
    #[default]
    Input,
}

pub struct SliderRule(Rule<f64, SliderRuleTrigger>);

impl SliderRule {
    pub fn validator(
        f: impl Fn(&f64, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: SliderRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for SliderRule {
    type Target = Rule<f64, SliderRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
