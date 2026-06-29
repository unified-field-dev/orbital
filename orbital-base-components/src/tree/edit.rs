use leptos::{html, prelude::*};

#[derive(Clone)]
pub struct TreeEditCommit {
    pub item_id: String,
    pub label: String,
}

impl TreeEditCommit {
    pub fn commit(
        item_id: String,
        draft: RwSignal<String>,
        editing: RwSignal<bool>,
    ) -> Option<Self> {
        if !editing.get_untracked() {
            return None;
        }
        let label = draft.get_untracked();
        editing.set(false);
        Some(Self { item_id, label })
    }

    pub fn cancel(draft: RwSignal<String>, editing: RwSignal<bool>, original: &str) {
        draft.set(original.to_string());
        editing.set(false);
    }
}

/// Read the rendered label text when config/registry labels are empty.
pub fn label_from_row(row_ref: &NodeRef<html::Div>) -> Option<String> {
    row_ref.get_untracked().and_then(|row| {
        row.query_selector(".orbital-tree-item-layout__label")
            .ok()
            .flatten()
            .and_then(|element| element.text_content())
            .map(|text| text.trim().to_string())
            .filter(|text| !text.is_empty())
    })
}

pub fn resolve_edit_label(
    _item_id: &str,
    item_label: &StoredValue<String>,
    label_override: &RwSignal<Option<String>>,
    registry_label: Option<String>,
    row_ref: &NodeRef<html::Div>,
) -> String {
    if let Some(label) = label_override.get_untracked() {
        return label;
    }
    let stored = item_label.get_value();
    if !stored.is_empty() {
        return stored;
    }
    if let Some(label) = registry_label.filter(|label| !label.is_empty()) {
        return label;
    }
    label_from_row(row_ref).unwrap_or_default()
}
