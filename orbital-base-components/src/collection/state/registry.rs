use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CollectionRegistryEntry {
    pub id: String,
    pub label: String,
    pub parent_id: Option<String>,
    pub is_branch: bool,
    pub depth: usize,
    pub order: usize,
}

#[derive(Clone, Default)]
pub struct CollectionRegistry {
    pub entries: RwSignal<HashMap<String, CollectionRegistryEntry>>,
    pub ordered_ids: RwSignal<Vec<String>>,
}

impl CollectionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    fn normalized_parent_id(parent_id: &Option<String>) -> Option<&str> {
        parent_id.as_deref().filter(|id| !id.is_empty())
    }

    pub fn register(&self, entry: CollectionRegistryEntry) {
        let entry = CollectionRegistryEntry {
            parent_id: Self::normalized_parent_id(&entry.parent_id).map(str::to_string),
            ..entry
        };
        self.entries.update(|entries| {
            entries.insert(entry.id.clone(), entry.clone());
        });
        self.ordered_ids.update(|ordered| {
            if !ordered.iter().any(|id| id == &entry.id) {
                ordered.push(entry.id.clone());
            }
        });
    }

    pub fn unregister(&self, item_id: &str) {
        self.entries.update(|entries| {
            entries.remove(item_id);
        });
        self.ordered_ids.update(|ordered| {
            ordered.retain(|id| id != item_id);
        });
    }

    pub fn update_label(&self, item_id: &str, label: String) {
        self.entries.update(|entries| {
            if let Some(entry) = entries.get_mut(item_id) {
                entry.label = label;
            }
        });
    }

    pub fn get_entry(&self, item_id: &str) -> Option<CollectionRegistryEntry> {
        self.entries
            .with_untracked(|entries| entries.get(item_id).cloned())
    }

    pub fn ordered_children_ids(&self, parent_id: &str) -> Vec<String> {
        self.entries.with_untracked(|entries| {
            let mut children: Vec<_> = entries
                .values()
                .filter(|entry| Self::normalized_parent_id(&entry.parent_id) == Some(parent_id))
                .cloned()
                .collect();
            children.sort_by_key(|entry| entry.order);
            children.into_iter().map(|entry| entry.id).collect()
        })
    }

    pub fn root_ids(&self) -> Vec<String> {
        self.entries.with_untracked(|entries| {
            let mut roots: Vec<_> = entries
                .values()
                .filter(|entry| Self::normalized_parent_id(&entry.parent_id).is_none())
                .cloned()
                .collect();
            roots.sort_by_key(|entry| entry.order);
            roots.into_iter().map(|entry| entry.id).collect()
        })
    }

    /// Depth-first visible item ids respecting expansion state.
    pub fn visible_ordered_ids(&self, is_open: impl Fn(&str) -> bool) -> Vec<String> {
        let mut result = Vec::new();
        self.walk_visible(&mut result, None, &is_open);
        result
    }

    fn walk_visible(
        &self,
        result: &mut Vec<String>,
        parent_id: Option<&str>,
        is_open: &impl Fn(&str) -> bool,
    ) {
        let children = self.entries.with_untracked(|entries| {
            let mut children: Vec<_> = entries
                .values()
                .filter(|entry| Self::normalized_parent_id(&entry.parent_id) == parent_id)
                .cloned()
                .collect();
            children.sort_by_key(|entry| entry.order);
            children
        });

        for child in children {
            result.push(child.id.clone());
            if child.is_branch && is_open(&child.id) {
                self.walk_visible(result, Some(&child.id), is_open);
            }
        }
    }

    pub fn item_tree(&self) -> Vec<(String, Vec<String>)> {
        self.entries.with_untracked(|entries| {
            let mut roots: Vec<_> = entries
                .values()
                .filter(|entry| Self::normalized_parent_id(&entry.parent_id).is_none())
                .cloned()
                .collect();
            roots.sort_by_key(|entry| entry.order);
            roots
                .into_iter()
                .map(|entry| {
                    let children = self.ordered_children_ids(&entry.id);
                    (entry.id, children)
                })
                .collect()
        })
    }

    pub fn descendant_ids(&self, item_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        self.collect_descendants(item_id, &mut result);
        result
    }

    fn collect_descendants(&self, item_id: &str, result: &mut Vec<String>) {
        for child_id in self.ordered_children_ids(item_id) {
            result.push(child_id.clone());
            self.collect_descendants(&child_id, result);
        }
    }

    pub fn ancestor_ids(&self, item_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = self.get_entry(item_id).and_then(|e| e.parent_id);
        while let Some(id) = current {
            result.push(id.clone());
            current = self.get_entry(&id).and_then(|e| e.parent_id);
        }
        result
    }

    /// Reorder `source_id` relative to `target_id` among shared siblings. `order` is `0` for before the target and `1` for after.
    pub fn reorder_siblings(&self, source_id: &str, target_id: &str, order: usize) -> bool {
        if source_id == target_id {
            return false;
        }

        let parent_id = match (self.get_entry(source_id), self.get_entry(target_id)) {
            (Some(source), Some(target)) if source.parent_id == target.parent_id => {
                source.parent_id
            }
            _ => return false,
        };

        self.entries.update(|entries| {
            let mut sorted: Vec<(usize, String)> = entries
                .values()
                .filter(|entry| entry.parent_id == parent_id)
                .map(|entry| (entry.order, entry.id.clone()))
                .collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
            let mut ids: Vec<String> = sorted.into_iter().map(|(_, id)| id).collect();
            ids.retain(|id| id != source_id);
            let Some(target_index) = ids.iter().position(|id| id == target_id) else {
                return;
            };
            let insert_at = if order == 0 {
                target_index
            } else {
                target_index + 1
            };
            ids.insert(insert_at.min(ids.len()), source_id.to_string());
            for (index, id) in ids.iter().enumerate() {
                if let Some(entry) = entries.get_mut(id) {
                    entry.order = index;
                }
            }
        });

        true
    }
}
