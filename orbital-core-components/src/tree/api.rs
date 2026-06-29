use leptos::prelude::*;
use orbital_base_components::{
    apply_sibling_dom_reorder, install_tree_drag_listeners, TreeDropPosition, TreeExpansionState,
    TreeFocusState, TreeItemDomRegistry, TreeItemRegistry, TreeSelectionState, TreeState,
};

use super::types::{TreeAppearance, TreeBehavior, TreeExpansion, TreeSelection};

#[derive(Clone, Default)]
pub struct TreeApiRef(RwSignal<Option<TreeApiHandle>>);

#[derive(Clone)]
pub struct TreeApiHandle {
    state: TreeState,
}

impl TreeApiRef {
    pub fn new() -> Self {
        Self(RwSignal::new(None))
    }

    pub fn load(&self, handle: TreeApiHandle) {
        self.0.set(Some(handle));
    }

    pub fn with<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&TreeApiHandle) -> R,
    {
        self.0.with(|handle| handle.as_ref().map(f))
    }
}

impl TreeApiHandle {
    pub fn from_state(state: TreeState) -> Self {
        Self { state }
    }

    pub fn get_item(&self, item_id: &str) -> Option<orbital_base_components::TreeRegistryEntry> {
        self.state.registry.get_entry(item_id)
    }

    pub fn get_item_dom_element(&self, item_id: &str) -> Option<web_sys::HtmlElement> {
        self.state.dom_registry.get_dom_element(item_id)
    }

    pub fn get_item_tree(&self) -> Vec<(String, Vec<String>)> {
        self.state.registry.item_tree()
    }

    pub fn get_item_ordered_children_ids(&self, item_id: &str) -> Vec<String> {
        self.state.registry.ordered_children_ids(item_id)
    }

    pub fn select_item(
        &self,
        item_id: String,
        keep_existing: bool,
        should_be_selected: Option<bool>,
        shift_range: Option<Vec<String>>,
    ) {
        self.state.selection.select_item(
            item_id,
            keep_existing,
            should_be_selected,
            shift_range,
            Some(&self.state.registry),
        );
    }

    pub fn set_item_expansion(&self, item_id: String, expanded: bool) {
        self.state.expansion.set_expansion(item_id, expanded);
    }

    pub fn focus_item(&self, item_id: String) {
        self.state.focus.focus_item(item_id.clone());
        self.state.dom_registry.focus_dom_element(&item_id);
    }

    pub fn update_item_label(&self, item_id: String, label: String) {
        self.state.registry.update_label(&item_id, label);
    }
}

pub fn build_tree_state(
    expansion: TreeExpansion,
    selection: TreeSelection,
    behavior: TreeBehavior,
    appearance: TreeAppearance,
) -> TreeState {
    let mut expansion_state = TreeExpansionState::new(expansion.open_items)
        .with_default_open(expansion.default_open)
        .with_expansion_trigger(expansion.expansion_trigger);
    if let Some(handler) = expansion.on_expansion_toggle {
        expansion_state = expansion_state.with_on_expansion_toggle(handler);
    }

    let mut selection_state = TreeSelectionState::new(selection.mode, selection.selected_items)
        .with_default_selected(selection.default_selected)
        .with_cascade(selection.cascade);
    if let Some(handler) = selection.on_select {
        selection_state = selection_state.with_on_select(handler);
    }
    if let Some(handler) = selection.on_selection_toggle {
        selection_state = selection_state.with_on_selection_toggle(handler);
    }

    TreeState {
        expansion: expansion_state,
        selection: selection_state,
        focus: TreeFocusState::new(),
        registry: TreeItemRegistry::new(),
        dom_registry: TreeItemDomRegistry::new(),
        size: appearance.size,
        disabled_items: behavior.disabled_items,
        disabled_items_focusable: behavior.disabled_items_focusable,
        editable: behavior.editable,
        editable_items: behavior.editable_items,
        reorderable: behavior.reorderable,
        on_item_click: behavior.on_item_click,
        on_label_change: behavior.on_label_change,
        on_reorder: behavior.on_reorder,
        drag_state: orbital_base_components::TreeDragState::new(),
    }
}

pub fn install_tree_reorder_listeners(
    state: TreeState,
) -> orbital_base_components::TreeDragListenerHandle {
    let reorderable = state.reorderable;
    let on_reorder = state.on_reorder.clone();
    let registry = state.registry.clone();
    let dom_registry = state.dom_registry.clone();
    let drag_state = state.drag_state.clone();
    install_tree_drag_listeners(
        drag_state,
        reorderable,
        Callback::new(
            move |(source_id, target_id, position): (String, String, TreeDropPosition)| {
                let order = match position {
                    TreeDropPosition::Before => 0,
                    TreeDropPosition::After => 1,
                };
                if let Some(handler) = &on_reorder {
                    handler.run((source_id.clone(), target_id.clone(), order));
                } else if registry.reorder_siblings(&source_id, &target_id, order) {
                    apply_sibling_dom_reorder(&dom_registry, &source_id, &target_id, order);
                }
            },
        ),
    )
}
