use leptos::{context::Provider, prelude::*};

use crate::form::field_injection::{new_field_id, FieldInjection};
use crate::form::field_validation::FieldValidationState;
use crate::form::types::FieldOrientation;

/// Headless field shell — provides [`FieldInjection`] to child controls.
#[component(transparent)]
pub fn BaseField(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] field_orientation: Signal<FieldOrientation>,
    #[prop(optional, into)] required: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let _ = (field_orientation, required);
    let id = StoredValue::new(new_field_id());
    let validation_state = RwSignal::new(None::<FieldValidationState>);

    view! {
        <div
            class=move || {
                let mut parts = Vec::new();
                if let Some(c) = class.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = orientation.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                parts.join(" ")
            }
        >
            <Provider value=FieldInjection {
                id,
                name,
                label,
                validation_state,
            }>{children()}</Provider>
        </div>
    }
}
