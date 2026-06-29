use leptos::{either::Either, ev, prelude::*};
use orbital_base_components::{new_field_id, ListboxInjection};

use super::types::TagPickerInjection;

/// Selectable option rendered in the tag picker listbox.
#[component]
pub fn TagPickerOption(
    /// Optional CSS class merged onto the option root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Sets an option to the disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Defines a unique identifier for the option.
    #[prop(into)]
    value: String,
    /// Optional override for display text; defaults to children content.
    #[prop(into)]
    text: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let listbox = ListboxInjection::expect_context();
    let value = StoredValue::new(value);
    let text = StoredValue::new(text);
    let is_selected = Memo::new({
        let tag_picker = tag_picker.clone();
        move |_| value.with_value(|v| tag_picker.is_selected(v))
    });
    let id = new_field_id();

    {
        tag_picker.insert_option(id.clone(), (value.get_value(), text.get_value(), disabled));
        let id_for_cleanup = id.clone();
        let tag_picker_cleanup = tag_picker.clone();
        listbox.trigger();
        on_cleanup(move || {
            tag_picker_cleanup.remove_option(&id_for_cleanup);
            listbox.trigger();
        });
    }

    let tag_picker_click = tag_picker.clone();
    let on_click = move |e: ev::MouseEvent| {
        if disabled.get_untracked() {
            e.stop_propagation();
            return;
        }
        value.with_value(|v| {
            tag_picker_click.select_option(v);
        });
    };

    view! {
        <div
            role="option"
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            aria-selected=move || is_selected.get().to_string()
            id=id
            class=move || {
                let mut parts = vec!["orbital-tag-picker-option".to_string()];
                if is_selected.get() {
                    parts.push("orbital-tag-picker-option--selected".to_string());
                }
                if disabled.get() {
                    parts.push("orbital-tag-picker-option--disabled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            on:click=on_click
        >
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(text.get_value())
            }}
        </div>
    }
}
