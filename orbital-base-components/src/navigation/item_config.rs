use leptos::{ev, prelude::*};

/// Configuration for [`BaseNavigationItem`](super::item::BaseNavigationItem).
#[derive(Clone)]
pub struct BaseNavigationItemConfig {
    pub item_id: Signal<String>,
    pub label: Signal<String>,
    pub depth: usize,
    pub item_class: Signal<String>,
    pub item_style: Signal<Option<String>>,
    pub disabled: Signal<bool>,
    pub selected: Signal<bool>,
    pub on_user_click: Callback<ev::MouseEvent>,
}

impl BaseNavigationItemConfig {
    pub fn new(item_id: Signal<String>, on_user_click: Callback<ev::MouseEvent>) -> Self {
        Self {
            item_id,
            label: item_id,
            depth: 0,
            item_class: Signal::from(String::new()),
            item_style: Signal::from(None),
            disabled: Signal::from(false),
            selected: Signal::from(false),
            on_user_click,
        }
    }

    pub fn with_label(mut self, label: Signal<String>) -> Self {
        self.label = label;
        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_item_class(mut self, item_class: Signal<String>) -> Self {
        self.item_class = item_class;
        self
    }

    pub fn with_item_style(mut self, item_style: Signal<Option<String>>) -> Self {
        self.item_style = item_style;
        self
    }

    pub fn with_disabled(mut self, disabled: Signal<bool>) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_selected(mut self, selected: Signal<bool>) -> Self {
        self.selected = selected;
        self
    }
}
