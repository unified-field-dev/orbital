use chrono::{Duration, TimeZone, Utc};

use crate::{
    DiscussionAuthor, DiscussionAuthorRole, DiscussionCitation, DiscussionComposerSubmit,
    DiscussionLabel, DiscussionMetadata, DiscussionPart, DiscussionReply, DiscussionReplyStatus,
    DiscussionStepPart,
};

fn reply_at(
    id: &str,
    parent_id: Option<&str>,
    author_name: &str,
    text: &str,
    labels: Vec<DiscussionLabel>,
    hidden_child_count: u32,
    role: DiscussionAuthorRole,
    created_at: chrono::DateTime<Utc>,
    edited: bool,
) -> DiscussionReply {
    DiscussionReply {
        id: id.to_string(),
        parent_id: parent_id.map(str::to_string),
        author: DiscussionAuthor::new(id, author_name).with_role(role),
        metadata: DiscussionMetadata {
            created_at,
            edited_at: edited.then(|| created_at + Duration::minutes(15)),
            labels,
        },
        parts: vec![DiscussionPart::text(text)],
        citations: vec![],
        status: DiscussionReplyStatus::Ready,
        hidden_child_count,
    }
}

fn reply(
    id: &str,
    parent_id: Option<&str>,
    author_name: &str,
    text: &str,
    labels: Vec<DiscussionLabel>,
    hidden_child_count: u32,
    role: DiscussionAuthorRole,
    hours_ago: i64,
    edited: bool,
) -> DiscussionReply {
    let created_at = Utc::now() - Duration::hours(hours_ago);
    reply_at(
        id,
        parent_id,
        author_name,
        text,
        labels,
        hidden_child_count,
        role,
        created_at,
        edited,
    )
}

/// Logged-in viewer author id used in previews for accent card tinting.
pub const PREVIEW_VIEWER_AUTHOR_ID: &str = "r-jordan";

/// Build an in-memory reply from a composer submit (preview / demo only).
pub fn reply_from_submit(
    payload: &DiscussionComposerSubmit,
    author_id: &str,
    author_name: &str,
) -> DiscussionReply {
    let id = format!("preview-{}", Utc::now().timestamp_millis());
    let mut parts = vec![DiscussionPart::text(&payload.body_markdown)];

    for attachment in &payload.attachments {
        let url = attachment
            .uploaded_url
            .clone()
            .unwrap_or_else(|| format!("https://orbital.dev/files/{}", attachment.name));
        parts.push(DiscussionPart::File {
            name: attachment.name.clone(),
            url,
            mime: attachment.mime.clone(),
            size_bytes: attachment.size_bytes,
        });
    }

    DiscussionReply {
        id,
        parent_id: payload.parent_id.clone(),
        author: DiscussionAuthor::new(author_id, author_name),
        metadata: DiscussionMetadata {
            created_at: Utc::now(),
            edited_at: None,
            labels: vec![],
        },
        parts,
        citations: payload.citations.clone(),
        status: DiscussionReplyStatus::Ready,
        hidden_child_count: 0,
    }
}

/// Append an optimistic in-memory reply from a composer submit (preview / demo only).
pub fn append_composer_reply(
    replies: &mut Vec<DiscussionReply>,
    payload: &DiscussionComposerSubmit,
    author_id: &str,
    author_name: &str,
) {
    replies.push(reply_from_submit(payload, author_id, author_name));
}

