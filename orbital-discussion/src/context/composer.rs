use leptos::prelude::*;

use crate::{DiscussionAttachmentDraft, DiscussionReply};

/// Composer draft state shared within a discussion thread.
#[derive(Clone, Copy)]
pub struct ComposerContext {
    pub draft: RwSignal<String>,
    pub attachment_drafts: RwSignal<Vec<DiscussionAttachmentDraft>>,
    pub citation_drafts: RwSignal<Vec<crate::DiscussionCitation>>,
}

impl ComposerContext {
    pub fn new() -> Self {
        Self {
            draft: RwSignal::new(String::new()),
            attachment_drafts: RwSignal::new(Vec::new()),
            citation_drafts: RwSignal::new(Vec::new()),
        }
    }
}

impl Default for ComposerContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Reply target resolved from composer `reply_to` state.
#[derive(Clone, Copy)]
pub struct ComposerReplyTarget {
    pub reply: Signal<Option<DiscussionReply>>,
}

/// Read the active reply-to target from the nearest composer root.
pub fn use_composer_reply_target() -> Signal<Option<DiscussionReply>> {
    expect_context::<ComposerReplyTarget>().reply
}
