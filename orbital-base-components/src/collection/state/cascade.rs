use super::registry::CollectionRegistry;
use super::selection::{CollectionSelectionMode, CollectionSelectionState};

/// Checkbox tri-state for cascade selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionCheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

pub fn checkbox_state(
    registry: &CollectionRegistry,
    selection: &CollectionSelectionState,
    item_id: &str,
) -> CollectionCheckboxState {
    if selection.is_selected(item_id) {
        return CollectionCheckboxState::Checked;
    }

    let descendants = registry.descendant_ids(item_id);
    if descendants.is_empty() {
        return CollectionCheckboxState::Unchecked;
    }

    let selected_count = descendants
        .iter()
        .filter(|id| selection.is_selected(id))
        .count();

    if selected_count == 0 {
        CollectionCheckboxState::Unchecked
    } else {
        CollectionCheckboxState::Indeterminate
    }
}

pub fn apply_cascade(
    registry: &CollectionRegistry,
    selection: &CollectionSelectionState,
    item_id: &str,
    selected: bool,
) {
    if selection.mode != CollectionSelectionMode::Checkbox || !selection.cascade {
        return;
    }

    let descendants = registry.descendant_ids(item_id);
    selection.selected_items.update(|set| {
        if selected {
            set.insert(item_id.to_string());
            for id in &descendants {
                set.insert(id.clone());
            }
        } else {
            set.remove(item_id);
            for id in &descendants {
                set.remove(id);
            }
        }
    });

    update_ancestors(registry, selection, item_id);
}

fn update_ancestors(
    registry: &CollectionRegistry,
    selection: &CollectionSelectionState,
    item_id: &str,
) {
    for ancestor_id in registry.ancestor_ids(item_id) {
        let children = registry.ordered_children_ids(&ancestor_id);
        if children.is_empty() {
            continue;
        }
        let all_selected = children.iter().all(|id| selection.is_selected(id));
        selection.selected_items.update(|set| {
            if all_selected {
                set.insert(ancestor_id.clone());
            } else {
                set.remove(&ancestor_id);
            }
        });
    }
}

pub fn shift_range_in_visible(
    visible: &[String],
    anchor: &str,
    item_id: &str,
) -> Option<Vec<String>> {
    let start = visible.iter().position(|id| id == anchor)?;
    let end = visible.iter().position(|id| id == item_id)?;
    let (from, to) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };
    Some(visible[from..=to].to_vec())
}

pub fn typeahead_match(
    registry: &CollectionRegistry,
    visible: &[String],
    prefix: &str,
    start_after: Option<&str>,
) -> Option<String> {
    let lower = prefix.to_lowercase();
    let start_index = start_after
        .and_then(|id| visible.iter().position(|v| v == id))
        .map(|i| i + 1)
        .unwrap_or(0);

    for id in visible
        .iter()
        .skip(start_index)
        .chain(visible.iter().take(start_index))
    {
        if registry
            .get_entry(id)
            .is_some_and(|entry| entry.label.to_lowercase().starts_with(&lower))
        {
            return Some(id.clone());
        }
    }
    None
}
