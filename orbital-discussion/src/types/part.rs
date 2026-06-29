use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    DiscussionReasoningPart, DiscussionReasoningStatus, DiscussionStepPart, DiscussionStepStatus,
    DiscussionToolPart,
};

/// Content part composing a reply body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiscussionPart {
    Text {
        markdown: String,
        streaming: bool,
    },
    File {
        name: String,
        url: String,
        mime: Option<String>,
        size_bytes: Option<u64>,
    },
    Custom {
        kind: String,
        data: Value,
    },
    Tool(DiscussionToolPart),
    Reasoning(DiscussionReasoningPart),
    Step(DiscussionStepPart),
}

impl DiscussionPart {
    /// Plain markdown text part.
    pub fn text(markdown: impl Into<String>) -> Self {
        Self::Text {
            markdown: markdown.into(),
            streaming: false,
        }
    }

    /// Markdown text part with streaming cursor when AGENT_PARTS is enabled.
    pub fn streaming_text(markdown: impl Into<String>) -> Self {
        Self::Text {
            markdown: markdown.into(),
            streaming: true,
        }
    }

    /// Tool invocation part.
    pub fn tool(part: DiscussionToolPart) -> Self {
        Self::Tool(part)
    }

    /// Reasoning / thinking trace part.
    pub fn reasoning(text: impl Into<String>, status: DiscussionReasoningStatus) -> Self {
        Self::Reasoning(DiscussionReasoningPart::new(text, status))
    }

    /// Step delimiter part.
    pub fn step(step_number: u32, label: impl Into<String>, status: DiscussionStepStatus) -> Self {
        Self::Step(DiscussionStepPart::new(step_number, label, status))
    }
}
