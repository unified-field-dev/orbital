use leptos::{ev, prelude::*};

use super::item_state::CollectionItemSignals;
use crate::collection::state::{
    shift_range_in_visible, CollectionSelectionMode, CollectionState, ExpansionTrigger,
};

pub fn use_item_activation(
    state: CollectionState,
    item_id: StoredValue<String>,
    is_branch: bool,
    signals: CollectionItemSignals,
    should_ignore_click: impl Fn(ev::MouseEvent) -> bool + 'static + Clone,
    is_icon_container_click: impl Fn(ev::MouseEvent) -> bool + 'static + Clone,
) -> impl Fn(ev::MouseEvent) + Clone {
    move |event: ev::MouseEvent| {
        if signals.disabled.get_untracked() {
            return;
        }
        if should_ignore_click(event.clone()) {
            return;
        }

        let id = item_id.get_value();
        if let Some(handler) = &state.on_item_click {
            handler.run((id.clone(), event.clone()));
        }

        let icon_click = is_icon_container_click(event.clone());
        let should_toggle = is_branch
            && match state.expansion.expansion_trigger {
                ExpansionTrigger::Row => true,
                ExpansionTrigger::IconContainer => icon_click,
            };

        if should_toggle {
            state.expansion.toggle(id.clone(), is_branch);
        }

        if state.selection.mode != CollectionSelectionMode::None {
            let keep_existing = event.ctrl_key() || event.meta_key();
            let shift_range = if event.shift_key() {
                let visible = {
                    let dom_order = state.dom_registry.visible_ids_in_dom_order();
                    if dom_order.len() >= 2 {
                        dom_order
                    } else {
                        state.visible_ordered_ids()
                    }
                };
                state
                    .selection
                    .last_selected()
                    .and_then(|anchor| shift_range_in_visible(&visible, &anchor, &id))
            } else {
                None
            };
            let should_be_selected = match state.selection.mode {
                CollectionSelectionMode::Single => Some(true),
                CollectionSelectionMode::Multi if !keep_existing && shift_range.is_none() => {
                    Some(true)
                }
                _ => None,
            };
            state.selection.select_item(
                id.clone(),
                keep_existing,
                should_be_selected,
                shift_range,
                Some(&state.registry),
            );
        }

        state.focus.focus_item(id);
        state.dom_registry.focus_dom_element(&item_id.get_value());
    }
}
