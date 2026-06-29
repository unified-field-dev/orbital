use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::preview::fixtures::thread_with_file_parts;
use crate::{DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};

/// Text, file, and custom reply part renderers.
///
/// # When to use
///
/// - Markdown bodies, file attachments, and host-defined custom parts.
///
/// # Usage
///
/// Enable `DiscussionFeatures::CUSTOM_PARTS` and nest [`DiscussionReplyPart`] on
/// [`DiscussionThread`] for host-defined part renderers.
///
/// # Best Practices
///
/// ## Do's
///
/// * Enable `DiscussionFeatures::MARKDOWN` for fenced code blocks in text parts.
/// * Keep file part URLs host-controlled — the crate renders links, not uploads.
///
/// ## Don'ts
///
/// * Do not embed large binary payloads in `DiscussionPart::Custom` — pass references instead.
///
/// # See also
///
/// * [`DiscussionCitationsDoc`](crate::products::discussion::docs::citations::DiscussionCitationsDoc)
/// * [`DiscussionCustomAreasDoc`](crate::products::discussion::docs::custom_areas::DiscussionCustomAreasDoc)
///
/// # Examples
///
/// ## Code block and file parts
/// Fenced code block, inline image preview, and download link.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::thread_with_file_parts;
/// use crate::{DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(thread_with_file_parts());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-parts-preview">
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
    preview_slug = "discussion-parts",
    preview_label = "Discussion Parts",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionPartsDoc() -> impl IntoView {
    let (replies, _set_replies) = signal(thread_with_file_parts());
    let (focus, set_focus) = signal(DiscussionFocus::Root);
    let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
    let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());
    let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;

    view! {
        <div data-testid="discussion-parts-preview">
            <DiscussionThread
                replies=Signal::derive(move || replies.get())
                focus=Signal::derive(move || focus.get())
                set_focus=set_focus
                view_mode=Signal::derive(move || view_mode.get())
                set_view_mode=set_view_mode
                collapsed=Signal::derive(move || collapsed.get())
                set_collapsed=set_collapsed
                features=features
            />
        </div>
    }
}
