use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Label chip shown in reply metadata (OP, moderator, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionLabel {
    Op,
    Moderator,
    Custom(String),
}

/// Timestamps and labels for a reply.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionMetadata {
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub labels: Vec<DiscussionLabel>,
}
