use std::collections::HashSet;

pub fn toggle_selection(selected: &mut HashSet<String>, id: &str, multiselect: bool) {
    if multiselect {
        if !selected.insert(id.to_string()) {
            selected.remove(id);
        }
    } else {
        selected.clear();
        selected.insert(id.to_string());
    }
}

/// Select a contiguous range between anchor and target in visible row order.
pub fn range_select(
    selected: &mut HashSet<String>,
    anchor_id: &str,
    target_id: &str,
    visible_ids: &[String],
    extend: bool,
) {
    let Some(anchor_idx) = visible_ids.iter().position(|id| id == anchor_id) else {
        if !extend {
            selected.clear();
        }
        selected.insert(target_id.to_string());
        return;
    };
    let Some(target_idx) = visible_ids.iter().position(|id| id == target_id) else {
        return;
    };

    let (start, end) = if anchor_idx <= target_idx {
        (anchor_idx, target_idx)
    } else {
        (target_idx, anchor_idx)
    };

    if !extend {
        selected.clear();
    }
    for id in &visible_ids[start..=end] {
        selected.insert(id.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_select_replaces_without_extend() {
        let ids: Vec<String> = (1..=5).map(|i| i.to_string()).collect();
        let mut selected = HashSet::from(["99".to_string()]);
        range_select(&mut selected, "1", "3", &ids, false);
        assert_eq!(selected.len(), 3);
        assert!(selected.contains("1"));
        assert!(selected.contains("2"));
        assert!(selected.contains("3"));
        assert!(!selected.contains("99"));
    }

    #[test]
    fn range_select_extends_with_ctrl() {
        let ids: Vec<String> = (1..=5).map(|i| i.to_string()).collect();
        let mut selected = HashSet::from(["5".to_string()]);
        range_select(&mut selected, "1", "3", &ids, true);
        assert!(selected.contains("5"));
        assert!(selected.contains("1"));
        assert!(selected.contains("3"));
    }

    #[test]
    fn range_select_reverse_order() {
        let ids: Vec<String> = (1..=5).map(|i| i.to_string()).collect();
        let mut selected = HashSet::new();
        range_select(&mut selected, "4", "2", &ids, false);
        assert_eq!(selected.len(), 3);
        for id in ["2", "3", "4"] {
            assert!(selected.contains(id));
        }
    }
}
