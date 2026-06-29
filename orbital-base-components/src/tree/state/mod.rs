//! Tree-specific state extensions and backward-compatible aliases for collection state.

pub use crate::collection::state::{
    checkbox_state, collection_keyboard_action, CollectionCheckboxState, CollectionExpansionState,
    CollectionFocusState, CollectionItemDomRegistry, CollectionKeyboardAction, CollectionRegistry,
    CollectionRegistryEntry, CollectionSelectionMode, CollectionSelectionState, CollectionState,
    ExpansionTrigger,
};

pub type TreeCheckboxState = CollectionCheckboxState;
pub type TreeExpansionState = CollectionExpansionState;
pub type TreeFocusState = CollectionFocusState;
pub type TreeItemDomRegistry = CollectionItemDomRegistry;
pub type TreeKeyboardAction = CollectionKeyboardAction;
pub type TreeItemRegistry = CollectionRegistry;
pub type TreeRegistryEntry = CollectionRegistryEntry;
pub type TreeSelectionMode = CollectionSelectionMode;
pub type TreeSelectionState = CollectionSelectionState;

pub fn tree_keyboard_action(event: &leptos::ev::KeyboardEvent) -> TreeKeyboardAction {
    collection_keyboard_action(event)
}

use leptos::{ev, prelude::*};
use std::collections::HashSet;

use crate::signals::SignalModel;
use crate::tree::dnd::TreeDragState;
use crate::tree::types::TreeSize;
use crate::Handler;

/// Tree behavior state: shared collection core plus tree-only fields.
#[derive(Clone)]
pub struct TreeState {
    pub expansion: TreeExpansionState,
    pub selection: TreeSelectionState,
    pub focus: TreeFocusState,
    pub registry: TreeItemRegistry,
    pub dom_registry: TreeItemDomRegistry,
    pub size: Signal<TreeSize>,
    pub disabled_items: SignalModel<HashSet<String>>,
    pub disabled_items_focusable: Signal<bool>,
    pub editable: Signal<bool>,
    pub editable_items: SignalModel<HashSet<String>>,
    pub reorderable: Signal<bool>,
    pub on_item_click: Option<Handler<(String, ev::MouseEvent)>>,
    pub on_label_change: Option<Handler<(String, String)>>,
    pub on_reorder: Option<Handler<(String, String, usize)>>,
    pub drag_state: TreeDragState,
}

impl TreeState {
    pub fn from_collection(collection: CollectionState, tree_fields: TreeStateFields) -> Self {
        Self {
            expansion: collection.expansion,
            selection: collection.selection,
            focus: collection.focus,
            registry: collection.registry,
            dom_registry: collection.dom_registry,
            disabled_items: collection.disabled_items,
            disabled_items_focusable: collection.disabled_items_focusable,
            on_item_click: collection.on_item_click,
            size: tree_fields.size,
            editable: tree_fields.editable,
            editable_items: tree_fields.editable_items,
            reorderable: tree_fields.reorderable,
            on_label_change: tree_fields.on_label_change,
            on_reorder: tree_fields.on_reorder,
            drag_state: tree_fields.drag_state,
        }
    }

    pub fn collection(&self) -> CollectionState {
        CollectionState {
            expansion: self.expansion.clone(),
            selection: self.selection.clone(),
            focus: self.focus.clone(),
            registry: self.registry.clone(),
            dom_registry: self.dom_registry.clone(),
            disabled_items: self.disabled_items.clone(),
            disabled_items_focusable: self.disabled_items_focusable,
            on_item_click: self.on_item_click.clone(),
        }
    }

    pub fn is_item_disabled(&self, item_id: &str) -> bool {
        self.disabled_items.with(|items| items.contains(item_id))
    }

    pub fn is_item_editable(&self, item_id: &str) -> bool {
        if !self.editable.get() {
            return false;
        }
        self.editable_items
            .with(|items| items.is_empty() || items.contains(item_id))
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
pub struct TreeStateFields {
    pub size: Signal<TreeSize>,
    pub editable: Signal<bool>,
    pub editable_items: SignalModel<HashSet<String>>,
    pub reorderable: Signal<bool>,
    pub on_label_change: Option<Handler<(String, String)>>,
    pub on_reorder: Option<Handler<(String, String, usize)>>,
    pub drag_state: TreeDragState,
}

#[derive(Clone)]
pub struct TreeStateInjection(pub TreeState);

impl TreeStateInjection {
    pub fn expect_context() -> TreeState {
        expect_context::<Self>().0
    }

    pub fn use_context() -> Option<TreeState> {
        use_context::<Self>().map(|injection| injection.0)
    }
}
