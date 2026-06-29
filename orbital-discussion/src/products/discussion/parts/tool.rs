use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, ButtonType, Card, CardContent, MaterialElevation, MaterialVariant,
    Tag, TagAppearance, TagSize,
};

use crate::{use_discussion, DiscussionToolPart, DiscussionToolRunStatus, ToolApprovalDecision};

/// Approve / deny actions for a tool awaiting user confirmation.
#[component]
pub fn DiscussionToolApprovalBar(reply_id: String, tool_call_id: String) -> impl IntoView {
    let ctx = use_discussion();

    let on_approve = {
        let reply_id = reply_id.clone();
        let tool_call_id = tool_call_id.clone();
        move |_| {
            ctx.events.with_value(|events| {
                events.notify_tool_approval(ToolApprovalDecision {
                    tool_call_id: tool_call_id.clone(),
                    reply_id: reply_id.clone(),
                    approved: true,
                    reason: None,
                });
            });
        }
    };

    let on_deny = {
        let reply_id = reply_id.clone();
        let tool_call_id = tool_call_id.clone();
        move |_| {
            ctx.events.with_value(|events| {
                events.notify_tool_approval(ToolApprovalDecision {
                    tool_call_id: tool_call_id.clone(),
                    reply_id: reply_id.clone(),
                    approved: false,
                    reason: None,
                });
            });
        }
    };

    view! {
        <div class="orbital-discussion__tool-approval" data-testid="discussion-tool-approval">
            <span data-testid="discussion-tool-approve">
                <Button
                    appearance=ButtonAppearance::Primary
                    button_type=ButtonType::Button
                    on:click=on_approve
                >
                    "Approve"
                </Button>
            </span>
            <span data-testid="discussion-tool-deny">
                <Button
                    appearance=ButtonAppearance::Secondary
                    button_type=ButtonType::Button
                    on:click=on_deny
                >
                    "Deny"
                </Button>
            </span>
        </div>
    }
}

fn tool_status_label(status: DiscussionToolRunStatus) -> &'static str {
    match status {
        DiscussionToolRunStatus::Running => "Running",
        DiscussionToolRunStatus::Success => "Success",
        DiscussionToolRunStatus::Error => "Error",
        DiscussionToolRunStatus::AwaitingApproval => "Awaiting approval",
        DiscussionToolRunStatus::Denied => "Denied",
    }
}

fn tool_status_appearance(status: DiscussionToolRunStatus) -> TagAppearance {
    match status {
        DiscussionToolRunStatus::Running => TagAppearance::Outline,
        DiscussionToolRunStatus::Success => TagAppearance::Filled,
        DiscussionToolRunStatus::Error => TagAppearance::Outline,
        DiscussionToolRunStatus::AwaitingApproval => TagAppearance::Brand,
        DiscussionToolRunStatus::Denied => TagAppearance::Outline,
    }
}

/// Tool invocation card with input JSON and optional approval bar.
#[component]
pub fn DiscussionToolPartView(reply_id: String, part: DiscussionToolPart) -> impl IntoView {
    let ctx = use_discussion();
    let tool_name = part.tool_name.clone();
    let tool_call_id = part.tool_call_id.clone();
    let status = part.status;
    let status_attr = status.as_str().to_string();
    let error_message = part.error_message.clone();
    let show_approval = status == DiscussionToolRunStatus::AwaitingApproval;

    let input_json = serde_json::to_string_pretty(&part.input).unwrap_or_else(|_| "{}".to_string());
    let output_json = part
        .output
        .as_ref()
        .and_then(|value| serde_json::to_string_pretty(value).ok());

    Effect::new({
        let part = part.clone();
        move |_| {
            ctx.events
                .with_value(|events| events.notify_tool_call(part.clone()));
        }
    });

    view! {
        <div
            class="orbital-discussion__tool-part-wrap"
            data-testid="discussion-tool-part"
            data-tool-status=status_attr
        >
            <Card
                variant=MaterialVariant::Outlined
                elevation=MaterialElevation::Flat
                class="orbital-discussion__tool-part".to_string()
            >
            <CardContent class="orbital-discussion__tool-part-content".to_string()>
                <div class="orbital-discussion__tool-part-header">
                    <span class="orbital-discussion__tool-part-name">{format!("Tool: {tool_name}")}</span>
                    <Tag
                        class="orbital-discussion__tool-part-status".to_string()
                        appearance=Signal::derive(move || tool_status_appearance(status))
                        size=Signal::derive(|| TagSize::ExtraSmall)
                    >
                        {tool_status_label(status)}
                    </Tag>
                </div>
                <pre class="orbital-discussion__tool-input" data-testid="discussion-tool-input">
                    {input_json}
                </pre>
                {error_message.map(|message| view! {
                    <p class="orbital-discussion__tool-error">{message}</p>
                })}
                {output_json.map(|json| view! {
                    <pre class="orbital-discussion__tool-output" data-testid="discussion-tool-output">
                        {json}
                    </pre>
                })}
                <Show when=move || show_approval>
                    <DiscussionToolApprovalBar
                        reply_id=reply_id.clone()
                        tool_call_id=tool_call_id.clone()
                    />
                </Show>
            </CardContent>
        </Card>
        </div>
    }
}
