//! Reply tree graph engine — adjacency index and visibility projection.
//!
//! # Performance
//!
//! Build the graph once per reply-list change with a Leptos [`Memo`](leptos::prelude::Memo)
//! in [`DiscussionThread`](crate::DiscussionThread). The memoized graph is stored on
//! [`DiscussionContext::graph`](crate::DiscussionContext::graph) for descendant lookups
//! (composer reply banner, show-more drill-in).
//!
//! Avoid calling [`DiscussionReplyGraph::from_flat`] inside derived signals or per-row
//! render paths — that rebuilds the full index on every subscription tick. Prefer
//! `ctx.graph.get()` from [`use_discussion()`](crate::use_discussion).
//!
//! Reply lists use keyed [`For`](leptos::prelude::For) iteration over stable reply ids.
//! For very large threads, prefer subscribing to individual reply records in custom
//! layouts rather than cloning the entire `replies` vector in leaf components.

use std::collections::HashMap;

use crate::{DiscussionFocus, DiscussionReply, DiscussionSort, DiscussionViewMode};

/// Default maximum visible nesting depth in tree mode.
pub const DEFAULT_MAX_VISIBLE_DEPTH: u32 = 4;

/// Adjacency graph built from a flat reply list.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiscussionReplyGraph {
    by_id: HashMap<String, DiscussionReply>,
    roots: Vec<String>,
    children: HashMap<String, Vec<String>>,
    input_order: Vec<String>,
}

impl DiscussionReplyGraph {
    /// Build a reply graph from a flat list, preserving input order for siblings.
    ///
    /// Replies whose parent is missing from the list become roots (orphan tolerance).
    /// Duplicate ids keep the last occurrence.
    pub fn from_flat(replies: &[DiscussionReply]) -> Self {
        let mut by_id = HashMap::new();
        let mut input_order = Vec::with_capacity(replies.len());

        for reply in replies {
            input_order.push(reply.id.clone());
            by_id.insert(reply.id.clone(), reply.clone());
        }

        let mut roots = Vec::new();
        let mut children: HashMap<String, Vec<String>> = HashMap::new();

        for id in &input_order {
            let Some(reply) = by_id.get(id) else {
                continue;
            };
            match &reply.parent_id {
                None => roots.push(id.clone()),
                Some(parent_id) if by_id.contains_key(parent_id) => {
                    children
                        .entry(parent_id.clone())
                        .or_default()
                        .push(id.clone());
                }
                Some(_) => roots.push(id.clone()),
            }
        }

        Self {
            by_id,
            roots,
            children,
            input_order,
        }
    }

    /// Lookup a reply by id.
    pub fn get(&self, id: &str) -> Option<&DiscussionReply> {
        self.by_id.get(id)
    }

    /// Ordered child ids for a parent reply.
    pub fn child_ids(&self, parent_id: &str) -> &[String] {
        self.children
            .get(parent_id)
            .map(|ids| ids.as_slice())
            .unwrap_or(&[])
    }

    /// Project visible replies for the current focus, mode, and sort.
    pub fn visible_replies(
        &self,
        focus: &DiscussionFocus,
        mode: DiscussionViewMode,
        sort: DiscussionSort,
    ) -> Vec<&DiscussionReply> {
        match mode {
            DiscussionViewMode::Tree => self.visible_tree(focus, sort),
            DiscussionViewMode::Flat | DiscussionViewMode::Compact => self.visible_flat(sort),
        }
    }

    /// Depth of `id` relative to the current focus anchor (anchor = 0).
    pub fn depth_in_view(&self, id: &str, focus: &DiscussionFocus) -> u32 {
        match focus {
            DiscussionFocus::Root => self.depth_from_roots(id),
            DiscussionFocus::Branch { anchor_id, .. } => {
                if id == anchor_id {
                    return 0;
                }
                if !is_descendant_of(self, id, anchor_id) {
                    return u32::MAX;
                }
                self.depth_between(anchor_id, id)
            }
        }
    }

    /// Whether `id` is `ancestor_id` or a descendant of `ancestor_id`.
    pub fn is_descendant_of(&self, id: &str, ancestor_id: &str) -> bool {
        is_descendant_of(self, id, ancestor_id)
    }

    fn visible_tree(&self, focus: &DiscussionFocus, sort: DiscussionSort) -> Vec<&DiscussionReply> {
        let root_ids = match focus {
            DiscussionFocus::Root => self.roots.clone(),
            DiscussionFocus::Branch { anchor_id, .. } => {
                if self.by_id.contains_key(anchor_id) {
                    vec![anchor_id.clone()]
                } else {
                    return Vec::new();
                }
            }
        };

        self.resolve_ids(&self.apply_sort(&root_ids, sort))
    }

