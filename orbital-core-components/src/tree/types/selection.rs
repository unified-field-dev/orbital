use orbital_base_components::{SignalModel, TreeSelectionMode};
use std::collections::HashSet;

use orbital_base_components::Handler;

/// Selection settings for [`Tree`] and [`RichTree`](super::super::rich_tree::RichTree).
#[derive(Clone)]
pub struct TreeSelection {
    pub mode: TreeSelectionMode,
    pub selected_items: SignalModel<HashSet<String>>,
    pub default_selected: Vec<String>,
    pub cascade: bool,
    pub on_select: Option<Handler<HashSet<String>>>,
    pub on_selection_toggle: Option<Handler<(String, bool)>>,
}

impl TreeSelection {
    pub fn new(mode: TreeSelectionMode, selected_items: SignalModel<HashSet<String>>) -> Self {
        Self {
            mode,
            selected_items,
            default_selected: Vec::new(),
            cascade: false,
            on_select: None,
            on_selection_toggle: None,
        }
    }

    pub fn none() -> Self {
        Self::new(TreeSelectionMode::None, SignalModel::new(HashSet::new()))
    }

    pub fn single(selected_items: SignalModel<HashSet<String>>) -> Self {
        Self::new(TreeSelectionMode::Single, selected_items)
    }

    pub fn multi(selected_items: SignalModel<HashSet<String>>) -> Self {
        Self::new(TreeSelectionMode::Multi, selected_items)
    }

    pub fn checkbox(selected_items: SignalModel<HashSet<String>>) -> Self {
        Self::new(TreeSelectionMode::Checkbox, selected_items)
    }

    pub fn with_default_selected(mut self, default_selected: Vec<String>) -> Self {
        self.default_selected = default_selected;
        self
    }

    pub fn with_cascade(mut self, cascade: bool) -> Self {
        self.cascade = cascade;
        self
    }

    pub fn with_on_select(mut self, handler: Handler<HashSet<String>>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub fn with_on_selection_toggle(mut self, handler: Handler<(String, bool)>) -> Self {
        self.on_selection_toggle = Some(handler);
        self
    }
}

impl Default for TreeSelection {
    fn default() -> Self {
        Self::none()
    }
}
