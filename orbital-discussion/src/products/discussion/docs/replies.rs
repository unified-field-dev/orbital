use leptos::prelude::*;
use orbital_macros::component_doc;

/// Displays reply count from [`use_discussion`] for hook verification previews.
#[component]
pub fn DiscussionHookReplyCount() -> impl IntoView {
    let discussion = crate::use_discussion();
    let count = Memo::new(move |_| discussion.replies.get().len());

    view! {
        <span data-testid="discussion-hook-reply-count">{move || count.get()}</span>
    }
}

/// Reply row anatomy, metadata, and tree rendering.
///
/// # When to use
///
/// - Understanding how flat reply records project into nested thread rows.
/// - Planning custom header and footer regions on reply nodes.
///
/// # Usage
///
/// Rich reply chrome renders avatars, metadata labels, markdown bodies, and agent role badges.
/// Density follows the active Orbital theme via `use_theme_options().density`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep reply data flat in the host store; use the graph engine for tree projection.
/// * Enable `DiscussionFeatures::MARKDOWN` when rendering formatted reply bodies.
///
/// ## Don'ts
///
/// * Do not embed network fetching inside reply row components.
///
/// # See also
///
/// * [`DiscussionTreeNavigationDoc`](crate::products::discussion::docs::tree_navigation::DiscussionTreeNavigationDoc)
/// * [`DiscussionViewModesDoc`](crate::products::discussion::docs::view_modes::DiscussionViewModesDoc)
/// * [`DiscussionPartsDoc`](crate::products::discussion::docs::parts::DiscussionPartsDoc)
///
/// # Examples
///
/// ## Rich reply thread
/// Nested fixture with markdown body, OP label, edited indicator, and agent role badge.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{append_composer_reply, sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
/// use crate::products::discussion::docs::replies::DiscussionHookReplyCount;
/// use crate::{
///     DiscussionComposerSubmit, DiscussionDefaultThreadToolbar, DiscussionEvents,
///     DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionThreadToolbar,
///     DiscussionViewMode,
/// };
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, set_replies) = signal(sample_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));
/// let events = DiscussionEvents {
///     on_submit: Some(Callback::new(move |payload: DiscussionComposerSubmit| {
///         set_replies.update(|list| {
///             append_composer_reply(list, &payload, PREVIEW_VIEWER_AUTHOR_ID, "Jordan Lee");
///         });
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-thread-preview">
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             current_user_id=current_user_id
///             events=events
///             features=features
///         >
///             <DiscussionThreadToolbar slot>
///                 <DiscussionDefaultThreadToolbar />
///                 <DiscussionHookReplyCount />
///             </DiscussionThreadToolbar>
///         </DiscussionThread>
///     </div>
/// }
/// ```
///
/// ## Density spacing
/// Toggle compact, default, and spacious density to adjust row padding and avatar size.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
/// use crate::{DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(sample_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-density-preview">
///         <Flex align=FlexAlign::Center gap=FlexGap::Medium>
///             <ThemeDensityStepper />
///         </Flex>
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             current_user_id=current_user_id
///             features=features
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-replies",
    preview_label = "Discussion Replies",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionRepliesDoc() -> impl IntoView {
    view! { () }
}
