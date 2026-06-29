use leptos::{context::Provider, ev, prelude::*};

use super::injection::SwatchPickerInjection;
use super::types::SwatchPickerLayout;

/// Headless swatch picker with radiogroup semantics and arrow-key navigation.
#[component]
pub fn BaseSwatchPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] layout: Signal<SwatchPickerLayout>,
    #[prop(optional, into)] spacing: Signal<Option<i32>>,
    #[prop(optional, into)] shape: Signal<super::SwatchPickerShape>,
    #[prop(optional, into)] size: Signal<super::SwatchPickerSize>,
    #[prop(optional, into)] selected_value: MaybeProp<String>,
    #[prop(optional, into)] default_selected_value: MaybeProp<String>,
    #[prop(optional)] on_selection_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let selected = RwSignal::new(
        default_selected_value
            .get_untracked()
            .or_else(|| selected_value.get_untracked()),
    );

    Effect::new(move |_| {
        if let Some(value) = selected_value.get() {
            selected.set(Some(value));
        }
    });

    let injection = SwatchPickerInjection {
        selected_value: selected,
        registered_items: RwSignal::new(Default::default()),
        item_order: RwSignal::new(Vec::new()),
        layout,
        shape,
        size,
        on_selection_change,
    };

    let on_keydown = {
        let injection = injection.clone();
        move |ev: ev::KeyboardEvent| match ev.key().as_str() {
            "ArrowRight" | "ArrowDown" => {
                ev.prevent_default();
                injection.select_adjacent(1);
            }
            "ArrowLeft" | "ArrowUp" => {
                ev.prevent_default();
                injection.select_adjacent(-1);
            }
            "Home" => {
                ev.prevent_default();
                injection.select_endpoint(true);
            }
            "End" => {
                ev.prevent_default();
                injection.select_endpoint(false);
            }
            _ => {}
        }
    };

    view! {
        <Provider value=injection>
            <div
                class=move || class.get().unwrap_or_default()
                role="radiogroup"
                style=move || {
                    spacing
                        .get()
                        .map(|gap| format!("gap: {gap}px;"))
                        .unwrap_or_default()
                }
                on:keydown=on_keydown
            >
                {children()}
            </div>
        </Provider>
    }
}
