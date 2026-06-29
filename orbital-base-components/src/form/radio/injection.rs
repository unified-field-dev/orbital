use leptos::prelude::*;

use crate::form::OptionBind;

#[derive(Clone)]
pub(crate) struct RadioGroupInjection {
    pub value: OptionBind<String>,
    pub name: Signal<String>,
}

impl RadioGroupInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
