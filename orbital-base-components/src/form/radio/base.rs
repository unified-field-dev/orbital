use leptos::prelude::*;

use crate::form::field_injection::new_field_id;

use super::injection::RadioGroupInjection;

/// Headless radio option bound to a parent [`BaseRadioGroup`](super::group::BaseRadioGroup).
#[component(transparent)]
pub fn BaseRadio(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] value: String,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    let id = StoredValue::new(new_field_id());
    let group = RadioGroupInjection::expect_context();
    let item_value = StoredValue::new(value);

    let checked = Memo::new({
        let group = group.clone();
        move |_| group.value.get().as_ref() == Some(&item_value.get_value())
    });

    let on_change = move |_| {
        group.value.set(Some(item_value.get_value()));
    };

    view! {
        <span class=move || class.get().unwrap_or_default()>
            <input
                class="orbital-radio__input"
                type="radio"
                id=move || id.get_value()
                name=group.name
                value=move || item_value.get_value()
                prop:checked=move || checked.get()
                checked=move || checked.get()
                on:change=on_change
            />
            <div class="orbital-radio__indicator" aria-hidden="true" role="presentation"></div>
            {move || {
                label.get().map(|text| {
                    view! {
                        <label class="orbital-radio__label" for=id.get_value()>
                            {text}
                        </label>
                    }
                })
            }}
        </span>
    }
}
