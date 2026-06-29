use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, ButtonType, Caption1, Flex, FlexAlign, FlexGap, Toolbar,
};
use orbital_macros::component_doc;

use crate::{navigate_to_reply, use_discussion};

/// Toolbar links for reply-anchor doc previews.
#[component]
pub fn DiscussionReplyAnchorToolbar() -> impl IntoView {
    let ctx = use_discussion();
    view! {
        <div class="orbital-discussion__toolbar" data-testid="discussion-reply-anchor-toolbar">
            <Toolbar attr:aria-label="Reply permalinks">
                <Flex align=FlexAlign::Center gap=FlexGap::Small>
                    <Caption1>"Jump to:"</Caption1>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        button_type=ButtonType::Button
                        attr:data-testid="discussion-reply-anchor-d-l4"
                        on:click=move |_| navigate_to_reply(ctx, "d-l4")
                    >
                        "#reply-d-l4"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        button_type=ButtonType::Button
                        attr:data-testid="discussion-reply-anchor-d-l2"
                        on:click=move |_| navigate_to_reply(ctx, "d-l2")
                    >
                        "#reply-d-l2"
                    </Button>
                </Flex>
            </Toolbar>
        </div>
    }
}

/// Deep-link navigation to a nested reply by id.
///
/// # When to use
///
/// - Permalinks such as `/post/123#reply-abc` that must reveal a deeply nested reply.
/// - In-app notifications that jump to a specific reply in a thread.
///
/// # Usage
///
/// Parse the URL fragment with [`parse_reply_anchor_hash`](crate::parse_reply_anchor_hash),
/// then call [`navigate_to_reply`](crate::navigate_to_reply) from a descendant of
/// [`DiscussionProvider`](crate::DiscussionProvider) (typically inside a thread slot or
/// host wrapper after mount).
///
/// The helper uncollapses ancestors on the path, drills focus through show-more barriers
/// in tree mode, and scrolls the target row (`[data-reply-id]`) into view.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep focus controlled in the host store — `navigate_to_reply` updates `set_focus` for you.
/// * Use stable reply ids in URL fragments (`#reply-{id}` or `#{id}`).
/// * Enable `DiscussionFeatures::FOCUS_NAVIGATION` when depth caps apply.
///
/// ## Don'ts
///
/// * Do not call `navigate_to_reply` before the thread has mounted — the scroll target must exist.
///
/// # See also
///
/// * [`DiscussionTreeNavigationDoc`](crate::products::discussion::docs::tree_navigation::DiscussionTreeNavigationDoc)
/// * [`DiscussionEventsDoc`](crate::products::discussion::docs::events::DiscussionEventsDoc)
///
/// # Examples
///
/// ## Jump to a nested reply
/// Five-level fixture with toolbar links that call `navigate_to_reply` for `d-l4`.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::deep_thread;
/// use crate::products::discussion::docs::reply_anchor::DiscussionReplyAnchorToolbar;
/// use crate::{
///     DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionThreadToolbar,
///     DiscussionViewMode,
/// };
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(deep_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-reply-anchor-preview">
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             max_visible_depth=4u32
///             features=features
///         >
///             <DiscussionThreadToolbar slot>
///                 <DiscussionReplyAnchorToolbar />
///             </DiscussionThreadToolbar>
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-reply-anchor",
    preview_label = "Reply Anchor",
    preview_icon = icondata::AiLinkOutlined,
)]
#[component]
pub fn DiscussionReplyAnchorDoc() -> impl IntoView {
    view! { () }
}
