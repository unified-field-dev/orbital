use leptos::{html, prelude::*};
use std::collections::HashMap;

const TYPEAHEAD_RESET_MS: f64 = 500.0;

#[derive(Clone)]
pub struct CollectionFocusState {
    pub focused_item: RwSignal<Option<String>>,
    pub typeahead: RwSignal<String>,
    pub typeahead_deadline: RwSignal<Option<f64>>,
}

impl Default for CollectionFocusState {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionFocusState {
    pub fn new() -> Self {
        Self {
            focused_item: RwSignal::new(None),
            typeahead: RwSignal::new(String::new()),
            typeahead_deadline: RwSignal::new(None),
        }
    }

    pub fn focus_item(&self, item_id: String) {
        self.focused_item.set(Some(item_id));
    }

    pub fn append_typeahead(&self, key: String) -> String {
        let now = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);
        let prefix = if self
            .typeahead_deadline
            .get_untracked()
            .is_some_and(|deadline| now > deadline)
        {
            String::new()
        } else {
            self.typeahead.get_untracked()
        };
        let next = format!("{prefix}{key}");
        self.typeahead.set(next.clone());
        self.typeahead_deadline.set(Some(now + TYPEAHEAD_RESET_MS));
        next
    }

    pub fn reset_typeahead(&self) {
        self.typeahead.set(String::new());
        self.typeahead_deadline.set(None);
    }
}

#[derive(Clone)]
pub struct CollectionItemDomRegistry {
    pub item_refs: RwSignal<HashMap<String, NodeRef<html::Div>>>,
}

impl Default for CollectionItemDomRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionItemDomRegistry {
    pub fn new() -> Self {
        Self {
            item_refs: RwSignal::new(HashMap::new()),
        }
    }

    pub fn register(&self, item_id: String, node_ref: NodeRef<html::Div>) {
        self.item_refs.update(|refs| {
            refs.insert(item_id, node_ref);
        });
    }

    pub fn unregister(&self, item_id: &str) {
        self.item_refs.update(|refs| {
            refs.remove(item_id);
        });
    }

    pub fn get_dom_element(&self, item_id: &str) -> Option<web_sys::HtmlElement> {
        self.item_refs.with_untracked(|refs| {
            refs.get(item_id)
                .and_then(|node_ref| node_ref.get_untracked())
                .map(|element| element.into())
        })
    }

    pub fn focus_dom_element(&self, item_id: &str) {
        if let Some(element) = self.get_dom_element(item_id) {
            let _ = element.focus();
        }
    }

    /// Item ids sorted by on-screen position for range selection.
    pub fn visible_ids_in_dom_order(&self) -> Vec<String> {
        self.item_refs.with_untracked(|refs| {
            let mut positioned: Vec<(String, f64, f64)> = refs
                .iter()
                .filter_map(|(id, node_ref)| {
                    node_ref.get_untracked().map(|element| {
                        let rect = element.get_bounding_client_rect();
                        (id.clone(), rect.top(), rect.left())
                    })
                })
                .collect();
            positioned.sort_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
                    .then_with(|| a.0.cmp(&b.0))
            });
            positioned.into_iter().map(|(id, _, _)| id).collect()
        })
    }
}
