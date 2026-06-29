use leptos::prelude::*;
use std::collections::HashSet;

use crate::collection::state::CollectionState;
use crate::signals::SignalModel;

use super::collection::{
    build_navigation_collection, sync_open_categories_to_vec, sync_option_to_selected_value,
    sync_selected_value_to_option, sync_vec_to_open_categories,
};
use super::types::NavigationDensity;

#[derive(Clone)]
pub struct NavigationInjection {
    pub collection: CollectionState,
    open_categories: RwSignal<Vec<String>>,
    selected_value: RwSignal<Option<String>>,
    open_set: SignalModel<HashSet<String>>,
    selected_set: SignalModel<HashSet<String>>,
    pub selected_category_value: RwSignal<Option<String>>,
    pub multiple: Signal<bool>,
    pub density: Signal<NavigationDensity>,
    pub collapsed: RwSignal<bool>,
    pub open: Signal<bool>,
}

impl NavigationInjection {
    pub fn new(
        selected_value: RwSignal<Option<String>>,
        selected_category_value: RwSignal<Option<String>>,
        open_categories: RwSignal<Vec<String>>,
        multiple: Signal<bool>,
        density: Signal<NavigationDensity>,
        collapsed: RwSignal<bool>,
        open: Signal<bool>,
    ) -> Self {
        let open_set = SignalModel::new(open_categories.get_untracked().into_iter().collect());
        let selected_set = SignalModel::new(selected_value.get_untracked().into_iter().collect());
        let collection = build_navigation_collection(open_set.clone(), selected_set.clone());

        let injection = Self {
            collection,
            open_categories,
            selected_value,
            open_set,
            selected_set,
            selected_category_value,
            multiple,
            density,
            collapsed,
            open,
        };
        injection.install_sync_effects();
        injection
    }

    fn install_sync_effects(&self) {
        let open_categories = self.open_categories;
        let selected_value = self.selected_value;
        let open_set = self.open_set.clone();
        let selected_set = self.selected_set.clone();

        Effect::new(move |_| {
            sync_vec_to_open_categories(&open_categories, &open_set);
        });
        Effect::new(move |_| {
            sync_option_to_selected_value(&selected_value, &selected_set);
        });
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn selected_value(&self) -> RwSignal<Option<String>> {
        self.selected_value
    }

    pub fn open_categories(&self) -> RwSignal<Vec<String>> {
        self.open_categories
    }

    pub fn sync_selected_from_collection(&self) {
        sync_selected_value_to_option(&self.selected_value, &self.selected_set);
    }

    pub fn is_selected_category(&self, value: &str) -> bool {
        self.selected_category_value
            .with(|selected_category_value| selected_category_value.as_deref() == Some(value))
    }

    pub fn is_category_open(&self, value: &str) -> bool {
        self.collection.expansion.is_open(value)
    }

    pub fn on_request_category_toggle(&self, category_value: String) {
        let is_open = self.collection.expansion.is_open(&category_value);
        if self.multiple.get_untracked() {
            self.collection
                .expansion
                .set_expansion(category_value, !is_open);
        } else if is_open {
            self.collection
                .expansion
                .set_expansion(category_value.clone(), false);
        } else {
            self.open_set.update(|open| {
                open.clear();
                open.insert(category_value.clone());
            });
            self.collection
                .expansion
                .set_expansion(category_value, true);
        }
        sync_open_categories_to_vec(&self.open_categories, &self.open_set);
    }
}

#[derive(Clone, Copy)]
pub struct NavigationCategoryInjection {
    pub value: Signal<String>,
}

impl NavigationCategoryInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}

#[derive(Clone, Copy)]
pub struct NavigationSubItemGroupInjection;

impl NavigationSubItemGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
