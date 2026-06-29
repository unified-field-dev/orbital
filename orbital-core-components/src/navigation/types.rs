use leptos::prelude::*;
use orbital_base_components::{NavigationDensity, NavigationMode};

/// Selection, expansion, and layout settings for [`Navigation`](super::navigation::Navigation).
#[derive(Clone)]
pub struct NavigationConfig {
    pub selected_value: RwSignal<Option<String>>,
    pub selected_category_value: RwSignal<Option<String>>,
    pub open_categories: RwSignal<Vec<String>>,
    pub multiple: Signal<bool>,
    pub density: NavigationDensity,
    pub mode: NavigationMode,
    pub open: Signal<bool>,
    pub collapsible: bool,
    pub collapsed: RwSignal<bool>,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            selected_value: RwSignal::new(None),
            selected_category_value: RwSignal::new(None),
            open_categories: RwSignal::new(Vec::new()),
            multiple: Signal::from(true),
            density: NavigationDensity::Standard,
            mode: NavigationMode::Inline,
            open: Signal::from(true),
            collapsible: false,
            collapsed: RwSignal::new(false),
        }
    }
}

impl NavigationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_selected_value(mut self, selected_value: RwSignal<Option<String>>) -> Self {
        self.selected_value = selected_value;
        self
    }

    pub fn with_selected_category_value(
        mut self,
        selected_category_value: RwSignal<Option<String>>,
    ) -> Self {
        self.selected_category_value = selected_category_value;
        self
    }

    pub fn with_open_categories(mut self, open_categories: RwSignal<Vec<String>>) -> Self {
        self.open_categories = open_categories;
        self
    }

    pub fn with_multiple(mut self, multiple: Signal<bool>) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn with_density(mut self, density: NavigationDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_mode(mut self, mode: NavigationMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_open(mut self, open: Signal<bool>) -> Self {
        self.open = open;
        self
    }

    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn with_collapsed(mut self, collapsed: RwSignal<bool>) -> Self {
        self.collapsed = collapsed;
        self
    }
}
