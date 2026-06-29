//! Shortcut preset row for calendar and picker layouts.

use leptos::prelude::*;
use orbital_base_components::{OrbitalDateTime, PickerShortcut};

use crate::button::{Button, ButtonAppearance};
use crate::flex::{Flex, FlexGap, FlexWrap};

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
        <div data-testid="picker-shortcuts-bar">
            <Flex class="orb-picker-shortcuts" gap=FlexGap::Small wrap=FlexWrap::Wrap full_width=true>
                {move || {
                    shortcuts
                        .get()
                        .into_iter()
                        .map(|shortcut| {
                            let label = shortcut.label.clone();
                            let value = shortcut.value;
                            let on_select = on_select;
                            view! {
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    disabled=disabled
                                    on_click=Callback::new(move |_| on_select.run(value))
                                >
                                    {label}
                                </Button>
                            }
                        })
                        .collect_view()
                }}
            </Flex>
        </div>
    }
}