    fn visible_flat(&self, sort: DiscussionSort) -> Vec<&DiscussionReply> {
        let ids: Vec<String> = match sort {
            DiscussionSort::OldestFirst => self.input_order.clone(),
            DiscussionSort::NewestFirst => self.input_order.iter().rev().cloned().collect(),
        };
        self.resolve_ids(&ids)
    }

    fn resolve_ids<'a>(&'a self, ids: &[String]) -> Vec<&'a DiscussionReply> {
        ids.iter().filter_map(|id| self.by_id.get(id)).collect()
    }

    fn apply_sort(&self, ids: &[String], sort: DiscussionSort) -> Vec<String> {
        match sort {
            DiscussionSort::OldestFirst => ids.to_vec(),
            DiscussionSort::NewestFirst => ids.iter().rev().cloned().collect(),
        }
    }

    fn depth_from_roots(&self, id: &str) -> u32 {
        let mut depth = 0;
        let mut current = id;
        while let Some(reply) = self.by_id.get(current) {
            match &reply.parent_id {
                None => return depth,
                Some(parent_id) if self.by_id.contains_key(parent_id) => {
                    depth += 1;
                    current = parent_id;
                }
                Some(_) => return depth,
            }
        }
        depth
    }

    fn depth_between(&self, ancestor_id: &str, descendant_id: &str) -> u32 {
        let mut depth = 0;
        let mut current = descendant_id;
        while current != ancestor_id {
            let Some(reply) = self.by_id.get(current) else {
                return u32::MAX;
            };
            match &reply.parent_id {
                Some(parent_id) if self.by_id.contains_key(parent_id) => {
                    depth += 1;
                    current = parent_id;
                }
                _ => return u32::MAX,
            }
        }
        depth
    }
}

fn is_descendant_of(graph: &DiscussionReplyGraph, id: &str, ancestor_id: &str) -> bool {
    if id == ancestor_id {
        return true;
    }
    let mut current = id;
    while let Some(reply) = graph.get(current) {
        match &reply.parent_id {
            Some(parent_id) if parent_id == ancestor_id => return true,
            Some(parent_id) if graph.get(parent_id).is_some() => current = parent_id,
            _ => return false,
        }
    }
    false
}

/// Push focus onto a reply, extending the breadcrumb trail.
pub fn push_focus(
    current: &DiscussionFocus,
    reply_id: &str,
    graph: &DiscussionReplyGraph,
) -> DiscussionFocus {
    if graph.get(reply_id).is_none() {
        return current.clone();
    }

    match current {
        DiscussionFocus::Root => DiscussionFocus::Branch {
            anchor_id: reply_id.to_string(),
            breadcrumb: vec![reply_id.to_string()],
        },
        DiscussionFocus::Branch { breadcrumb, .. } => {
            let mut next = breadcrumb.clone();
            next.push(reply_id.to_string());
            DiscussionFocus::Branch {
                anchor_id: reply_id.to_string(),
                breadcrumb: next,
            }
        }
    }
}

/// Pop one level of focus navigation, or return to root.
pub fn pop_focus(current: &DiscussionFocus) -> DiscussionFocus {
    match current {
        DiscussionFocus::Root => DiscussionFocus::Root,
        DiscussionFocus::Branch { breadcrumb, .. } => {
            if breadcrumb.len() <= 1 {
                DiscussionFocus::Root
            } else {
                let next = breadcrumb[..breadcrumb.len() - 1].to_vec();
                let anchor_id = next.last().cloned().unwrap_or_default();
                DiscussionFocus::Branch {
                    anchor_id,
                    breadcrumb: next,
                }
            }
        }
    }
}

