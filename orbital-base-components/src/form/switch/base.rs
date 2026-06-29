use leptos::{html, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::{new_field_id, FieldInjection};

/// Headless switch — `role="switch"` checkbox input and indicator.
#[component(transparent)]
pub fn BaseSwitch(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] checked: FormBind<bool>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let fallback_id = StoredValue::new(new_field_id());
    let resolved_id = Signal::derive(move || id.get().unwrap_or_else(|| fallback_id.get_value()));
    let input_ref = NodeRef::<html::Input>::new();

    let checked_bind = checked.clone();
    let on_change = move |_| {
        if let Some(input) = input_ref.get_untracked() {
            checked_bind.set(input.checked());
        }
    };

    view! {
        <div class=move || class.get().unwrap_or_default()>
            <input
                type="checkbox"
                role="switch"
                id=resolved_id
                name=name
                value=move || value.get()
                prop:checked=move || checked.get()
                node_ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" role="presentation"></div>
            {move || {
                label.get().map(|text| {
                    view! {
                        <label for=resolved_id.get()>{text}</label>
                    }
                })
            }}
        </div>
    }
}
