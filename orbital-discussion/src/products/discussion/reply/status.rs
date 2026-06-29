use leptos::prelude::*;
use orbital_core_components::{Spinner, SpinnerSize, Tag, TagAppearance, TagSize};

use crate::DiscussionReplyStatus;

/// Inline delivery/streaming status for a reply row header.
#[component]
pub fn DiscussionReplyStatusIndicator(
    reply_id: String,
    status: DiscussionReplyStatus,
) -> impl IntoView {
    let testid = format!("discussion-reply-status-{reply_id}");

    match status {
        DiscussionReplyStatus::Ready => view! { <span></span> }.into_any(),
        DiscussionReplyStatus::Sending => view! {
            <span
                class="orbital-discussion__reply-status orbital-discussion__reply-status--sending"
                data-testid=testid
                data-status="sending"
            >
                <Spinner size=Signal::derive(|| SpinnerSize::ExtraTiny) label=Signal::derive(|| "Sending".to_string()) />
                <Tag
                    class="orbital-discussion__reply-status-tag".to_string()
                    appearance=Signal::derive(|| TagAppearance::Outline)
                    size=Signal::derive(|| TagSize::ExtraSmall)
                >
                    "Sending…"
                </Tag>
            </span>
        }
        .into_any(),
        DiscussionReplyStatus::Streaming => view! {
            <span
                class="orbital-discussion__reply-status orbital-discussion__reply-status--streaming"
                data-testid=testid
                data-status="streaming"
            >
                <Spinner size=Signal::derive(|| SpinnerSize::ExtraTiny) label=Signal::derive(|| "Streaming".to_string()) />
                <Tag
                    class="orbital-discussion__reply-status-tag".to_string()
                    appearance=Signal::derive(|| TagAppearance::Brand)
                    size=Signal::derive(|| TagSize::ExtraSmall)
                >
                    "Streaming…"
                </Tag>
            </span>
        }
        .into_any(),
        DiscussionReplyStatus::Error(message) => {
            let error_title = message.clone();
            view! {
                <span
                    class="orbital-discussion__reply-status orbital-discussion__reply-status--error"
                    data-testid=testid
                    data-status="error"
                    title=error_title
                >
                    {message}
                </span>
            }
            .into_any()
        }
    }
}