/// Whether a reply at the depth cap should show a "more replies" affordance.
pub fn should_show_more_children(
    graph: &DiscussionReplyGraph,
    reply_id: &str,
    depth_in_view: u32,
    max_visible_depth: u32,
) -> bool {
    // Not visible in the current focus, or a broken parent chain.
    if depth_in_view == u32::MAX {
        return false;
    }
    if depth_in_view.saturating_add(1) < max_visible_depth {
        return false;
    }

    let reply = graph.get(reply_id);
    let hidden = reply.map(|r| r.hidden_child_count).unwrap_or(0);
    let has_children = !graph.child_ids(reply_id).is_empty();
    hidden > 0 || has_children
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DiscussionAuthor, DiscussionMetadata, DiscussionPart, DiscussionReplyStatus};
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

    fn sample_tree() -> Vec<DiscussionReply> {
        vec![
            reply("r-root", None, "root"),
            reply("r-sam", Some("r-root"), "sam"),
            reply("r-jordan", Some("r-sam"), "jordan"),
            reply("r-pat", Some("r-jordan"), "pat"),
            reply("r-morgan", Some("r-root"), "morgan"),
        ]
    }

    #[test]
    fn from_flat_builds_roots_and_children() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        assert_eq!(graph.roots, vec!["r-root".to_string()]);
        assert_eq!(
            graph.child_ids("r-root"),
            &["r-sam".to_string(), "r-morgan".to_string()]
        );
        assert_eq!(graph.child_ids("r-sam"), &["r-jordan".to_string()]);
    }

    #[test]
    fn from_flat_orphan_becomes_root() {
        let graph = DiscussionReplyGraph::from_flat(&[reply("orphan", Some("missing"), "orphan")]);
        assert_eq!(graph.roots, vec!["orphan".to_string()]);
    }

    #[test]
    fn visible_replies_tree_root() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        let visible = graph.visible_replies(
            &DiscussionFocus::Root,
            DiscussionViewMode::Tree,
            DiscussionSort::OldestFirst,
        );
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, "r-root");
    }

    #[test]
    fn visible_replies_branch_focus() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        let focus = DiscussionFocus::Branch {
            anchor_id: "r-sam".to_string(),
            breadcrumb: vec!["r-sam".to_string()],
        };
        let visible = graph.visible_replies(
            &focus,
            DiscussionViewMode::Tree,
            DiscussionSort::OldestFirst,
        );
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, "r-sam");
    }

    #[test]
    fn depth_in_view_from_root() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        assert_eq!(graph.depth_in_view("r-root", &DiscussionFocus::Root), 0);
        assert_eq!(graph.depth_in_view("r-sam", &DiscussionFocus::Root), 1);
        assert_eq!(graph.depth_in_view("r-pat", &DiscussionFocus::Root), 3);
    }

    #[test]
    fn push_focus_pop_focus() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        let focus = push_focus(&DiscussionFocus::Root, "r-sam", &graph);
        assert!(matches!(
            focus,
            DiscussionFocus::Branch { ref anchor_id, .. } if anchor_id == "r-sam"
        ));
        let focus = push_focus(&focus, "r-jordan", &graph);
        let focus = pop_focus(&focus);
        assert!(matches!(
            focus,
            DiscussionFocus::Branch { ref anchor_id, .. } if anchor_id == "r-sam"
        ));
        assert_eq!(pop_focus(&focus), DiscussionFocus::Root);
    }

    #[test]
    fn push_focus_drill_in_updates_visible_root() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        let focus = push_focus(&DiscussionFocus::Root, "r-sam", &graph);
        let focus = push_focus(&focus, "r-jordan", &graph);
        let visible = graph.visible_replies(
            &focus,
            DiscussionViewMode::Tree,
            DiscussionSort::OldestFirst,
        );
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, "r-jordan");
    }

    #[test]
    fn should_show_more_at_depth_cap() {
        let mut replies = sample_tree();
        replies[3].hidden_child_count = 3;
        let graph = DiscussionReplyGraph::from_flat(&replies);
        let depth = graph.depth_in_view("r-pat", &DiscussionFocus::Root);
        assert_eq!(depth, 3);
        assert!(should_show_more_children(
            &graph,
            "r-pat",
            depth,
            DEFAULT_MAX_VISIBLE_DEPTH
        ));
    }

    #[test]
    fn should_show_more_returns_false_for_invalid_depth() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        assert!(!should_show_more_children(
            &graph,
            "r-morgan",
            u32::MAX,
            DEFAULT_MAX_VISIBLE_DEPTH
        ));
    }

    #[test]
    fn visible_flat_returns_all_in_order() {
        let graph = DiscussionReplyGraph::from_flat(&sample_tree());
        let visible = graph.visible_replies(
            &DiscussionFocus::Root,
            DiscussionViewMode::Flat,
            DiscussionSort::OldestFirst,
        );
        assert_eq!(visible.len(), 5);
        assert_eq!(visible[0].id, "r-root");
        assert_eq!(visible[4].id, "r-morgan");
    }
}
