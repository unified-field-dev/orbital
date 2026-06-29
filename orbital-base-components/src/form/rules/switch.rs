use std::ops::Deref;

use leptos::prelude::*;

use super::{FieldValidationState, Rule};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SwitchRuleTrigger {
    #[default]
    Change,
}

pub struct SwitchRule(Rule<bool, SwitchRuleTrigger>);

impl SwitchRule {
    pub fn validator(
        f: impl Fn(&bool, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: SwitchRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for SwitchRule {
    type Target = Rule<bool, SwitchRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
