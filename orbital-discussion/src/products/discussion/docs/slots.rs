use leptos::prelude::*;
use orbital_macros::component_doc;

/// Leptos slot regions for structural thread composition.
///
/// # When to use
///
/// - Replacing the default toolbar, empty state, or composer placement.
/// - Embedding host-owned chrome without callback props.
///
/// # Usage
///
/// Pass slot children to [`DiscussionThread`]: [`DiscussionThreadToolbar`], [`DiscussionEmptyView`],
/// [`DiscussionComposerSlot`], [`DiscussionComposerTools`], [`DiscussionComposerHint`], and
/// reply section slots such as [`DiscussionReplyMeta`].
///
/// # Examples
///
/// ## Custom toolbar with sample thread
/// Host-owned toolbar buttons above a short thread with two replies.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::slots_thread;
/// use crate::{DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionThreadToolbar, DiscussionViewMode};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(slots_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-slots-preview">
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             features=features
///         >
///             <DiscussionThreadToolbar slot>
///                 <div data-testid="discussion-custom-toolbar" class="orbital-discussion__toolbar">
///                     <span data-testid="discussion-custom-toolbar-expand">
///                         <Button
///                             appearance=ButtonAppearance::Secondary
///                             on:click=move |_| set_collapsed.set(HashSet::new())
///                         >
///                             "Expand all"
///                         </Button>
///                     </span>
///                     <span data-testid="discussion-custom-toolbar-collapse">
///                         <Button
///                             appearance=ButtonAppearance::Secondary
///                             on:click=move |_| set_collapsed.update(|set| { set.insert("slots-reply".into()); })
///                         >
///                             "Collapse replies"
///                         </Button>
///                     </span>
///                     <span class="orbital-discussion__toolbar-label">"Custom slot toolbar"</span>
///                 </div>
///             </DiscussionThreadToolbar>
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-slots",
    preview_label = "Discussion Slots",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionSlotsDoc() -> impl IntoView {
    view! { () }
}
