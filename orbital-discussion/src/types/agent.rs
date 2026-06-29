use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Lifecycle status for a tool invocation part.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionToolRunStatus {
    #[default]
    Running,
    Success,
    Error,
    AwaitingApproval,
    Denied,
}

impl DiscussionToolRunStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Success => "success",
            Self::Error => "error",
            Self::AwaitingApproval => "awaiting-approval",
            Self::Denied => "denied",
        }
    }
}

/// Tool invocation content part inside an agent reply body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionToolPart {
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    pub output: Option<Value>,
    pub status: DiscussionToolRunStatus,
    pub error_message: Option<String>,
}

impl DiscussionToolPart {
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            output: None,
            status: DiscussionToolRunStatus::Running,
            error_message: None,
        }
    }
}

/// User decision on a tool approval gate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolApprovalDecision {
    pub tool_call_id: String,
    pub reply_id: String,
    pub approved: bool,
    pub reason: Option<String>,
}

/// Streaming lifecycle for a reasoning (thinking) part.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionReasoningStatus {
    Streaming,
    #[default]
    Done,
}

/// Chain-of-thought / reasoning trace part.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionReasoningPart {
    pub text: String,
    pub status: DiscussionReasoningStatus,
}

impl DiscussionReasoningPart {
    pub fn new(text: impl Into<String>, status: DiscussionReasoningStatus) -> Self {
        Self {
            text: text.into(),
            status,
        }
    }
}

/// Status for a multi-step agent progress delimiter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionStepStatus {
    #[default]
    Active,
    Complete,
    Error,
}

impl DiscussionStepStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Complete => "complete",
            Self::Error => "error",
        }
    }
}

/// Step delimiter part for multi-step agent replies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionStepPart {
    pub step_number: u32,
    pub label: String,
    pub status: DiscussionStepStatus,
}

impl DiscussionStepPart {
    pub fn new(step_number: u32, label: impl Into<String>, status: DiscussionStepStatus) -> Self {
        Self {
            step_number,
            label: label.into(),
            status,
        }
    }
}
