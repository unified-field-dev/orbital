mod api;
mod item;
mod rich_tree;
mod slots;
mod styles;
mod tree;
mod types;

pub use api::{TreeApiHandle, TreeApiRef};
pub use item::TreeItem;
pub use orbital_base_components::{ExpansionTrigger, TreeItemType, TreeSelectionMode, TreeSize};
pub use rich_tree::RichTree;
pub use slots::{
    TreeItemAside, TreeItemCheckbox, TreeItemCollapse, TreeItemIconAfter, TreeItemIconBefore,
    TreeItemLabelInput, TreeItemLayout,
};
pub use tree::Tree;
pub use types::{
    RichTreeData, TreeAppearance, TreeBehavior, TreeConfig, TreeExpansion, TreeItemConfig,
    TreeSelection,
};

#[cfg(feature = "preview")]
pub use rich_tree::RICHTREE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use tree::TREE_PREVIEW_REGISTRATION;
