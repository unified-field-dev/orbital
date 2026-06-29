use leptos::prelude::*;

use super::{
    DiscussionCitation, DiscussionComposerSubmit, DiscussionFocus, DiscussionReply,
    DiscussionToolPart, ToolApprovalDecision,
};

/// Event callbacks fired when the user interacts with the discussion thread.
#[derive(Clone, Default)]
pub struct DiscussionEvents {
    /// Fires when focus navigation changes (drill-in or go back).
    pub on_focus_change: Option<Callback<DiscussionFocus, ()>>,
    /// Fires when the primary Reply affordance is clicked.
    pub on_reply_click: Option<Callback<String, ()>>,
    /// Fires when a reply overflow menu action is chosen.
    pub on_reply_action: Option<Callback<(DiscussionReply, String), ()>>,
    /// Fires when a branch collapse toggle changes (`reply_id`, collapsed).
    pub on_collapse_change: Option<Callback<(String, bool), ()>>,
    /// Fires when a citation menu action is chosen (wired in Phase 4).
    pub on_citation_action: Option<Callback<(DiscussionCitation, String), ()>>,
    /// Fires when the composer submits a reply draft.
    pub on_submit: Option<Callback<DiscussionComposerSubmit, ()>>,
    /// Fires when a tool part is rendered or its status changes (Phase 6).
    pub on_tool_call: Option<Callback<DiscussionToolPart, ()>>,
    /// Fires when the user approves or denies a tool invocation (Phase 6).
    pub on_tool_approval: Option<Callback<ToolApprovalDecision, ()>>,
    /// Fires when attachment validation rejects one or more files (name, reason).
    pub on_attachment_reject: Option<Callback<Vec<(String, String)>, ()>>,
}

impl DiscussionEvents {
    pub fn notify_focus_change(&self, focus: DiscussionFocus) {
        if let Some(cb) = &self.on_focus_change {
            cb.run(focus);
        }
    }

    pub fn notify_collapse_change(&self, reply_id: String, collapsed: bool) {
        if let Some(cb) = &self.on_collapse_change {
            cb.run((reply_id, collapsed));
        }
    }

    pub fn notify_reply_click(&self, reply_id: String) {
        if let Some(cb) = &self.on_reply_click {
            cb.run(reply_id);
        }
    }

    pub fn notify_reply_action(&self, reply: DiscussionReply, action: String) {
        if let Some(cb) = &self.on_reply_action {
            cb.run((reply, action));
        }
    }

    pub fn notify_submit(&self, payload: DiscussionComposerSubmit) {
        if let Some(cb) = &self.on_submit {
            cb.run(payload);
        }
    }

    pub fn notify_citation_action(&self, citation: DiscussionCitation, action: String) {
        if let Some(cb) = &self.on_citation_action {
            cb.run((citation, action));
        }
    }

    pub fn notify_tool_call(&self, part: DiscussionToolPart) {
        if let Some(cb) = &self.on_tool_call {
            cb.run(part);
        }
    }

    pub fn notify_tool_approval(&self, decision: ToolApprovalDecision) {
        if let Some(cb) = &self.on_tool_approval {
            cb.run(decision);
        }
    }

    pub fn notify_attachment_reject(&self, rejections: Vec<(String, String)>) {
        if let Some(cb) = &self.on_attachment_reject {
            cb.run(rejections);
        }
    }
}
