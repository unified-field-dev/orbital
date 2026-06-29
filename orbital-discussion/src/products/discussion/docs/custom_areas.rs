use leptos::prelude::*;
use orbital_core_components::{Caption1, Material, MaterialElevation, MaterialVariant};
use orbital_macros::component_doc;
use std::sync::Arc;

use crate::preview::fixtures::sample_thread;
use crate::{
    DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionMenuItem,
    DiscussionReplyMenuExtras, DiscussionReplyMeta, DiscussionThread, DiscussionViewMode,
    ReplyMenuExtrasView, ReplyMetaView, ReplyRenderContext,
};

/// Custom meta, reply overflow actions, and footer regions via Leptos slot children.
///
/// # When to use
///
/// - Reactions and votes in a custom meta region.
/// - Moderation and host actions via reply overflow menu extras (beside Reply).
/// - Polls and status widgets in a custom footer region.
///
/// # Usage
///
/// Nest [`DiscussionReplyMeta`] and [`DiscussionReplyMenuExtras`] slot children on
/// [`DiscussionThread`]. Wire `events.on_reply_action` for overflow menu selections.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `DiscussionReplyMeta` for reactions and vote counts — the crate does not ship a vote rail.
/// * Keep moderation actions in `DiscussionReplyMenuExtras` with stable action ids for analytics.
///
/// ## Don'ts
///
/// * Do not replace the built-in Reply button when you only need extra overflow actions —
///   use `DiscussionReplyMenuExtras` instead of a full `DiscussionReplyMenu` override.
///
/// # See also
///
/// * [`DiscussionEventsDoc`](crate::products::discussion::docs::events::DiscussionEventsDoc)
/// * [`DiscussionRepliesDoc`](crate::products::discussion::docs::replies::DiscussionRepliesDoc)
///
/// # Examples
///
/// ## Custom meta and reply overflow actions
/// Host-owned reaction count and a Report ToS action beside Reply.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_thread;
/// use crate::{
///     DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionMenuItem,
///     DiscussionReplyMenuExtras, DiscussionReplyMeta, DiscussionThread, DiscussionViewMode,
///     ReplyMenuExtrasView, ReplyMetaView, ReplyRenderContext,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Caption1, Material, MaterialElevation, MaterialVariant};
/// use std::collections::HashSet;
/// use std::sync::Arc;
/// let meta_view: ReplyMetaView = Arc::new(|_ctx: ReplyRenderContext| {
///     view! {
///         <Caption1 class="orbital-discussion__custom-meta" attr:data-testid="discussion-custom-meta">
///             "12 likes"
///         </Caption1>
///     }
///     .into_any()
/// });
/// let reply_menu_extras: ReplyMenuExtrasView = Arc::new(|_ctx: ReplyRenderContext| {
///     vec![
///         DiscussionMenuItem {
///             id: "report_tos".into(),
///             label: "Report ToS violation".into(),
///             disabled: None,
///         },
///         DiscussionMenuItem {
///             id: "copy_link".into(),
///             label: "Copy permalink".into(),
///             disabled: None,
///         },
///     ]
/// });
/// let (replies, _set_replies) = signal(sample_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let (log, set_log) = signal(String::new());
/// let events = DiscussionEvents {
///     on_reply_action: Some(Callback::new(move |(reply, action): (crate::DiscussionReply, String)| {
///         set_log.set(format!("{}:{}", reply.id, action));
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-custom-areas-preview">
///         <Material
///             variant=MaterialVariant::Solid
///             elevation=MaterialElevation::Flat
///             class="orbital-discussion__custom-action-log".to_string()
///         >
///             <Caption1 attr:data-testid="discussion-custom-action-log">{move || log.get()}</Caption1>
///         </Material>
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
///         >
///             <DiscussionReplyMeta slot render=meta_view />
///             <DiscussionReplyMenuExtras slot items=reply_menu_extras />
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-custom-areas",
    preview_label = "Custom Areas",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionCustomAreasDoc() -> impl IntoView {
    let meta_view: ReplyMetaView = Arc::new(|_ctx: ReplyRenderContext| {
        view! {
            <Caption1
                class="orbital-discussion__custom-meta"
                attr:data-testid="discussion-custom-meta"
            >
                "12 likes"
            </Caption1>
        }
        .into_any()
    });
    let reply_menu_extras: ReplyMenuExtrasView = Arc::new(|_ctx: ReplyRenderContext| {
        vec![
            DiscussionMenuItem {
                id: "report_tos".into(),
                label: "Report ToS violation".into(),
                disabled: None,
            },
            DiscussionMenuItem {
                id: "copy_link".into(),
                label: "Copy permalink".into(),
                disabled: None,
            },
        ]
    });

    let (replies, _set_replies) = signal(sample_thread());
    let (focus, set_focus) = signal(DiscussionFocus::Root);
    let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
    let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());
    let (log, set_log) = signal(String::new());

    let events = DiscussionEvents {
        on_reply_action: Some(Callback::new(
            move |(reply, action): (crate::DiscussionReply, String)| {
                set_log.set(format!("{}:{}", reply.id, action));
            },
        )),
        ..Default::default()
    };

    let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;

    view! {
        <div data-testid="discussion-custom-areas-preview">
            <Material
                variant=MaterialVariant::Solid
                elevation=MaterialElevation::Flat
                class="orbital-discussion__custom-action-log".to_string()
            >
                <Caption1 attr:data-testid="discussion-custom-action-log">{move || log.get()}</Caption1>
            </Material>
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
            >
                <DiscussionReplyMeta slot render=meta_view />
                <DiscussionReplyMenuExtras slot items=reply_menu_extras />
            </DiscussionThread>
        </div>
    }
}
