use leptos::prelude::*;
use orbital_macros::component_doc;

/// Tree depth limits, focus drill-in, and show-more navigation.
///
/// # When to use
///
/// - Deep threads where nested replies exceed the visible depth cap.
/// - Reddit-style focus navigation into a reply branch.
///
/// # Usage
///
/// Set `max_visible_depth` (default 4) and enable `DiscussionFeatures::FOCUS_NAVIGATION`.
/// Click "Show N more replies" to drill in; use the go-back bar to return.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep focus state controlled in the host store when persisting drill-in navigation.
/// * Localize go-back and show-more labels via [`DiscussionLocale`](crate::DiscussionLocale).
///
/// ## Don'ts
///
/// * Do not disable `FOCUS_NAVIGATION` when depth caps are in use — users need drill-in affordances.
///
/// # See also
///
/// * [`DiscussionRepliesDoc`](crate::products::discussion::docs::replies::DiscussionRepliesDoc)
/// * [`DiscussionReplyAnchorDoc`](crate::products::discussion::docs::reply_anchor::DiscussionReplyAnchorDoc)
/// * [`DiscussionViewModesDoc`](crate::products::discussion::docs::view_modes::DiscussionViewModesDoc)
/// * [`DiscussionLocalizationDoc`](crate::products::discussion::docs::localization::DiscussionLocalizationDoc)
///
/// # Examples
///
/// ## Deep thread with drill-in
/// Five-level fixture with show-more at depth cap and collapse toggle.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::deep_thread;
/// use crate::{DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(deep_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-tree-navigation-preview">
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
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-tree-navigation",
    preview_label = "Tree Navigation",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionTreeNavigationDoc() -> impl IntoView {
    view! { () }
}
