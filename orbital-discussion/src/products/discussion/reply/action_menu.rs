use leptos::prelude::*;

use crate::{use_discussion, DiscussionReply, ReplyRenderContext};

use super::super::shared::DiscussionOverflowMenu;

/// Optional overflow menu beside Reply for host-defined secondary actions.
#[component]
pub fn DiscussionReplyActionMenu(reply: DiscussionReply) -> impl IntoView {
    let ctx = use_discussion();
    let render_ctx = ReplyRenderContext::new(reply.clone());
    let reply_for_select = reply.clone();
    let reply_id = reply.id.clone();
    let menu_testid = format!("discussion-reply-menu-{reply_id}");

    let custom_menu = ctx.renderers.with_value(|renderers| {
        renderers
            .reply_menu
            .as_ref()
            .map(|view| view(render_ctx.clone()))
    });

    if let Some(custom_view) = custom_menu {
        return view! {
            <div class="orbital-discussion__reply-action-menu" data-testid=menu_testid>
                {custom_view}
            </div>
        }
        .into_any();
    }

    let menu_items = ctx.renderers.with_value(|renderers| {
        renderers
            .reply_menu_extras
            .as_ref()
            .map(|view| view(render_ctx))
            .unwrap_or_default()
    });

    if menu_items.is_empty() {
        return view! { <span></span> }.into_any();
    }

    let on_select = Callback::new(move |action: String| {
        ctx.events.with_value(|events| {
            events.notify_reply_action(reply_for_select.clone(), action);
        });
    });

    view! {
        <div class="orbital-discussion__reply-action-menu" data-testid=menu_testid>
            <DiscussionOverflowMenu
                items=menu_items
                on_select=on_select
                action_testid_prefix="discussion-reply-action".to_string()
                aria_label="Reply actions".to_string()
            />
        </div>
    }
    .into_any()
}
