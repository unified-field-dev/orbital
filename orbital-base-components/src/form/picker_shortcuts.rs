use leptos::prelude::*;

use super::OrbitalDateTime;
use super::PickerShortcut;

/// Horizontal row of shortcut preset buttons for calendar/picker layouts.
#[component]
pub fn PickerShortcutsBar(
    /// Preset labels and target values.
    #[prop(into)]
    shortcuts: Signal<Vec<PickerShortcut>>,
    /// Disables all shortcut buttons.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Called when a shortcut is selected with its target value.
    #[prop(into)]
    on_select: Callback<OrbitalDateTime>,
) -> impl IntoView {
    view! {
        <div class="orb-picker-shortcuts" data-testid="picker-shortcuts-bar">
            {move || {
                shortcuts
                    .get()
                    .into_iter()
                    .map(|shortcut| {
                        let label = shortcut.label.clone();
                        let value = shortcut.value;
                        let on_select = on_select;
                        view! {
                            <button
                                type="button"
                                class="orb-picker-shortcuts__button"
                                disabled=move || disabled.get()
                                on:click=move |_| on_select.run(value)
                            >
                                {label}
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
