use std::ops::Deref;

use leptos::prelude::*;

use crate::form::{format_datetime, DatetimeFormat, OrbitalDateTime};

use super::{FieldValidationState, Rule};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DatePickerRuleTrigger {
    #[default]
    Blur,
    Change,
    Select,
}

pub struct DatePickerRule(Rule<Option<OrbitalDateTime>, DatePickerRuleTrigger>);

impl DatePickerRule {
    pub fn required(required: Signal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_none() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please select date!"),
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

    pub fn min_date(min: Signal<Option<OrbitalDateTime>>) -> Self {
        Self::validator(move |value, _| {
            let Some(value) = value else {
                return Ok(());
            };
            let Some(min) = min.get_untracked() else {
                return Ok(());
            };
            if value.start_of_day() < min.start_of_day() {
                let bound = format_datetime(min.start_of_day(), DatetimeFormat::IsoDate);
                Err(FieldValidationState::Error(format!(
                    "Date must be on or after {bound}"
                )))
            } else {
                Ok(())
            }
        })
    }

    pub fn min_date_with_message(
        min: Signal<Option<OrbitalDateTime>>,
        message: Signal<String>,
    ) -> Self {
        Self::validator(move |value, _| {
            let Some(value) = value else {
                return Ok(());
            };
            let Some(min) = min.get_untracked() else {
                return Ok(());
            };
            if value.start_of_day() < min.start_of_day() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn max_date(max: Signal<Option<OrbitalDateTime>>) -> Self {
        Self::validator(move |value, _| {
            let Some(value) = value else {
                return Ok(());
            };
            let Some(max) = max.get_untracked() else {
                return Ok(());
            };
            if value.start_of_day() > max.start_of_day() {
                let bound = format_datetime(max.start_of_day(), DatetimeFormat::IsoDate);
                Err(FieldValidationState::Error(format!(
                    "Date must be on or before {bound}"
                )))
            } else {
                Ok(())
            }
        })
    }

    pub fn max_date_with_message(
        max: Signal<Option<OrbitalDateTime>>,
        message: Signal<String>,
    ) -> Self {
        Self::validator(move |value, _| {
            let Some(value) = value else {
                return Ok(());
            };
            let Some(max) = max.get_untracked() else {
                return Ok(());
            };
            if value.start_of_day() > max.start_of_day() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&Option<OrbitalDateTime>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: DatePickerRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for DatePickerRule {
    type Target = Rule<Option<OrbitalDateTime>, DatePickerRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
