use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance};
use orbital_macros::component_doc;

use crate::preview::fixtures::empty_thread;
use crate::{DiscussionEmptyView, DiscussionFocus, DiscussionThread, DiscussionViewMode};

/// Empty thread and loading skeleton overlays.
///
/// # When to use
///
/// - Initial thread fetch before replies arrive.
/// - Zero-reply threads with the built-in empty message or a custom empty slot.
///
/// # Examples
///
/// ## Empty and loading states
/// Toggle between custom empty view and skeleton loading overlay.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::empty_thread;
/// use crate::{DiscussionEmptyView, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use std::collections::HashSet;
/// let (replies, set_replies) = signal(empty_thread());
/// let (loading, set_loading) = signal(false);
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// view! {
///     <div data-testid="discussion-overlays-preview">
///         <div style="display:flex;gap:8px;margin-bottom:8px;">
///             <Button appearance=ButtonAppearance::Secondary on:click=move |_| set_loading.set(false)>"Empty"</Button>
///             <Button appearance=ButtonAppearance::Secondary on:click=move |_| set_loading.set(true)>"Loading"</Button>
///         </div>
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             loading=Some(Signal::derive(move || loading.get()))
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///         >
///             <DiscussionEmptyView slot>
///                 <div data-testid="discussion-custom-empty">
///                     "No replies yet — start the discussion."
///                 </div>
///             </DiscussionEmptyView>
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-overlays",
    preview_label = "Discussion Overlays",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionOverlaysDoc() -> impl IntoView {
    let (replies, _set_replies) = signal(empty_thread());
    let (loading, set_loading) = signal(false);
    let (focus, set_focus) = signal(DiscussionFocus::Root);
    let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
    let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());

    view! {
        <div data-testid="discussion-overlays-preview">
            <div class="orbital-discussion__overlays-controls" style="display:flex;gap:8px;margin-bottom:8px;">
                <span data-testid="discussion-overlays-empty-btn">
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on:click=move |_| set_loading.set(false)
                    >
                        "Empty"
                    </Button>
                </span>
                <span data-testid="discussion-overlays-loading-btn">
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on:click=move |_| set_loading.set(true)
                    >
                        "Loading"
                    </Button>
                </span>
            </div>
            <DiscussionThread
                replies=Signal::derive(move || replies.get())
                loading=Some(Signal::derive(move || loading.get()))
                focus=Signal::derive(move || focus.get())
                set_focus=set_focus
                view_mode=Signal::derive(move || view_mode.get())
                set_view_mode=set_view_mode
                collapsed=Signal::derive(move || collapsed.get())
                set_collapsed=set_collapsed
            >
                <DiscussionEmptyView slot>
                    <div data-testid="discussion-custom-empty">
                        "No replies yet — start the discussion."
                    </div>
                </DiscussionEmptyView>
            </DiscussionThread>
        </div>
    }
}
