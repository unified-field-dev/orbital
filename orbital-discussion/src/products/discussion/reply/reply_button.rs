use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance};

use crate::{use_discussion, DiscussionReply};

/// Primary Reply affordance on a reply row footer.
#[component]
pub fn DiscussionReplyButton(reply: DiscussionReply) -> impl IntoView {
    let ctx = use_discussion();
    let reply_id = reply.id.clone();

    view! {
        <Button
            appearance=ButtonAppearance::Subtle
            class="orbital-discussion__reply-action".to_string()
            on:click=move |_| {
                ctx.events
                    .with_value(|events| events.notify_reply_click(reply_id.clone()));
            }
        >
            "Reply"
        </Button>
    }
}
