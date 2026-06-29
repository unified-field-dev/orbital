use leptos::prelude::*;
use orbital_base_components::{Handler, SignalModel};
use std::collections::HashSet;

/// Interaction settings for [`Tree`] and [`RichTree`](super::super::rich_tree::RichTree).
#[derive(Clone)]
pub struct TreeBehavior {
    pub disabled_items: SignalModel<HashSet<String>>,
    pub disabled_items_focusable: Signal<bool>,
    pub editable: Signal<bool>,
    pub editable_items: SignalModel<HashSet<String>>,
    pub reorderable: Signal<bool>,
    pub on_item_click: Option<Handler<(String, leptos::ev::MouseEvent)>>,
    pub on_label_change: Option<Handler<(String, String)>>,
    pub on_reorder: Option<Handler<(String, String, usize)>>,
}

impl TreeBehavior {
    pub fn new() -> Self {
        Self {
            disabled_items: SignalModel::new(HashSet::new()),
            disabled_items_focusable: Signal::from(false),
            editable: Signal::from(false),
            editable_items: SignalModel::new(HashSet::new()),
            reorderable: Signal::from(false),
            on_item_click: None,
            on_label_change: None,
            on_reorder: None,
        }
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

    pub fn with_editable_items(mut self, editable_items: SignalModel<HashSet<String>>) -> Self {
        self.editable_items = editable_items;
        self
    }

    pub fn with_reorderable(mut self, reorderable: Signal<bool>) -> Self {
        self.reorderable = reorderable;
        self
    }

    pub fn with_on_item_click(
        mut self,
        handler: Handler<(String, leptos::ev::MouseEvent)>,
    ) -> Self {
        self.on_item_click = Some(handler);
        self
    }

    pub fn with_on_label_change(mut self, handler: Handler<(String, String)>) -> Self {
        self.on_label_change = Some(handler);
        self
    }

    pub fn with_on_reorder(mut self, handler: Handler<(String, String, usize)>) -> Self {
        self.on_reorder = Some(handler);
        self
    }
}

impl Default for TreeBehavior {
    fn default() -> Self {
        Self::new()
    }
}
