mod date_picker;
mod input;
mod numeric_stepper;
mod radio_group;
mod rating;
mod select;
mod slider;
mod switch;
mod textarea;

#[cfg(test)]
mod tests;

pub use date_picker::{DatePickerRule, DatePickerRuleTrigger};
pub use input::{InputRule, InputRuleTrigger};
pub use numeric_stepper::{NumericStepperRule, NumericStepperRuleTrigger};
pub use radio_group::{RadioGroupRule, RadioGroupRuleTrigger};
pub use rating::{RatingRule, RatingRuleTrigger};
pub use select::{SelectRule, SelectRuleTrigger};
pub use slider::{SliderRule, SliderRuleTrigger};
pub use switch::{SwitchRule, SwitchRuleTrigger};
pub use textarea::{TextareaRule, TextareaRuleTrigger};

use std::ops::Deref;

use leptos::prelude::*;

use super::bind::FormBind;
use super::field_injection::FieldInjection;
use super::field_validation::FieldValidationState;
use super::option_bind::OptionBind;

type RuleValidator<T> =
    Box<dyn Fn(&T, Signal<Option<String>>) -> Result<(), FieldValidationState> + Send + Sync>;

pub struct Rule<T, Trigger> {
    pub(crate) validator: RuleValidator<T>,
    pub(crate) trigger: Trigger,
}

impl<T, Trigger> Rule<T, Trigger> {
    pub fn validator(
        f: impl Fn(&T, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Trigger: Default,
    {
        Self {
            trigger: Default::default(),
            validator: Box::new(f),
        }
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;
        self
    }

    pub fn validate<V, R>(
        rules: Vec<R>,
        value: V,
        name: Signal<Option<String>>,
    ) -> Callback<Option<Trigger>, bool>
    where
        V: RuleValueWithUntracked<T>,
        V: Send + Sync + Copy + 'static,
        R: Deref<Target = Rule<T, Trigger>> + Send + Sync + 'static,
        Trigger: PartialEq + 'static,
    {
        if rules.is_empty() {
            return Callback::new(move |_trigger: Option<Trigger>| true);
        }

        let field_injection = FieldInjection::use_context();
        Callback::new(move |trigger: Option<Trigger>| {
            let state = {
                let mut rules_iter = rules.iter();
                let mut call_count = 0;
                loop {
                    let Some(rule) = rules_iter.next() else {
                        break if call_count == 0 { None } else { Some(Ok(())) };
                    };

                    if let Some(trigger) = trigger.as_ref() {
                        if &rule.trigger != trigger {
                            continue;
                        }
                    }
                    call_count += 1;

                    let state = value.value_with_untracked(|value| (rule.validator)(value, name));
                    if state.is_err() {
                        break Some(state);
                    }
                }
            };

            let Some(state) = state else {
                return true;
            };

            let rt = state.is_ok();
            if let Some(field_injection) = field_injection.as_ref() {
                field_injection.update_validation_state(state);
            }
            rt
        })
    }
}

pub trait RuleValueWithUntracked<T> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&T) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState>;
}

impl<T: Clone + Send + Sync + 'static> RuleValueWithUntracked<T> for FormBind<T> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&T) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        f(&self.get_untracked())
    }
}

impl<T: Clone + Send + Sync + 'static> RuleValueWithUntracked<Option<T>> for OptionBind<T> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Option<T>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        f(&self.get_untracked())
    }
}

impl<T: Clone + Send + Sync + 'static> RuleValueWithUntracked<T> for StoredValue<FormBind<T>> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&T) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        f(&self.get_value().get_untracked())
    }
}

impl<T: Clone + Send + Sync + 'static> RuleValueWithUntracked<Option<T>>
    for StoredValue<OptionBind<T>>
{
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Option<T>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        f(&self.get_value().get_untracked())
    }
}
