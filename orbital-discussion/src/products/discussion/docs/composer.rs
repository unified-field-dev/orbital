use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::preview::fixtures::{append_composer_reply, sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
use crate::{
    DiscussionComposerHint, DiscussionComposerSubmit, DiscussionEvents, DiscussionFeatures,
    DiscussionFocus, DiscussionThread, DiscussionViewMode,
};

/// Reply composer input, reply banner, format toolbar, and submit flow.
///
/// # When to use
///
/// - Bottom-of-thread reply entry with markdown draft state.
/// - Reply-to banner when responding to a specific author.
///
/// # Usage
///
/// Enable `DiscussionFeatures::MARKDOWN` for the format toolbar, `ATTACHMENTS` for attach,
/// and `CITATIONS` for citation drafts. Wire `events.on_submit` to persist drafts.
/// Nest [`DiscussionComposerTools`] and [`DiscussionComposerHint`] for host-owned composer chrome.
///
/// # See also
///
/// * [`DiscussionIntegrationDoc`](crate::products::discussion::docs::integration::DiscussionIntegrationDoc)
/// * [`DiscussionLocalizationDoc`](crate::products::discussion::docs::localization::DiscussionLocalizationDoc)
/// * [`DiscussionCustomAreasDoc`](crate::products::discussion::docs::custom_areas::DiscussionCustomAreasDoc)
///
/// # Examples
///
/// ## Composer with format toolbar and submit log
/// Click Reply on a row, type a message, and submit to update the log panel and thread.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{append_composer_reply, sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
/// use crate::{
///     DiscussionComposerHint, DiscussionComposerSubmit, DiscussionEvents, DiscussionFeatures,
///     DiscussionFocus, DiscussionThread, DiscussionViewMode,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::Caption1;
/// use std::collections::HashSet;
/// let (replies, set_replies) = signal(sample_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let (reply_to, set_reply_to) = signal(None::<String>);
/// let (last_submit, set_last_submit) = signal(String::new());
/// let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));
/// let events = DiscussionEvents {
///     on_reply_click: Some(Callback::new(move |id| set_reply_to.set(Some(id)))),
///     on_submit: Some(Callback::new(move |payload: DiscussionComposerSubmit| {
///         set_replies.update(|list| append_composer_reply(list, &payload, PREVIEW_VIEWER_AUTHOR_ID, "Jordan Lee"));
///         let parent = payload.parent_id.unwrap_or_else(|| "none".to_string());
///         set_last_submit.set(format!(
///             "{parent}:{}:attachments={}:citations={}",
///             payload.body_markdown,
///             payload.attachments.len(),
///             payload.citations.len(),
///         ));
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN
///     | DiscussionFeatures::FOCUS_NAVIGATION
///     | DiscussionFeatures::ATTACHMENTS
///     | DiscussionFeatures::CITATIONS;
/// view! {
///     <div data-testid="discussion-composer-preview">
///         <pre data-testid="discussion-composer-submit-log">{move || last_submit.get()}</pre>
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             reply_to=Signal::derive(move || reply_to.get())
///             set_reply_to=set_reply_to
///             current_user_id=current_user_id
///             events=events
///             features=features
///         >
///             <DiscussionComposerHint slot>
///                 <Caption1 class="orbital-discussion__composer-hint">
///                     "Markdown supported. Citations appear in the reply body."
///                 </Caption1>
///             </DiscussionComposerHint>
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-composer",
    preview_label = "Discussion Composer",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionComposerDoc() -> impl IntoView {
    let (replies, set_replies) = signal(sample_thread());
    let (focus, set_focus) = signal(DiscussionFocus::Root);
    let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
    let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());
    let (reply_to, set_reply_to) = signal(None::<String>);
    let (last_submit, set_last_submit) = signal(String::new());
    let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));

    let events = DiscussionEvents {
        on_reply_click: Some(Callback::new(move |id| set_reply_to.set(Some(id)))),
        on_submit: Some(Callback::new(move |payload: DiscussionComposerSubmit| {
            set_replies.update(|list| {
                append_composer_reply(list, &payload, PREVIEW_VIEWER_AUTHOR_ID, "Jordan Lee")
            });
            let parent = payload.parent_id.unwrap_or_else(|| "none".to_string());
            set_last_submit.set(format!(
                "{parent}:{}:attachments={}:citations={}",
                payload.body_markdown,
                payload.attachments.len(),
                payload.citations.len(),
            ));
        })),
        ..Default::default()
    };

    let features = DiscussionFeatures::MARKDOWN
        | DiscussionFeatures::FOCUS_NAVIGATION
        | DiscussionFeatures::ATTACHMENTS
        | DiscussionFeatures::CITATIONS;

    view! {
        <div data-testid="discussion-composer-preview">
            <pre data-testid="discussion-composer-submit-log">{move || last_submit.get()}</pre>
            <DiscussionThread
                replies=Signal::derive(move || replies.get())
                focus=Signal::derive(move || focus.get())
                set_focus=set_focus
                view_mode=Signal::derive(move || view_mode.get())
                set_view_mode=set_view_mode
                collapsed=Signal::derive(move || collapsed.get())
                set_collapsed=set_collapsed
                reply_to=Signal::derive(move || reply_to.get())
                set_reply_to=set_reply_to
                current_user_id=current_user_id
                events=events
                features=features
            >
                <DiscussionComposerHint slot>
                    <orbital_core_components::Caption1 class="orbital-discussion__composer-hint">
                        "Markdown supported. Citations appear in the reply body."
                    </orbital_core_components::Caption1>
                </DiscussionComposerHint>
            </DiscussionThread>
        </div>
    }
}
