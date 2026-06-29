use std::collections::HashSet;

use super::node::RichTreeNode;
use crate::tree::types::RichTreeData;

#[derive(Clone)]
pub(crate) struct RichTreeCtx<T: Clone + Send + Sync + 'static>(pub RichTreeData<T>);

pub(crate) fn build_nodes<T: Clone + Send + Sync + 'static>(
    data: &RichTreeData<T>,
) -> Vec<RichTreeNode> {
    data.items
        .iter()
        .enumerate()
        .map(|(i, item)| map_item(data, item, i))
        .collect()
}

pub(crate) fn map_item<T: Clone + Send + Sync + 'static>(
    data: &RichTreeData<T>,
    item: &T,
    _order: usize,
) -> RichTreeNode {
    let children = (data.get_children)(item);
    let lazy = data.lazy_fetch.is_some() && children.is_empty();
    RichTreeNode {
        id: (data.get_id)(item),
        label: (data.get_label)(item),
        children: if lazy {
            vec![]
        } else {
            children
                .iter()
                .enumerate()
                .map(|(i, child)| map_item(data, child, i))
                .collect()
        },
        disabled: (data.is_disabled)(item),
        editable: (data.is_editable)(item),
        lazy,
    }
}

pub(crate) fn collect_editable(nodes: &[RichTreeNode]) -> HashSet<String> {
    let mut editable = HashSet::new();
    collect_editable_into(nodes, &mut editable);
    editable
}

fn collect_editable_into(nodes: &[RichTreeNode], editable: &mut HashSet<String>) {
    for node in nodes {
        if node.editable {
            editable.insert(node.id.clone());
        }
        collect_editable_into(&node.children, editable);
    }
}

pub(crate) fn merge_disabled(nodes: &[RichTreeNode], base: &HashSet<String>) -> HashSet<String> {
    let mut disabled = base.clone();
    collect_disabled_into(nodes, &mut disabled);
    disabled
}

fn collect_disabled_into(nodes: &[RichTreeNode], disabled: &mut HashSet<String>) {
    for node in nodes {
        if node.disabled {
            disabled.insert(node.id.clone());
        }
        collect_disabled_into(&node.children, disabled);
    }
}

pub(crate) fn merge_lazy_children(
    roots: &mut [RichTreeNode],
    parent_id: &str,
    children: Vec<RichTreeNode>,
) {
    for node in roots.iter_mut() {
        if node.id == parent_id {
            node.children = children;
            node.lazy = false;
            return;
        }
        merge_lazy_children(&mut node.children, parent_id, children.clone());
    }
}

pub(crate) fn update_node_label(roots: &mut [RichTreeNode], item_id: &str, label: &str) -> bool {
    for node in roots.iter_mut() {
        if node.id == item_id {
            node.label = label.to_string();
            return true;
        }
        if update_node_label(&mut node.children, item_id, label) {
            return true;
        }
    }
    false
}
