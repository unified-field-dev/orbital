use leptos::prelude::*;
use std::collections::HashSet;

use super::state::TreeSelectionMode;
use crate::signals::SignalModel;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TreeSize {
    Small,
    #[default]
    Medium,
}

impl TreeSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TreeItemType {
    #[default]
    Leaf,
    Branch,
}

impl TreeItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Leaf => "leaf",
            Self::Branch => "branch",
        }
    }
}

/// Legacy injection — prefer [`TreeStateInjection`].
#[derive(Clone)]
pub struct TreeInjection {
    pub open_items: SignalModel<HashSet<String>>,
}

impl TreeInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Clone)]
pub struct SubtreeInjection {
    pub level: usize,
    pub parent_id: Option<String>,
}

impl SubtreeInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn use_context() -> Option<Self> {
        use_context()
    }
}

#[derive(Clone)]
pub struct TreeItemInjection {
    pub open: Memo<bool>,
    pub item_type: TreeItemType,
    pub item_id: String,
    pub subtree_ref: NodeRef<leptos::html::Div>,
    pub selected: Memo<bool>,
    pub focused: Memo<bool>,
    pub disabled: Memo<bool>,
}

impl TreeItemInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TreeItemRenderMode {
    #[default]
    Default,
    Editing,
}

#[derive(Clone)]
pub struct TreeItemEditInjection {
    pub editing: RwSignal<bool>,
    pub draft_label: RwSignal<String>,
    pub label_override: RwSignal<Option<String>>,
    pub on_commit: Option<Callback<()>>,
}

impl TreeItemEditInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

/// Configuration for legacy [`BaseTree`](crate::tree::root::BaseTree).
#[derive(Clone)]
pub struct BaseTreeConfig {
    pub open_items: SignalModel<HashSet<String>>,
    pub size: Signal<TreeSize>,
    pub selected_items: SignalModel<HashSet<String>>,
    pub selection_mode: TreeSelectionMode,
    pub disabled_items: SignalModel<HashSet<String>>,
    pub disabled_items_focusable: Signal<bool>,
    pub editable: Signal<bool>,
    pub reorderable: Signal<bool>,
}

impl Default for BaseTreeConfig {
    fn default() -> Self {
        Self {
            open_items: SignalModel::new(HashSet::new()),
            size: Signal::from(TreeSize::default()),
            selected_items: SignalModel::new(HashSet::new()),
            selection_mode: TreeSelectionMode::default(),
            disabled_items: SignalModel::new(HashSet::new()),
            disabled_items_focusable: Signal::from(false),
            editable: Signal::from(false),
            reorderable: Signal::from(false),
        }
    }
}

impl BaseTreeConfig {
    pub fn new(open_items: SignalModel<HashSet<String>>) -> Self {
        Self {
            open_items,
            ..Default::default()
        }
    }

    pub fn with_size(mut self, size: Signal<TreeSize>) -> Self {
        self.size = size;
        self
    }

    pub fn with_selected_items(mut self, selected_items: SignalModel<HashSet<String>>) -> Self {
        self.selected_items = selected_items;
        self
    }

    pub fn with_selection_mode(mut self, selection_mode: TreeSelectionMode) -> Self {
        self.selection_mode = selection_mode;
        self
    }

    pub fn with_disabled_items(mut self, disabled_items: SignalModel<HashSet<String>>) -> Self {
        self.disabled_items = disabled_items;
        self
    }

    pub fn with_disabled_items_focusable(mut self, focusable: Signal<bool>) -> Self {
        self.disabled_items_focusable = focusable;
        self
    }

    pub fn with_editable(mut self, editable: Signal<bool>) -> Self {
        self.editable = editable;
        self
    }

    pub fn with_reorderable(mut self, reorderable: Signal<bool>) -> Self {
        self.reorderable = reorderable;
        self
    }
}
