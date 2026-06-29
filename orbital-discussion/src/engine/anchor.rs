//! Reply anchor resolution for deep-link navigation into nested threads.

use crate::{DiscussionFocus, DiscussionReplyGraph};

use super::graph::{push_focus, should_show_more_children};

/// Root-to-target ancestor chain (inclusive).
pub fn ancestor_chain(graph: &DiscussionReplyGraph, reply_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut current = Some(reply_id);
    while let Some(id) = current {
        if graph.get(id).is_none() {
            return Vec::new();
        }
        chain.push(id.to_string());
        current = graph
            .get(id)
            .and_then(|reply| reply.parent_id.as_deref())
            .filter(|parent_id| graph.get(parent_id).is_some());
    }
    chain.reverse();
    chain
}

/// Parse a reply id from a URL fragment such as `#reply-d-l4` or `#d-l4`.
pub fn parse_reply_anchor_hash(hash: &str) -> Option<String> {
    let fragment = hash.strip_prefix('#').unwrap_or(hash).trim();
    if fragment.is_empty() {
        return None;
    }
    Some(
        fragment
            .strip_prefix("reply-")
            .unwrap_or(fragment)
            .to_string(),
    )
}

/// Whether `reply_id` is rendered under the current tree focus without drilling further.
pub fn is_reply_visible_in_tree(
    graph: &DiscussionReplyGraph,
    reply_id: &str,
    focus: &DiscussionFocus,
    max_visible_depth: u32,
) -> bool {
    if graph.depth_in_view(reply_id, focus) == u32::MAX {
        return false;
    }

    let chain = ancestor_chain(graph, reply_id);
    if chain.is_empty() {
        return false;
    }

    let start_idx = match focus {
        DiscussionFocus::Root => 0,
        DiscussionFocus::Branch { anchor_id, .. } => chain
            .iter()
            .position(|id| id == anchor_id)
            .unwrap_or(usize::MAX),
    };
    if start_idx == usize::MAX {
        return false;
    }

    for node in chain
        .iter()
        .take(chain.len().saturating_sub(1))
        .skip(start_idx)
    {
        let depth = graph.depth_in_view(node, focus);
        if should_show_more_children(graph, node, depth, max_visible_depth) {
            return false;
        }
    }

    true
}

/// Focus state that makes `reply_id` visible in tree mode, drilling through show-more barriers.
pub fn resolve_focus_for_reply(
    graph: &DiscussionReplyGraph,
    reply_id: &str,
    max_visible_depth: u32,
) -> Option<DiscussionFocus> {
    graph.get(reply_id)?;

    let mut focus = DiscussionFocus::Root;
    for _ in 0..32 {
        if is_reply_visible_in_tree(graph, reply_id, &focus, max_visible_depth) {
            return Some(focus);
        }
        let drill_id = find_drill_anchor_for_reply(graph, reply_id, &focus, max_visible_depth)?;
        let next = push_focus(&focus, &drill_id, graph);
        if next == focus {
            return None;
        }
        focus = next;
    }
    None
}

fn find_drill_anchor_for_reply(
    graph: &DiscussionReplyGraph,
    reply_id: &str,
    focus: &DiscussionFocus,
    max_visible_depth: u32,
) -> Option<String> {
    let chain = ancestor_chain(graph, reply_id);
    let start_idx = match focus {
        DiscussionFocus::Root => 0,
        DiscussionFocus::Branch { anchor_id, .. } => chain.iter().position(|id| id == anchor_id)?,
    };

    for node in chain
        .iter()
        .take(chain.len().saturating_sub(1))
        .skip(start_idx)
    {
        let depth = graph.depth_in_view(node, focus);
        if should_show_more_children(graph, node, depth, max_visible_depth) {
            return Some(node.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DiscussionAuthor, DiscussionMetadata, DiscussionPart, DiscussionReply,
        DiscussionReplyStatus, DEFAULT_MAX_VISIBLE_DEPTH,
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

    fn deep_thread() -> DiscussionReplyGraph {
        DiscussionReplyGraph::from_flat(&[
            reply("d-root", None, "root"),
            reply("d-l1", Some("d-root"), "l1"),
            reply("d-l2", Some("d-l1"), "l2"),
            reply("d-l3", Some("d-l2"), "l3"),
            reply("d-l4", Some("d-l3"), "l4"),
        ])
    }

    #[test]
    fn ancestor_chain_root_to_target() {
        let graph = deep_thread();
        assert_eq!(
            ancestor_chain(&graph, "d-l4"),
            vec![
                "d-root".to_string(),
                "d-l1".to_string(),
                "d-l2".to_string(),
                "d-l3".to_string(),
                "d-l4".to_string(),
            ]
        );
    }

    #[test]
    fn parse_reply_anchor_hash_accepts_prefix_and_raw_id() {
        assert_eq!(
            parse_reply_anchor_hash("#reply-d-l4").as_deref(),
            Some("d-l4")
        );
        assert_eq!(parse_reply_anchor_hash("#d-l4").as_deref(), Some("d-l4"));
        assert!(parse_reply_anchor_hash("#").is_none());
    }

    #[test]
    fn deep_reply_hidden_from_root() {
        let graph = deep_thread();
        assert!(!is_reply_visible_in_tree(
            &graph,
            "d-l4",
            &DiscussionFocus::Root,
            DEFAULT_MAX_VISIBLE_DEPTH,
        ));
    }

    #[test]
    fn resolve_focus_drills_to_parent_at_depth_cap() {
        let graph = deep_thread();
        let focus =
            resolve_focus_for_reply(&graph, "d-l4", DEFAULT_MAX_VISIBLE_DEPTH).expect("focus plan");
        assert!(is_reply_visible_in_tree(
            &graph,
            "d-l4",
            &focus,
            DEFAULT_MAX_VISIBLE_DEPTH,
        ));
        assert!(matches!(
            focus,
            DiscussionFocus::Branch { ref anchor_id, .. } if anchor_id == "d-l3"
        ));
    }

    #[test]
    fn shallow_reply_stays_at_root() {
        let graph = deep_thread();
        let focus =
            resolve_focus_for_reply(&graph, "d-l2", DEFAULT_MAX_VISIBLE_DEPTH).expect("focus plan");
        assert_eq!(focus, DiscussionFocus::Root);
    }
}
