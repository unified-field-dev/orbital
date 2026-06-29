mod tab;

pub use tab::BaseTab;

use leptos::{context::Provider, prelude::*};
use std::collections::HashMap;

#[component]
pub fn BaseTabList(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] selected_value: Option<RwSignal<String>>,
    children: Children,
) -> impl IntoView {
    let selected_value = selected_value.unwrap_or_else(|| RwSignal::new(String::new()));
    let registered_tabs = RwSignal::new(HashMap::new());

    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-tab-list".to_string()
        } else {
            format!("orbital-tab-list {extra}")
        }
    });

    view! {
        <Provider value=TabListInjection {
            previous_selected_value: StoredValue::new(selected_value.get_untracked()),
            selected_value,
            registered_tabs,
        }>
            <div class=root_class role="tablist">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TabListInjection {
    pub previous_selected_value: StoredValue<String>,
    pub selected_value: RwSignal<String>,
    pub registered_tabs: RwSignal<HashMap<String, TabRegisterData>>,
}

impl TabListInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn register(&self, data: TabRegisterData) {
        self.registered_tabs.update(|map| {
            map.insert(data.value.clone(), data);
        });
    }

    pub fn unregister(&self, value: &String) {
        self.registered_tabs.update(|map| {
            map.remove(value);
        });
    }

    pub fn on_select(&self, value: String) {
        self.previous_selected_value
            .set_value(self.selected_value.get_untracked());
        self.selected_value.set(value);
    }
}

pub(crate) struct TabRegisterData {
    pub value: String,
    pub tab_ref: NodeRef<leptos::html::Button>,
}
