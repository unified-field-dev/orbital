use leptos::prelude::*;
use orbital_base_components::TreeSize;

/// Visual settings for [`Tree`] and [`RichTree`](super::super::rich_tree::RichTree).
#[derive(Clone)]
pub struct TreeAppearance {
    pub size: Signal<TreeSize>,
    pub connectors: Signal<bool>,
}

impl TreeAppearance {
    pub fn new(size: Signal<TreeSize>) -> Self {
        Self {
            size,
            connectors: Signal::from(false),
        }
    }

    pub fn with_connectors(mut self, connectors: Signal<bool>) -> Self {
        self.connectors = connectors;
        self
    }
}

impl Default for TreeAppearance {
    fn default() -> Self {
        Self {
            size: Signal::derive(TreeSize::default),
            connectors: Signal::from(false),
        }
    }
}
