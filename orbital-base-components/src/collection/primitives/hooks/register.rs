use leptos::{html, prelude::*};

use crate::collection::state::{CollectionRegistryEntry, CollectionState};

pub fn use_item_registration(
    state: CollectionState,
    item_id: StoredValue<String>,
    item_label: StoredValue<String>,
    label_override: RwSignal<Option<String>>,
    parent_id: Option<String>,
    is_branch: bool,
    depth: usize,
    order: usize,
    row_ref: NodeRef<html::Div>,
) {
    Effect::new({
        let state = state.clone();
        move |_| {
            state.registry.register(CollectionRegistryEntry {
                id: item_id.get_value(),
                label: label_override
                    .get()
                    .or_else(|| Some(item_label.get_value()))
                    .unwrap_or_default(),
                parent_id: parent_id.clone(),
                is_branch,
                depth,
                order,
            });
            state.dom_registry.register(item_id.get_value(), row_ref);
        }
    });

    on_cleanup({
        let state = state.clone();
        move || {
            let id = item_id.get_value();
            state.registry.unregister(&id);
            state.dom_registry.unregister(&id);
        }
    });
}
