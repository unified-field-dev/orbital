use std::ops::Deref;

use leptos::prelude::*;

use super::{FieldValidationState, Rule};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum NumericStepperRuleTrigger {
    #[default]
    Change,
}

pub struct NumericStepperRule<T>(Rule<T, NumericStepperRuleTrigger>);

impl<T> NumericStepperRule<T> {
    pub fn validator(
        f: impl Fn(&T, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: NumericStepperRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl<T> Deref for NumericStepperRule<T> {
    type Target = Rule<T, NumericStepperRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
