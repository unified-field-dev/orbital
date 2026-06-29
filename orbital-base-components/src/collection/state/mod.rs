mod cascade;
mod expansion;
mod focus;
mod keyboard;
mod registry;
mod selection;

pub use cascade::{
    apply_cascade, checkbox_state, shift_range_in_visible, typeahead_match, CollectionCheckboxState,
};
pub use expansion::{CollectionExpansionState, ExpansionTrigger};
pub use focus::{CollectionFocusState, CollectionItemDomRegistry};
pub use keyboard::{collection_keyboard_action, CollectionKeyboardAction};
pub use registry::{CollectionRegistry, CollectionRegistryEntry};
pub use selection::{CollectionSelectionMode, CollectionSelectionState};

use leptos::{ev, prelude::*};
use std::collections::HashSet;

use crate::signals::SignalModel;
use crate::Handler;

/// DOM/role-agnostic collection behavior state shared by Tree and Navigation.
#[derive(Clone)]
pub struct CollectionState {
    pub expansion: CollectionExpansionState,
    pub selection: CollectionSelectionState,
    pub focus: CollectionFocusState,
    pub registry: CollectionRegistry,
    pub dom_registry: CollectionItemDomRegistry,
    pub disabled_items: SignalModel<HashSet<String>>,
    pub disabled_items_focusable: Signal<bool>,
    pub on_item_click: Option<Handler<(String, ev::MouseEvent)>>,
}

impl CollectionState {
    pub fn is_item_disabled(&self, item_id: &str) -> bool {
        self.disabled_items.with(|items| items.contains(item_id))
    }

    pub fn is_item_open(&self, item_id: &str) -> bool {
        self.expansion.is_open(item_id)
    }

    pub fn is_item_selected(&self, item_id: &str) -> bool {
        self.selection.is_selected(item_id)
    }

    pub fn visible_ordered_ids(&self) -> Vec<String> {
        let expansion = self.expansion.clone();
        self.registry
            .visible_ordered_ids(move |id| expansion.is_open(id))
    }
}

#[derive(Clone)]
pub struct CollectionStateInjection(pub CollectionState);

impl CollectionStateInjection {
    pub fn expect_context() -> CollectionState {
        expect_context::<Self>().0
    }

    pub fn use_context() -> Option<CollectionState> {
        use_context::<Self>().map(|injection| injection.0)
    }
}
