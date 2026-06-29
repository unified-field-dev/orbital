use crate::{DiscussionFocus, DiscussionReply, DiscussionReplyGraph, DiscussionViewMode};

/// Lookup a reply by id in the graph index.
pub fn select_reply_by_id<'a>(
    graph: &'a DiscussionReplyGraph,
    id: &str,
) -> Option<&'a DiscussionReply> {
    graph.get(id)
}

/// Depth-first visible subtree rooted at `id`, respecting focus and view mode.
///
/// In Tree mode, returns the anchor reply followed by descendants visible under
/// the current focus. In Flat/Compact mode, returns the flat projection filtered
/// to replies that are the root or descendants of `id`.
pub fn select_visible_branch<'a>(
    graph: &'a DiscussionReplyGraph,
    id: &str,
    focus: &DiscussionFocus,
    mode: DiscussionViewMode,
) -> Vec<&'a DiscussionReply> {
    let Some(root) = graph.get(id) else {
        return Vec::new();
    };

    match mode {
        DiscussionViewMode::Tree => {
            let mut out = vec![root];
            collect_tree_descendants(graph, id, focus, &mut out);
            out
        }
        DiscussionViewMode::Flat | DiscussionViewMode::Compact => graph
            .visible_replies(focus, mode, crate::DiscussionSort::OldestFirst)
            .into_iter()
            .filter(|reply| reply.id == id || graph.is_descendant_of(&reply.id, id))
            .collect(),
    }
}

fn collect_tree_descendants<'a>(
    graph: &'a DiscussionReplyGraph,
    parent_id: &str,
    focus: &DiscussionFocus,
    out: &mut Vec<&'a DiscussionReply>,
) {
    for child_id in graph.child_ids(parent_id) {
        if graph.depth_in_view(child_id, focus) == u32::MAX {
            continue;
        }
        if let Some(child) = graph.get(child_id) {
            out.push(child);
            collect_tree_descendants(graph, child_id, focus, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DiscussionAuthor, DiscussionMetadata, DiscussionPart, DiscussionReply,
        DiscussionReplyStatus,
    };
    use chrono::Utc;

    fn reply(id: &str, parent_id: Option<&str>, text: &str) -> DiscussionReply {
        DiscussionReply {
            id: id.to_string(),
            parent_id: parent_id.map(str::to_string),
            author: DiscussionAuthor::new(id, id),
            metadata: DiscussionMetadata {
                created_at: Utc::now(),
                edited_at: None,
                labels: vec![],
            },
            parts: vec![DiscussionPart::text(text)],
            citations: vec![],
            status: DiscussionReplyStatus::Ready,
            hidden_child_count: 0,
        }
    }

    fn sample_tree() -> DiscussionReplyGraph {
        DiscussionReplyGraph::from_flat(&[
            reply("r-root", None, "root"),
            reply("r-sam", Some("r-root"), "sam"),
            reply("r-jordan", Some("r-sam"), "jordan"),
            reply("r-pat", Some("r-jordan"), "pat"),
        ])
    }

    #[test]
    fn select_reply_by_id_finds_reply() {
        let graph = sample_tree();
        assert_eq!(
            select_reply_by_id(&graph, "r-sam").map(|r| r.id.as_str()),
            Some("r-sam")
        );
        assert!(select_reply_by_id(&graph, "missing").is_none());
    }

    #[test]
    fn select_visible_branch_tree_order() {
        let graph = sample_tree();
        let branch = select_visible_branch(
            &graph,
            "r-sam",
            &DiscussionFocus::Root,
            DiscussionViewMode::Tree,
        );
        assert_eq!(
            branch.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
            vec!["r-sam", "r-jordan", "r-pat"]
        );
    }

    #[test]
    fn select_visible_branch_flat_filters_subtree() {
        let graph = sample_tree();
        let branch = select_visible_branch(
            &graph,
            "r-sam",
            &DiscussionFocus::Root,
            DiscussionViewMode::Flat,
        );
        assert_eq!(branch.len(), 3);
        assert!(branch
            .iter()
            .all(|r| { r.id == "r-sam" || r.id == "r-jordan" || r.id == "r-pat" }));
    }
}
