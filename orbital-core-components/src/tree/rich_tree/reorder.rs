use super::node::RichTreeNode;

pub(crate) fn reorder_nodes(
    roots: &mut Vec<RichTreeNode>,
    source_id: &str,
    target_id: &str,
    order: usize,
) {
    let Some((node, parent_id)) = extract_node(roots, source_id) else {
        return;
    };
    match parent_id {
        Some(parent_id) => {
            let _ = insert_under_parent(roots, &parent_id, node, target_id, order);
        }
        None => {
            if let Some(index) = roots.iter().position(|n| n.id == target_id) {
                let insert_at = if order == 0 { index } else { index + 1 };
                roots.insert(insert_at.min(roots.len()), node);
            }
        }
    }
}

fn insert_under_parent(
    nodes: &mut [RichTreeNode],
    parent_id: &str,
    node: RichTreeNode,
    target_id: &str,
    order: usize,
) -> Result<(), RichTreeNode> {
    for child in nodes.iter_mut() {
        if child.id == parent_id {
            insert_relative(&mut child.children, node, target_id, order);
            return Ok(());
        }
    }
    let mut node = node;
    for child in nodes.iter_mut() {
        match insert_under_parent(&mut child.children, parent_id, node, target_id, order) {
            Ok(()) => return Ok(()),
            Err(returned) => node = returned,
        }
    }
    Err(node)
}

#[allow(clippy::ptr_arg)]
fn extract_node(nodes: &mut Vec<RichTreeNode>, id: &str) -> Option<(RichTreeNode, Option<String>)> {
    if let Some(index) = nodes.iter().position(|node| node.id == id) {
        return Some((nodes.remove(index), None));
    }
    for node in nodes.iter_mut() {
        if let Some(index) = node.children.iter().position(|child| child.id == id) {
            let removed = node.children.remove(index);
            return Some((removed, Some(node.id.clone())));
        }
        if let Some(found) = extract_node(&mut node.children, id) {
            return Some(found);
        }
    }
    None
}

fn insert_relative(
    siblings: &mut Vec<RichTreeNode>,
    node: RichTreeNode,
    target_id: &str,
    order: usize,
) {
    if let Some(index) = siblings.iter().position(|sibling| sibling.id == target_id) {
        let insert_at = if order == 0 { index } else { index + 1 };
        siblings.insert(insert_at.min(siblings.len()), node);
    }
}
