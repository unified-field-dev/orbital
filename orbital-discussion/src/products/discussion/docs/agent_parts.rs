use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::preview::fixtures::agent_thread_mock_h;
use crate::{
    DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionToolPart,
    DiscussionViewMode, ToolApprovalDecision,
};

/// Agent tool, reasoning, and step part renderers.
///
/// # When to use
///
/// - Agent-authored replies with tool calls, thinking blocks, and step tracking.
///
/// # Usage
///
/// Enable `DiscussionFeatures::AGENT_PARTS` and wire `on_tool_call` / `on_tool_approval`
/// on `DiscussionEvents`. The host pushes tool, reasoning, and streaming text state via
/// the controlled `replies` signal — no stream transport is built into the crate.
///
/// # Examples
///
/// ## Mock H agent reply
/// Agent card with reasoning, tool approval, and streaming text cursor.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::agent_thread_mock_h;
/// use crate::{
///     DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionToolPart,
///     DiscussionViewMode, ToolApprovalDecision,
/// };
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(agent_thread_mock_h());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let (log, set_log) = signal(String::new());
/// let events = DiscussionEvents {
///     on_tool_call: Some(Callback::new(move |part: DiscussionToolPart| {
///         set_log.set(format!("tool_call:{}", part.tool_name));
///     })),
///     on_tool_approval: Some(Callback::new(move |decision: ToolApprovalDecision| {
///         set_log.set(format!(
///             "approval:{}:approved={}",
///             decision.tool_call_id, decision.approved
///         ));
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN
///     | DiscussionFeatures::FOCUS_NAVIGATION
///     | DiscussionFeatures::AGENT_PARTS;
/// view! {
///     <div data-testid="discussion-agent-parts-preview">
///         <pre data-testid="discussion-agent-parts-log">{move || log.get()}</pre>
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             events=events
///             features=features
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-agent-parts",
    preview_label = "Agent Parts",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionAgentPartsDoc() -> impl IntoView {
    let (replies, _set_replies) = signal(agent_thread_mock_h());
    let (focus, set_focus) = signal(DiscussionFocus::Root);
    let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
    let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());
    let (log, set_log) = signal(String::new());

    let events = DiscussionEvents {
        on_tool_call: Some(Callback::new(move |part: DiscussionToolPart| {
            set_log.set(format!(
                "tool_call:{}:{}",
                part.tool_name,
                part.status.as_str()
            ));
        })),
        on_tool_approval: Some(Callback::new(move |decision: ToolApprovalDecision| {
            set_log.set(format!(
                "approval:{}:approved={}",
                decision.tool_call_id, decision.approved
            ));
        })),
        ..Default::default()
    };

    let features = DiscussionFeatures::MARKDOWN
        | DiscussionFeatures::FOCUS_NAVIGATION
        | DiscussionFeatures::AGENT_PARTS;

    view! {
        <div data-testid="discussion-agent-parts-preview">
            <pre data-testid="discussion-agent-parts-log">{move || log.get()}</pre>
            <DiscussionThread
                replies=Signal::derive(move || replies.get())
                focus=Signal::derive(move || focus.get())
                set_focus=set_focus
                view_mode=Signal::derive(move || view_mode.get())
                set_view_mode=set_view_mode
                collapsed=Signal::derive(move || collapsed.get())
                set_collapsed=set_collapsed
                events=events
                features=features
            />
        </div>
    }
}
