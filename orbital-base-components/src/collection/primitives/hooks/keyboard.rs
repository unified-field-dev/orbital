use leptos::{ev, prelude::*};

use super::item_state::CollectionItemSignals;
use crate::collection::state::{
    collection_keyboard_action, typeahead_match, CollectionKeyboardAction, CollectionSelectionMode,
    CollectionState,
};

pub fn use_item_keyboard(
    state: CollectionState,
    item_id: StoredValue<String>,
    is_branch: bool,
    signals: CollectionItemSignals,
    on_select: impl Fn() + 'static + Clone,
) -> impl Fn(ev::KeyboardEvent) + Clone {
    move |event: ev::KeyboardEvent| {
        if signals.disabled.get_untracked() && !state.disabled_items_focusable.get_untracked() {
            return;
        }

        let action = collection_keyboard_action(&event);
        let id = item_id.get_value();
        match action {
            CollectionKeyboardAction::Select => {
                event.prevent_default();
                on_select();
            }
            CollectionKeyboardAction::Expand => {
                if is_branch {
                    event.prevent_default();
                    state.expansion.set_expansion(id.clone(), true);
                }
            }
            CollectionKeyboardAction::Collapse => {
                if is_branch {
                    event.prevent_default();
                    state.expansion.set_expansion(id.clone(), false);
                }
            }
            CollectionKeyboardAction::Next
            | CollectionKeyboardAction::Previous
            | CollectionKeyboardAction::Home
            | CollectionKeyboardAction::End => {
                event.prevent_default();
                focus_relative_item(&state, &id, action);
            }
            CollectionKeyboardAction::TypeAhead => {
                event.prevent_default();
                typeahead_to_item(&state, event.key());
            }
            CollectionKeyboardAction::None => {}
        }
    }
}

pub fn default_select_action(state: &CollectionState, item_id: &str, is_branch: bool) {
    if state.selection.mode != CollectionSelectionMode::None {
        state.selection.select_item(
            item_id.to_string(),
            false,
            Some(true),
            None,
            Some(&state.registry),
        );
    } else if is_branch {
        state.expansion.toggle(item_id.to_string(), true);
    }
}

fn focus_relative_item(state: &CollectionState, item_id: &str, action: CollectionKeyboardAction) {
    let ordered = state.visible_ordered_ids();
    let Some(index) = ordered.iter().position(|id| id == item_id) else {
        return;
    };
    let next = match action {
        CollectionKeyboardAction::Next => (index + 1).min(ordered.len().saturating_sub(1)),
        CollectionKeyboardAction::Previous => index.saturating_sub(1),
        CollectionKeyboardAction::Home => 0,
        CollectionKeyboardAction::End => ordered.len().saturating_sub(1),
        _ => index,
    };
    if let Some(next_id) = ordered.get(next) {
        state.focus.focus_item(next_id.clone());
        state.dom_registry.focus_dom_element(next_id);
    }
}

fn typeahead_to_item(state: &CollectionState, key: String) {
    let prefix = state.focus.append_typeahead(key);
    let visible = state.visible_ordered_ids();
    let start_after = state.focus.focused_item.get_untracked();
    if let Some(next_id) =
        typeahead_match(&state.registry, &visible, &prefix, start_after.as_deref())
    {
        state.focus.focus_item(next_id.clone());
        state.dom_registry.focus_dom_element(&next_id);
    }
}
