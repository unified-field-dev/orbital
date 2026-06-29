use leptos::{context::Provider, either::Either, ev, html, prelude::*};
use wasm_bindgen::JsCast;

use crate::collection::primitives::{
    default_select_action, use_item_activation, use_item_keyboard, use_item_registration,
    use_item_state, BaseCollectionItem,
};
use crate::collection::state::CollectionSelectionMode;

use super::dnd::{use_tree_item_drag, TreeDropPosition};
use super::edit::TreeEditCommit;
use super::edit::{label_from_row, resolve_edit_label};
use super::state::TreeStateInjection;
use super::subtree::{BaseSubtree, BaseTreeCollapseSlot};
use super::types::{SubtreeInjection, TreeItemEditInjection, TreeItemInjection, TreeItemType};

#[slot]
pub struct BaseTreeItemRow {
    pub children: Children,
}

#[component]
pub fn BaseTreeItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] item_type: TreeItemType,
    #[prop(into, optional)] value: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(into, optional, default = String::new())] parent_id: String,
    #[prop(optional, default = 0_usize)] order: usize,
    base_tree_item_row: BaseTreeItemRow,
    #[prop(optional)] base_tree_collapse: Option<BaseTreeCollapseSlot>,
    #[prop(default = None)] children: Option<Children>,
) -> impl IntoView {
    let tree_state = TreeStateInjection::expect_context();
    let collection = tree_state.collection();
    let subtree_injection = SubtreeInjection::expect_context();

    let item_id = StoredValue::new(value.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()));
    let item_label = StoredValue::new(label.unwrap_or_default());
    let subtree_ref = NodeRef::<html::Div>::new();
    let row_ref = NodeRef::<html::Div>::new();
    let editing = RwSignal::new(false);
    let draft_label = RwSignal::new(String::new());
    let label_override = RwSignal::new(None::<String>);

    let resolved_type = item_type;
    let is_branch = resolved_type == TreeItemType::Branch;
    let resolved_parent_id = (!parent_id.is_empty())
        .then(|| parent_id.clone())
        .or(subtree_injection.parent_id.clone());

    use_item_registration(
        collection.clone(),
        item_id,
        item_label,
        label_override,
        resolved_parent_id.clone(),
        is_branch,
        subtree_injection.level,
        order,
        row_ref,
    );

    let signals = use_item_state(collection.clone(), item_id);

    let item_id_signal = Signal::derive(move || item_id.get_value());
    let (dragging, on_drag_pointerdown) = use_tree_item_drag(
        item_id_signal,
        tree_state.reorderable,
        tree_state.drag_state.clone(),
    );

    let commit_label = {
        let tree_state = tree_state.clone();
        let item_id = item_id;
        move || {
            if let Some(commit) = TreeEditCommit::commit(item_id.get_value(), draft_label, editing)
            {
                label_override.set(Some(commit.label.clone()));
                tree_state
                    .registry
                    .update_label(&commit.item_id, commit.label.clone());
                if let Some(handler) = &tree_state.on_label_change {
                    handler.run((commit.item_id, commit.label));
                }
            }
        }
    };

    let on_commit = Callback::new({
        let commit_label = commit_label.clone();
        move |_| commit_label()
    });

    let should_ignore_click = {
        let subtree_ref = subtree_ref;
        move |event: ev::MouseEvent| {
            let from_subtree = subtree_ref.with_untracked(|subtree| {
                if let Some(subtree) = subtree {
                    event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                        .map(|node| subtree.contains(Some(&node)))
                        .unwrap_or(false)
                } else {
                    false
                }
            });
            if from_subtree {
                return true;
            }
            event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|element| {
                    element
                        .closest(".orbital-tree-item-layout__aside, .orbital-tree-item-layout__checkbox, .orbital-tree-item-layout__drag-handle")
                        .ok()
                        .flatten()
                        .is_some()
                })
        }
    };

    let is_icon_container_click = move |event: ev::MouseEvent| {
        event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|element| {
                element
                    .closest(".orbital-tree-item-layout__expand-icon")
                    .ok()
                    .flatten()
                    .is_some()
            })
    };

    let on_click = use_item_activation(
        collection.clone(),
        item_id,
        is_branch,
        signals.clone(),
        should_ignore_click,
        is_icon_container_click,
    );

    let on_select = {
        let collection = collection.clone();
        let tree_state = tree_state.clone();
        let item_id = item_id;
        let row_ref = row_ref;
        move || {
            let id = item_id.get_value();
            if tree_state.is_item_editable(&id) {
                draft_label.set(resolve_edit_label(
                    &id,
                    &item_label,
                    &label_override,
                    tree_state.registry.get_entry(&id).map(|entry| entry.label),
                    &row_ref,
                ));
                editing.set(true);
            } else {
                default_select_action(&collection, &id, is_branch);
            }
        }
    };

    let on_keydown_base = use_item_keyboard(
        collection.clone(),
        item_id,
        is_branch,
        signals.clone(),
        on_select,
    );

    let on_keydown = {
        let commit_label = commit_label.clone();
        let tree_state = tree_state.clone();
        let row_ref = row_ref;
        move |event: ev::KeyboardEvent| {
            on_keydown_base(event.clone());
            if editing.get_untracked() {
                let id = item_id.get_value();
                let original = resolve_edit_label(
                    &id,
                    &item_label,
                    &label_override,
                    tree_state.registry.get_entry(&id).map(|entry| entry.label),
                    &row_ref,
                );
                match event.key().as_str() {
                    "Escape" => {
                        TreeEditCommit::cancel(draft_label, editing, &original);
                    }
                    "Enter" => commit_label(),
                    _ => {}
                }
            }
        }
    };

    let on_dblclick = Callback::new({
        let tree_state = tree_state.clone();
        let row_ref = row_ref;
        move |_| {
            if !tree_state.is_item_editable(&item_id.get_value())
                || signals.disabled.get_untracked()
            {
                return;
            }
            let id = item_id.get_value();
            draft_label.set(resolve_edit_label(
                &id,
                &item_label,
                &label_override,
                tree_state.registry.get_entry(&id).map(|entry| entry.label),
                &row_ref,
            ));
            editing.set(true);
        }
    });

    Effect::new({
        let tree_state = tree_state.clone();
        let item_id = item_id;
        let row_ref = row_ref;
        move |_| {
            row_ref.get();
            if !item_label.get_value().is_empty() {
                return;
            }
            if let Some(label) = label_from_row(&row_ref) {
                tree_state
                    .registry
                    .update_label(&item_id.get_value(), label);
            }
        }
    });

    let style = Signal::derive({
        let level = subtree_injection.level;
        move || format!("--orbital-tree-item--level: {level}")
    });

    let aria_selected = Signal::derive({
        let collection = collection.clone();
        move || {
            if collection.selection.mode == CollectionSelectionMode::None {
                None
            } else {
                Some(
                    if signals.selected.get() {
                        "true"
                    } else {
                        "false"
                    }
                    .to_string(),
                )
            }
        }
    });

    let aria_expanded = Signal::derive(move || {
        if is_branch {
            Some(if signals.open.get() { "true" } else { "false" }.to_string())
        } else {
            None
        }
    });

    let drag_state = tree_state.drag_state.clone();
    let tree_class = Signal::derive({
        let drag_state = drag_state.clone();
        move || {
            let mut parts = vec![
                "orbital-tree-item".to_string(),
                format!("orbital-tree-item--{}", resolved_type.as_str()),
            ];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            let item_id = item_id.get_value();
            if drag_state
                .drop_target_id
                .get()
                .is_some_and(|target_id| target_id == item_id)
            {
                match drag_state.drop_position.get() {
                    Some(TreeDropPosition::Before) => {
                        parts.push("orbital-tree-item--drop-before".to_string());
                    }
                    Some(TreeDropPosition::After) => {
                        parts.push("orbital-tree-item--drop-after".to_string());
                    }
                    None => {}
                }
            }
            parts.join(" ")
        }
    });

    view! {
        <BaseCollectionItem
            class=tree_class
            role="treeitem"
            base_class="orbital-tree-item"
            item_id=item_id.get_value()
            style=style
            signals=signals.clone()
            aria_expanded=aria_expanded
            aria_selected=aria_selected
            on_click=Callback::new(on_click)
            on_keydown=Callback::new(on_keydown)
            on_dblclick=on_dblclick
            on_pointerdown=Callback::new(move |ev| on_drag_pointerdown.run(ev))
            dragging=dragging
            row_ref=row_ref
        >
            <Provider value=TreeItemInjection {
                open: signals.open,
                item_type: resolved_type,
                item_id: item_id.get_value(),
                subtree_ref,
                selected: signals.selected,
                focused: signals.focused,
                disabled: signals.disabled,
            }>
                <Provider value=TreeItemEditInjection { editing, draft_label, label_override, on_commit: Some(on_commit) }>
                    {(base_tree_item_row.children)()}
                    {match resolved_type {
                        TreeItemType::Branch => {
                            let level = subtree_injection.level + 1;
                            Either::Left(match base_tree_collapse {
                                Some(collapse) => view! {
                                    <BaseSubtree
                                        level=level
                                        base_tree_collapse=collapse
                                        subtree_children=children
                                    />
                                }
                                .into_any(),
                                None => view! {
                                    <BaseSubtree level=level subtree_children=children />
                                }
                                .into_any(),
                            })
                        }
                        TreeItemType::Leaf => Either::Right(()),
                    }}
                </Provider>
            </Provider>
        </BaseCollectionItem>
    }
}
