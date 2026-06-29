use leptos::{html, prelude::*};
use std::collections::HashMap;

use super::types::{SwatchPickerLayout, SwatchPickerShape, SwatchPickerSize};

#[derive(Clone)]
pub struct SwatchItemRegistration {
    pub value: String,
    pub disabled: Signal<bool>,
    pub button_ref: NodeRef<html::Button>,
}

#[derive(Clone)]
pub struct SwatchPickerInjection {
    pub selected_value: RwSignal<Option<String>>,
    pub registered_items: RwSignal<HashMap<String, SwatchItemRegistration>>,
    pub item_order: RwSignal<Vec<String>>,
    pub layout: Signal<SwatchPickerLayout>,
    pub shape: Signal<SwatchPickerShape>,
    pub size: Signal<SwatchPickerSize>,
    pub on_selection_change: Option<Callback<String>>,
}

impl SwatchPickerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn register_item(&self, registration: SwatchItemRegistration) {
        let value = registration.value.clone();
        self.item_order.update(|order| {
            if !order.contains(&value) {
                order.push(value.clone());
            }
        });
        self.registered_items.update(|items| {
            items.insert(value, registration);
        });
    }

    pub fn unregister_item(&self, value: &str) {
        self.item_order.update(|order| {
            order.retain(|entry| entry != value);
        });
        self.registered_items.update(|items| {
            items.remove(value);
        });
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value
            .with(|selected| selected.as_deref() == Some(value))
    }

    pub fn select(&self, value: String) {
        if self.registered_items.with_untracked(|items| {
            items
                .get(&value)
                .is_some_and(|item| item.disabled.get_untracked())
        }) {
            return;
        }

        self.selected_value.set(Some(value.clone()));
        if let Some(callback) = self.on_selection_change {
            callback.run(value);
        }
    }

    pub fn ordered_items(&self) -> Vec<SwatchItemRegistration> {
        self.item_order.with(|order| {
            self.registered_items.with(|items| {
                order
                    .iter()
                    .filter_map(|value| items.get(value).cloned())
                    .collect()
            })
        })
    }

    pub fn focus_item(&self, value: &str) {
        if let Some(item) = self
            .registered_items
            .with_untracked(|items| items.get(value).cloned())
        {
            if let Some(button) = item.button_ref.get_untracked() {
                let _ = button.focus();
            }
        }
    }

    pub fn select_adjacent(&self, direction: i32) {
        let items = self.ordered_items();
        if items.is_empty() {
            return;
        }

        let enabled: Vec<_> = items
            .into_iter()
            .filter(|item| !item.disabled.get_untracked())
            .collect();
        if enabled.is_empty() {
            return;
        }

        let current = self.selected_value.get_untracked();
        let current_index = current
            .as_ref()
            .and_then(|value| enabled.iter().position(|item| &item.value == value))
            .unwrap_or(0);

        let next_index = if direction < 0 {
            if current_index == 0 {
                enabled.len() - 1
            } else {
                current_index - 1
            }
        } else if current_index + 1 >= enabled.len() {
            0
        } else {
            current_index + 1
        };

        let next_value = enabled[next_index].value.clone();
        self.select(next_value.clone());
        self.focus_item(&next_value);
    }

    pub fn select_endpoint(&self, first: bool) {
        let items = self.ordered_items();
        let target = if first {
            items
                .into_iter()
                .find(|item| !item.disabled.get_untracked())
        } else {
            items
                .into_iter()
                .rev()
                .find(|item| !item.disabled.get_untracked())
        };

        if let Some(item) = target {
            self.select(item.value.clone());
            self.focus_item(&item.value);
        }
    }
}
