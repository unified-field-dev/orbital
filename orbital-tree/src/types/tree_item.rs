//! Tree item model types (API shape only).

/// Tree item placeholder.
#[derive(Clone, Debug, Default)]
pub struct TreeItemModel {
    pub item_id: String,
    pub label: String,
}
