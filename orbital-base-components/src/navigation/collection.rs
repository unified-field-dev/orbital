use leptos::prelude::*;
use std::collections::HashSet;

use crate::collection::state::{
    CollectionExpansionState, CollectionFocusState, CollectionItemDomRegistry, CollectionRegistry,
    CollectionSelectionMode, CollectionSelectionState, CollectionState,
};
use crate::signals::SignalModel;

/// Build shared collection behavior state for navigation keyboard/focus/selection.
pub fn build_navigation_collection(
    open_categories: SignalModel<HashSet<String>>,
    selected_items: SignalModel<HashSet<String>>,
) -> CollectionState {
    CollectionState {
        expansion: CollectionExpansionState::new(open_categories),
        selection: CollectionSelectionState::new(CollectionSelectionMode::Single, selected_items),
        focus: CollectionFocusState::new(),
        registry: CollectionRegistry::new(),
        dom_registry: CollectionItemDomRegistry::new(),
        disabled_items: SignalModel::new(HashSet::new()),
        disabled_items_focusable: Signal::from(false),
        on_item_click: None,
    }
}

/// Keep a parent-owned `RwSignal<Vec<String>>` in sync with collection expansion ids.
pub fn sync_open_categories_to_vec(
    open_categories: &RwSignal<Vec<String>>,
    open_set: &SignalModel<HashSet<String>>,
) {
    let next: Vec<_> = open_set.get().into_iter().collect();
    if open_categories.get_untracked() != next {
        open_categories.set(next);
    }
}

/// Mirror parent-owned category open state into collection expansion.
pub fn sync_vec_to_open_categories(
    open_categories: &RwSignal<Vec<String>>,
    open_set: &SignalModel<HashSet<String>>,
) {
    open_set.set(open_categories.get().into_iter().collect());
}

/// Keep parent-owned single selection in sync with collection selection.
pub fn sync_selected_value_to_option(
    selected_value: &RwSignal<Option<String>>,
    selected_items: &SignalModel<HashSet<String>>,
) {
    let next = selected_items.get().into_iter().next();
    if selected_value.get_untracked().as_ref() != next.as_ref() {
        selected_value.set(next);
    }
}

pub fn sync_option_to_selected_value(
    selected_value: &RwSignal<Option<String>>,
    selected_items: &SignalModel<HashSet<String>>,
) {
    selected_items.set(selected_value.get().into_iter().collect());
}
