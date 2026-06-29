use leptos::prelude::*;
use orbital_macros::component_doc;

/// Tree, flat, and compact view mode projections.
///
/// # When to use
///
/// - Switching between nested tree layout and chronological flat lists.
/// - Dense single-column compact layouts for side panels.
///
/// # Usage
///
/// Bind `view_mode` and use the default toolbar Select, or pass a custom `DiscussionThreadToolbar` slot.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use Flat mode for chronological audit trails; Tree mode for nested forum reading.
/// * Override view mode labels through [`DiscussionLocale`](crate::DiscussionLocale) for i18n.
///
/// ## Don'ts
///
/// * Do not mutate the reply list when switching modes — projection is layout-only.
///
/// # See also
///
/// * [`DiscussionRepliesDoc`](crate::products::discussion::docs::replies::DiscussionRepliesDoc)
/// * [`DiscussionTreeNavigationDoc`](crate::products::discussion::docs::tree_navigation::DiscussionTreeNavigationDoc)
///
/// # Examples
///
/// ## View mode picker with date dividers
/// Flat mode shows chronological list with date breaks across two calendar days.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::flat_thread_with_dates;
/// use crate::{DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(flat_thread_with_dates());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Flat);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-view-modes-preview">
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             features=features
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-view-modes",
    preview_label = "View Modes",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionViewModesDoc() -> impl IntoView {
    view! { () }
}
