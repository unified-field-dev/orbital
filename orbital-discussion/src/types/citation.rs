use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Structured citation attached to a reply.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscussionCitation {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub excerpt: Option<String>,
    /// Host-defined citation type, e.g. "agreement_vote", "doi", "cross_post".
    pub kind: Option<String>,
    /// Opaque JSON passed to citation render callbacks (votes, scores, labels).
    pub data: Option<Value>,
}

impl DiscussionCitation {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            url: None,
            excerpt: None,
            kind: None,
            data: None,
        }
    }
}
