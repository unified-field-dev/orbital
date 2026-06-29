use leptos::prelude::*;
use orbital_base_components::{SignalModel, TreeSize as BaseTreeSize};
use std::collections::HashSet;

use super::{TreeAppearance, TreeExpansion};
use orbital_base_components::TreeSize;

/// Back-compat wrapper around [`TreeExpansion`] + [`TreeAppearance`].
#[derive(Clone)]
pub struct TreeConfig {
    pub open_items: SignalModel<HashSet<String>>,
    pub size: Signal<TreeSize>,
}

impl TreeConfig {
    pub fn new(open_items: SignalModel<HashSet<String>>) -> Self {
        Self {
            open_items,
            size: Signal::derive(BaseTreeSize::default),
        }
    }

    pub fn with_size(mut self, size: Signal<TreeSize>) -> Self {
        self.size = size;
        self
    }

    pub fn into_expansion(self) -> TreeExpansion {
        TreeExpansion::new(self.open_items)
    }

    pub fn into_appearance(self) -> TreeAppearance {
        TreeAppearance::new(self.size)
    }
}
