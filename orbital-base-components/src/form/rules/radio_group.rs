use std::ops::Deref;

use leptos::prelude::*;

use super::{FieldValidationState, Rule};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum RadioGroupRuleTrigger {
    #[default]
    Change,
}

pub struct RadioGroupRule(Rule<Option<String>, RadioGroupRuleTrigger>);

impl RadioGroupRule {
    pub fn required(required: Signal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_none() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please select!"),
                    |name| format!("Please select {name}!"),
                );
                Err(FieldValidationState::Error(message))
            } else {
                Ok(())
            }
        })
    }

    pub fn required_with_message(required: Signal<bool>, message: Signal<String>) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_none() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&Option<String>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: RadioGroupRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for RadioGroupRule {
    type Target = Rule<Option<String>, RadioGroupRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
