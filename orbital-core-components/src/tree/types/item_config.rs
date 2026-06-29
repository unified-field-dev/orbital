/// Configuration for [`TreeItem`](crate::tree::item::TreeItem).
#[derive(Clone, Default)]
pub struct TreeItemConfig {
    pub item_type: TreeItemType,
    pub value: String,
    pub label: Option<String>,
    pub parent_id: Option<String>,
    pub order: usize,
}

impl TreeItemConfig {
    pub fn branch(value: impl Into<String>) -> Self {
        Self {
            item_type: TreeItemType::Branch,
            value: value.into(),
            label: None,
            parent_id: None,
            order: 0,
        }
    }

    pub fn leaf(value: impl Into<String>) -> Self {
        Self {
            item_type: TreeItemType::Leaf,
            value: value.into(),
            label: None,
            parent_id: None,
            order: 0,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

pub use orbital_base_components::TreeItemType;
