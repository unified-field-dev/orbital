use orbital_base_components::{ExpansionTrigger, SignalModel};
use std::collections::HashSet;

use orbital_base_components::Handler;

/// Expansion settings for [`Tree`] and [`RichTree`](super::super::rich_tree::RichTree).
#[derive(Clone)]
pub struct TreeExpansion {
    pub open_items: SignalModel<HashSet<String>>,
    pub default_open: Vec<String>,
    pub expansion_trigger: ExpansionTrigger,
    pub on_expansion_toggle: Option<Handler<(String, bool)>>,
}

impl TreeExpansion {
    pub fn new(open_items: SignalModel<HashSet<String>>) -> Self {
        Self {
            open_items,
            default_open: Vec::new(),
            expansion_trigger: ExpansionTrigger::default(),
            on_expansion_toggle: None,
        }
    }

    pub fn with_default_open(mut self, default_open: Vec<String>) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_expansion_trigger(mut self, trigger: ExpansionTrigger) -> Self {
        self.expansion_trigger = trigger;
        self
    }

    pub fn with_on_expansion_toggle(mut self, handler: Handler<(String, bool)>) -> Self {
        self.on_expansion_toggle = Some(handler);
        self
    }
}

impl Default for TreeExpansion {
    fn default() -> Self {
        Self::new(SignalModel::new(HashSet::new()))
    }
}
