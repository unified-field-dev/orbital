use leptos::prelude::*;

use crate::collection::state::CollectionState;

#[derive(Clone)]
pub struct CollectionItemSignals {
    pub open: Memo<bool>,
    pub selected: Memo<bool>,
    pub focused: Memo<bool>,
    pub disabled: Memo<bool>,
}

pub fn use_item_state(
    state: CollectionState,
    item_id: StoredValue<String>,
) -> CollectionItemSignals {
    let open = Memo::new({
        let state = state.clone();
        let item_id = item_id;
        move |_| state.expansion.is_open(&item_id.get_value())
    });

    let selected = Memo::new({
        let state = state.clone();
        let item_id = item_id;
        move |_| state.selection.is_selected(&item_id.get_value())
    });

    let focused = Memo::new({
        let state = state.clone();
        let item_id = item_id;
        move |_| {
            state
                .focus
                .focused_item
                .get()
                .is_some_and(|focused| focused == item_id.get_value())
        }
    });

    let disabled = Memo::new({
        let state = state.clone();
        let item_id = item_id;
        move |_| state.is_item_disabled(&item_id.get_value())
    });

    CollectionItemSignals {
        open,
        selected,
        focused,
        disabled,
    }
}
