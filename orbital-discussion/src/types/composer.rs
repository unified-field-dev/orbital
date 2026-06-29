//! Composer draft types passed to [`DiscussionAdapter::submit_reply`](super::DiscussionAdapter::submit_reply).

use serde::{Deserialize, Serialize};

use super::DiscussionCitation;

/// Client-side attachment validation rules for the composer attach button.
///
/// The host should re-validate attachments on submit and in
/// [`DiscussionAdapter::submit_reply`](super::DiscussionAdapter::submit_reply).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionAttachmentValidation {
    /// Allowed MIME types (exact or wildcard, e.g. `image/*`).
    pub accepted_mime_types: Option<Vec<String>>,
    pub max_file_count: Option<u32>,
    pub max_file_size: Option<u64>,
}

/// Result of uploading an attachment draft to host storage.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionUploadedFile {
    pub url: String,
    pub name: String,
    pub mime: Option<String>,
    pub size_bytes: Option<u64>,
}

/// Draft attachment queued in the composer before submit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionAttachmentDraft {
    pub id: String,
    pub name: String,
    pub mime: Option<String>,
    pub size_bytes: Option<u64>,
    /// Populated after host upload (pick-time or submit-time).
    pub uploaded_url: Option<String>,
}

impl DiscussionAttachmentDraft {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            mime: None,
            size_bytes: None,
            uploaded_url: None,
        }
    }
}

/// Payload emitted when the composer submits a reply.
///
/// Passed to [`DiscussionEvents::on_submit`](super::DiscussionEvents::on_submit) and
/// [`DiscussionAdapter::submit_reply`](super::DiscussionAdapter::submit_reply).
/// Attachments are collected in the composer but validated by the host (Phase 4).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionComposerSubmit {
    pub body_markdown: String,
    pub parent_id: Option<String>,
    pub attachments: Vec<DiscussionAttachmentDraft>,
    pub citations: Vec<DiscussionCitation>,
}
