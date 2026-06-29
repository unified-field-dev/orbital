use serde::{Deserialize, Serialize};

use super::{
    DiscussionAuthor, DiscussionCitation, DiscussionMetadata, DiscussionPart, DiscussionReplyStatus,
};

/// Single item in a discussion thread.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionReply {
    pub id: String,
    pub parent_id: Option<String>,
    pub author: DiscussionAuthor,
    pub metadata: DiscussionMetadata,
    pub parts: Vec<DiscussionPart>,
    pub citations: Vec<DiscussionCitation>,
    pub status: DiscussionReplyStatus,
    pub hidden_child_count: u32,
}

impl DiscussionReply {
    /// Concatenate text parts as plain fallback (no markdown rendering).
    pub fn plain_text_fallback(&self) -> String {
        self.parts
            .iter()
            .filter_map(|part| match part {
                DiscussionPart::Text { markdown, .. } => Some(markdown.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
