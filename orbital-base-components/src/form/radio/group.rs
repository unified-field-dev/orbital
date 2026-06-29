use leptos::{context::Provider, prelude::*};

use crate::form::field_injection::{new_field_id, FieldInjection};
use crate::form::rules::{RadioGroupRule, RadioGroupRuleTrigger, Rule};
use crate::form::OptionBind;

use super::injection::RadioGroupInjection;

/// Headless radio-group wrapper that provides name/value context to [`BaseRadio`](super::base::BaseRadio).
#[component(transparent)]
pub fn BaseRadioGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<RadioGroupRule>,
    #[prop(optional, into)] value: OptionBind<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    Effect::new(move |prev: Option<()>| {
        value.with_value(|value| {
            value.get();
        });
        if prev.is_some() {
            validate.run(Some(RadioGroupRuleTrigger::Change));
        }
    });

    let fallback_name = StoredValue::new(new_field_id());
    let resolved_name =
        Signal::derive(move || name.get().unwrap_or_else(|| fallback_name.get_value()));

    view! {
        <Provider value=RadioGroupInjection {
            value: value.get_value(),
            name: resolved_name,
        }>
            <div class=move || class.get().unwrap_or_default() id=id role="radiogroup">
                {children()}
            </div>
        </Provider>
    }
}
