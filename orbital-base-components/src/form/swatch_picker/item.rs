use leptos::{html, prelude::*};

use super::injection::SwatchItemRegistration;
use super::SwatchPickerInjection;

/// Headless swatch option for [`super::BaseSwatchPicker`].
#[component(transparent)]
pub fn BaseSwatchPickerItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: String,
    #[prop(into)] color: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    let picker = SwatchPickerInjection::expect_context();
    let value_stored = StoredValue::new(value);
    let color = StoredValue::new(color);
    let button_ref = NodeRef::<html::Button>::new();

    {
        picker.register_item(SwatchItemRegistration {
            value: value_stored.get_value(),
            disabled,
            button_ref,
        });
        let picker_cleanup = picker.clone();
        let value_for_cleanup = value_stored;
        on_cleanup(move || {
            value_for_cleanup.with_value(|value| picker_cleanup.unregister_item(value));
        });
    }

    let is_selected = Memo::new({
        let picker = picker.clone();
        move |_| value_stored.with_value(|value| picker.is_selected(value))
    });

    let aria_label = Memo::new(move |_| {
        label
            .get()
            .or_else(|| value_stored.with_value(|value| Some(value.clone())))
            .unwrap_or_default()
    });

    let on_click = {
        let picker = picker.clone();
        move |_| {
            if disabled.get_untracked() {
                return;
            }
            value_stored.with_value(|value| picker.select(value.clone()));
        }
    };

    view! {
        <button
            type="button"
            node_ref=button_ref
            class=move || class.get().unwrap_or_default()
            role="radio"
            aria-checked=move || if is_selected.get() { "true" } else { "false" }
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            aria-label=move || aria_label.get()
            tabindex=move || if is_selected.get() { "0" } else { "-1" }
            disabled=move || disabled.get()
            style=move || format!("--orbital-swatch-picker__color: {}", color.get_value())
            on:click=on_click
        />
    }
}
