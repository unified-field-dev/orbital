use serde::{Deserialize, Serialize};

/// Delivery and streaming status for a reply row.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionReplyStatus {
    #[default]
    Ready,
    Sending,
    Streaming,
    Error(String),
}
