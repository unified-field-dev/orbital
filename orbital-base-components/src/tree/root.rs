use leptos::{context::Provider, prelude::*};
use std::collections::HashSet;

use crate::collection::primitives::BaseCollectionRoot;

use super::dnd::TreeDragState;
use super::state::{
    TreeExpansionState, TreeFocusState, TreeItemDomRegistry, TreeItemRegistry, TreeSelectionState,
    TreeState, TreeStateInjection,
};
use super::types::{BaseTreeConfig, SubtreeInjection};
use crate::signals::SignalModel;

#[component]
pub fn BaseTreeRoot(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] tree_state: TreeState,
    children: Children,
) -> impl IntoView {
    let size = tree_state.size;
    let collection_state = tree_state.collection();
    let tree_state_for_provider = tree_state.clone();

    view! {
        <BaseCollectionRoot
            role="tree"
            base_class="orbital-tree"
            class=Signal::derive(move || {
                let mut parts = vec![format!("orbital-tree--{}", size.get().as_str())];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            })
            collection_state=collection_state
        >
            <Provider value=TreeStateInjection(tree_state_for_provider)>
                <Provider value=SubtreeInjection { level: 1, parent_id: None }>{children()}</Provider>
            </Provider>
        </BaseCollectionRoot>
    }
}

/// Simple tree root for legacy callers.
#[component]
pub fn BaseTree(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] config: BaseTreeConfig,
    children: Children,
) -> impl IntoView {
    let BaseTreeConfig {
        open_items,
        size,
        selected_items,
        selection_mode,
        disabled_items,
        disabled_items_focusable,
        editable,
        reorderable,
    } = config;

    let tree_state = TreeState {
        expansion: TreeExpansionState::new(open_items),
        selection: TreeSelectionState::new(selection_mode, selected_items),
        focus: TreeFocusState::new(),
        registry: TreeItemRegistry::new(),
        dom_registry: TreeItemDomRegistry::new(),
        size,
        disabled_items,
        disabled_items_focusable,
        editable,
        editable_items: SignalModel::new(HashSet::new()),
        reorderable,
        on_item_click: None,
        on_label_change: None,
        on_reorder: None,
        drag_state: TreeDragState::new(),
    };

    view! {
        <BaseTreeRoot class=class tree_state=tree_state>{children()}</BaseTreeRoot>
    }
}
