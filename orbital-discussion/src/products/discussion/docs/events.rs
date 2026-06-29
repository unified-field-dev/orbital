use leptos::prelude::*;
use orbital_macros::component_doc;

/// Event callbacks for focus, collapse, and host integration.
///
/// # When to use
///
/// - Persisting focus navigation or collapse state to the host store.
/// - Logging user interactions during integration testing.
///
/// # Usage
///
/// Pass `events` with `on_focus_change` and `on_collapse_change` on `DiscussionThread`.
///
/// # Examples
///
/// ## Event log panel
/// Displays focus and collapse events fired by the thread.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::deep_thread;
/// use crate::{DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(deep_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let (log, set_log) = signal(String::new());
/// let events = DiscussionEvents {
///     on_focus_change: Some(Callback::new(move |f| {
///         set_log.set(format!("focus: {f:?}"));
///     })),
///     on_collapse_change: Some(Callback::new(move |(id, c)| {
///         set_log.set(format!("collapse: {id} -> {c}"));
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-events-preview">
///         <pre data-testid="discussion-events-log">{move || log.get()}</pre>
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
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-events",
    preview_label = "Discussion Events",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionEventsDoc() -> impl IntoView {
    view! { () }
}