/// Sample nested thread for previews and tests.
pub fn sample_thread() -> Vec<DiscussionReply> {
    vec![
        reply(
            "r-root",
            None,
            "Alex Chen",
            "Main message with **markdown** and a [design link](https://orbital.dev).",
            vec![DiscussionLabel::Op],
            0,
            DiscussionAuthorRole::User,
            2,
            false,
        ),
        reply(
            "r-sam",
            Some("r-root"),
            "Sam Rivera",
            "Agree — let's track open questions here.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            1,
            true,
        ),
        reply(
            "r-jordan",
            Some("r-sam"),
            "Jordan Lee",
            "I'll draft the migration checklist.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
        reply(
            "r-pat",
            Some("r-jordan"),
            "Pat Okonkwo",
            "Depth-3 reply for tree testing.",
            vec![],
            3,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
        reply(
            "r-morgan",
            Some("r-root"),
            "Morgan Kim",
            "Separate branch at L1.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
        reply(
            "r-agent",
            Some("r-morgan"),
            "Orbital Agent",
            "Based on the thread, the default depth is **4**.",
            vec![],
            0,
            DiscussionAuthorRole::Agent,
            0,
            false,
        ),
    ]
}

/// Empty thread for overlay previews (Phase 4).
pub fn empty_thread() -> Vec<DiscussionReply> {
    vec![]
}

/// Short thread for slots composition preview (OP + one reply).
pub fn slots_thread() -> Vec<DiscussionReply> {
    vec![
        reply(
            "slots-root",
            None,
            "Alex Chen",
            "Welcome to the discussion — customize the toolbar slot above.",
            vec![DiscussionLabel::Op],
            0,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
        reply(
            "slots-reply",
            Some("slots-root"),
            "Sam Rivera",
            "Looks good. The host owns this toolbar region.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
    ]
}

/// Deep nested thread (5 levels) for tree navigation and show-more previews.
pub fn deep_thread() -> Vec<DiscussionReply> {
    vec![
        reply(
            "d-root",
            None,
            "Alex Chen",
            "Root post for depth testing.",
            vec![DiscussionLabel::Op],
            0,
            DiscussionAuthorRole::User,
            5,
            false,
        ),
        reply(
            "d-l1",
            Some("d-root"),
            "Sam Rivera",
            "Level 1 reply.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            4,
            false,
        ),
        reply(
            "d-l2",
            Some("d-l1"),
            "Jordan Lee",
            "Level 2 reply.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            3,
            false,
        ),
        reply(
            "d-l3",
            Some("d-l2"),
            "Pat Okonkwo",
            "Level 3 reply.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            2,
            false,
        ),
        reply(
            "d-l4",
            Some("d-l3"),
            "Casey Ng",
            "Level 4 reply at depth cap.",
            vec![],
            4,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
    ]
}

/// Flat thread with replies spanning two calendar days for date divider previews.
pub fn flat_thread_with_dates() -> Vec<DiscussionReply> {
    let yesterday = Utc::now().date_naive() - Duration::days(1);
    let today = Utc::now().date_naive();
    let yesterday_noon =
        Utc.from_utc_datetime(&yesterday.and_hms_opt(12, 0, 0).expect("valid time"));
    let today_noon = Utc.from_utc_datetime(&today.and_hms_opt(12, 0, 0).expect("valid time"));

    vec![
        reply_at(
            "fd-root",
            None,
            "Alex Chen",
            "Yesterday's root post.",
            vec![DiscussionLabel::Op],
            0,
            DiscussionAuthorRole::User,
            yesterday_noon,
            false,
        ),
        reply_at(
            "fd-y1",
            Some("fd-root"),
            "Sam Rivera",
            "Reply from yesterday.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            yesterday_noon + Duration::hours(1),
            false,
        ),
        reply_at(
            "fd-t1",
            Some("fd-root"),
            "Jordan Lee",
            "Reply from today.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            today_noon,
            false,
        ),
        reply_at(
            "fd-t2",
            Some("fd-t1"),
            "Pat Okonkwo",
            "Nested reply from today.",
            vec![],
            0,
            DiscussionAuthorRole::User,
            today_noon + Duration::hours(1),
            false,
        ),
    ]
}

/// Thread with a fenced code block in the root reply.
pub fn thread_with_code_block() -> Vec<DiscussionReply> {
    vec![reply(
        "cb-root",
        None,
        "Alex Chen",
        "Example usage:\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```",
        vec![DiscussionLabel::Op],
        0,
        DiscussionAuthorRole::User,
        1,
        false,
    )]
}

/// Thread with structured citations on the root reply (includes inline ref in body).
pub fn thread_with_citations() -> Vec<DiscussionReply> {
    thread_with_custom_citations()
}

/// Citations with custom kind/data and markdown `[^id]` reference in body.
pub fn thread_with_custom_citations() -> Vec<DiscussionReply> {
    vec![DiscussionReply {
        id: "cit-root".to_string(),
        parent_id: None,
        author: DiscussionAuthor::new("cit-root", "Alex Chen"),
        metadata: DiscussionMetadata {
            created_at: Utc::now() - Duration::hours(2),
            edited_at: None,
            labels: vec![DiscussionLabel::Op],
        },
        parts: vec![DiscussionPart::text(
            "Reply referencing external sources — see [^cit-1] and [^cit-2].",
        )],
        citations: vec![
            DiscussionCitation {
                id: "cit-1".to_string(),
                title: "Design doc §4.2".to_string(),
                url: Some("https://orbital.dev/discussion/design".to_string()),
                excerpt: Some(
                    "Depth cap defaults to four visible levels before show-more drill-in."
                        .to_string(),
                ),
                kind: Some("agreement_vote".to_string()),
                data: Some(serde_json::json!({
                    "score": 3,
                    "viewer_vote": "agree"
                })),
            },
            DiscussionCitation {
                id: "cit-2".to_string(),
                title: "Research worksheet DISC-19".to_string(),
                url: Some("https://orbital.dev/discussion/research".to_string()),
                excerpt: None,
                kind: Some("agreement_vote".to_string()),
                data: Some(serde_json::json!({ "score": 1, "viewer_vote": null })),
            },
        ],
        status: DiscussionReplyStatus::Ready,
        hidden_child_count: 0,
    }]
}

/// Thread with image and download file parts.
pub fn thread_with_file_parts() -> Vec<DiscussionReply> {
    vec![DiscussionReply {
        id: "fp-root".to_string(),
        parent_id: None,
        author: DiscussionAuthor::new("fp-root", "Alex Chen"),
        metadata: DiscussionMetadata {
            created_at: Utc::now() - Duration::hours(1),
            edited_at: None,
            labels: vec![DiscussionLabel::Op],
        },
        parts: vec![
            DiscussionPart::text("Screenshot and spec attached below."),
            DiscussionPart::File {
                name: "architecture.png".to_string(),
                url: "https://picsum.photos/seed/orbital-discussion/640/360".to_string(),
                mime: Some("image/png".to_string()),
                size_bytes: Some(245_760),
            },
            DiscussionPart::File {
                name: "specification.pdf".to_string(),
                url: "https://orbital.dev/files/specification.pdf".to_string(),
                mime: Some("application/pdf".to_string()),
                size_bytes: Some(1_048_576),
            },
        ],
        citations: vec![],
        status: DiscussionReplyStatus::Ready,
        hidden_child_count: 0,
    }]
}

/// Thread demonstrating non-ready reply status indicators.
pub fn thread_with_statuses() -> Vec<DiscussionReply> {
    vec![
        reply(
            "st-sending",
            None,
            "Alex Chen",
            "Optimistic reply while the host persists.",
            vec![DiscussionLabel::Op],
            0,
            DiscussionAuthorRole::User,
            0,
            false,
        ),
        DiscussionReply {
            id: "st-streaming".to_string(),
            parent_id: Some("st-sending".to_string()),
            author: DiscussionAuthor::new("st-streaming", "Orbital Agent")
                .with_role(DiscussionAuthorRole::Agent),
            metadata: DiscussionMetadata {
                created_at: Utc::now(),
                edited_at: None,
                labels: vec![],
            },
            parts: vec![DiscussionPart::text("Streaming response in progress…")],
            citations: vec![],
            status: DiscussionReplyStatus::Streaming,
            hidden_child_count: 0,
        },
        DiscussionReply {
            id: "st-error".to_string(),
            parent_id: Some("st-sending".to_string()),
            author: DiscussionAuthor::new("st-error", "Sam Rivera"),
            metadata: DiscussionMetadata {
                created_at: Utc::now(),
                edited_at: None,
                labels: vec![],
            },
            parts: vec![DiscussionPart::text("Failed to deliver.")],
            citations: vec![],
            status: DiscussionReplyStatus::Error("Network timeout".to_string()),
            hidden_child_count: 0,
        },
    ]
    .into_iter()
    .map(|mut reply| {
        if reply.id == "st-sending" {
            reply.status = DiscussionReplyStatus::Sending;
        }
        reply
    })
    .collect()
}

/// Thread with a custom part for part_view previews.
pub fn thread_with_custom_part() -> Vec<DiscussionReply> {
    vec![DiscussionReply {
        id: "cp-root".to_string(),
        parent_id: None,
        author: DiscussionAuthor::new("cp-root", "Alex Chen"),
        metadata: DiscussionMetadata {
            created_at: Utc::now() - Duration::hours(1),
            edited_at: None,
            labels: vec![DiscussionLabel::Op],
        },
        parts: vec![DiscussionPart::Custom {
            kind: "poll".to_string(),
            data: serde_json::json!({ "question": "Ship Phase 2?" }),
        }],
        citations: vec![],
        status: DiscussionReplyStatus::Ready,
        hidden_child_count: 0,
    }]
}

/// Mock H — agent reply with reasoning, tool approval, and streaming text (Phase 6).
pub fn agent_thread_mock_h() -> Vec<DiscussionReply> {
    use crate::{
        DiscussionReasoningStatus, DiscussionStepStatus, DiscussionToolPart,
        DiscussionToolRunStatus,
    };

    vec![
        reply(
            "r-agent-root",
            None,
            "Alex Chen",
            "What is the default tree depth limit?",
            vec![DiscussionLabel::Op],
            0,
            DiscussionAuthorRole::User,
            1,
            false,
        ),
        DiscussionReply {
            id: "r-agent-h".to_string(),
            parent_id: Some("r-agent-root".to_string()),
            author: DiscussionAuthor::new("r-agent-h", "Orbital Agent")
                .with_role(DiscussionAuthorRole::Agent),
            metadata: DiscussionMetadata {
                created_at: Utc::now(),
                edited_at: None,
                labels: vec![],
            },
            parts: vec![
                DiscussionPart::reasoning(
                    "Checking design doc for depth configuration…",
                    DiscussionReasoningStatus::Done,
                ),
                DiscussionPart::Step(DiscussionStepPart::new(
                    1,
                    "Search documentation",
                    DiscussionStepStatus::Active,
                )),
                DiscussionPart::tool(DiscussionToolPart {
                    tool_call_id: "tc-search-docs".to_string(),
                    tool_name: "search_docs".to_string(),
                    input: serde_json::json!({ "query": "depth limit" }),
                    output: None,
                    status: DiscussionToolRunStatus::AwaitingApproval,
                    error_message: None,
                }),
                DiscussionPart::streaming_text(
                    "Based on the design doc, the default depth is **4**.",
                ),
            ],
            citations: vec![],
            status: DiscussionReplyStatus::Streaming,
            hidden_child_count: 0,
        },
    ]
}
