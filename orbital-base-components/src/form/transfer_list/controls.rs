use std::collections::HashSet;

use super::types::TransferListItem;

pub fn move_checked(
    source: &mut Vec<TransferListItem>,
    target: &mut Vec<TransferListItem>,
    checked: &HashSet<String>,
) {
    let mut moving = Vec::new();
    source.retain(|item| {
        if checked.contains(&item.id) && !item.disabled {
            moving.push(item.clone());
            false
        } else {
            true
        }
    });
    target.extend(moving);
}

pub fn move_all(source: &mut Vec<TransferListItem>, target: &mut Vec<TransferListItem>) {
    let mut moving = Vec::new();
    source.retain(|item| {
        if item.disabled {
            true
        } else {
            moving.push(item.clone());
            false
        }
    });
    target.extend(moving);
}

pub fn selectable_ids(items: &[TransferListItem]) -> HashSet<String> {
    items
        .iter()
        .filter(|item| !item.disabled)
        .map(|item| item.id.clone())
        .collect()
}

pub fn toggle_all(items: &[TransferListItem], checked: &mut HashSet<String>, select: bool) {
    if select {
        checked.extend(selectable_ids(items));
    } else {
        checked.clear();
    }
}

pub fn selected_count(items: &[TransferListItem], checked: &HashSet<String>) -> usize {
    items
        .iter()
        .filter(|item| checked.contains(&item.id))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, disabled: bool) -> TransferListItem {
        TransferListItem {
            id: id.to_string(),
            label: id.to_string(),
            disabled,
        }
    }

    #[test]
    fn move_all_leaves_disabled_rows_in_source() {
        let mut left = vec![item("open", false), item("locked", true)];
        let mut right = Vec::new();
        move_all(&mut left, &mut right);
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].id, "locked");
        assert_eq!(right.len(), 1);
        assert_eq!(right[0].id, "open");
    }

    #[test]
    fn move_checked_moves_only_selected_enabled_rows() {
        let mut left = vec![item("a", false), item("b", true), item("c", false)];
        let mut right = Vec::new();
        let checked = HashSet::from(["a".to_string(), "b".to_string()]);
        move_checked(&mut left, &mut right, &checked);
        assert_eq!(left.len(), 2);
        assert_eq!(left[0].id, "b");
        assert_eq!(left[1].id, "c");
        assert_eq!(right.len(), 1);
        assert_eq!(right[0].id, "a");
    }
}
