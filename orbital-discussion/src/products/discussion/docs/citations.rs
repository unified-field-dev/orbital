use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, ButtonType};
use orbital_macros::component_doc;
use std::sync::Arc;

use crate::preview::fixtures::thread_with_custom_citations;
use crate::{
    CitationAffordanceView, CitationMenuExtrasView, CitationRenderContext,
    DiscussionCitationAffordance, DiscussionCitationMenuExtras, DiscussionEvents,
    DiscussionFeatures, DiscussionFocus, DiscussionMenuItem, DiscussionThread, DiscussionViewMode,
};

/// Structured citation rows and per-citation menus.
///
/// # When to use
///
/// - Replies that reference external sources with title, url, and excerpt.
///
/// # Usage
///
/// Enable `DiscussionFeatures::CITATIONS`, wire `events.on_citation_action`, and optionally
/// nest [`DiscussionCitationAffordance`] / [`DiscussionCitationMenuExtras`] slot children
/// for host-owned chrome.
///
/// # Best Practices
///
/// ## Do's
///
/// * Wire `events.on_citation_action` for Open/Copy and custom menu selections.
/// * Keep citation titles host-controlled — the crate renders the list, not persistence.
///
/// ## Don'ts
///
/// * Do not invent citation URLs in the crate — host adapter owns source links.
///
/// # See also
///
/// * [`DiscussionPartsDoc`](crate::products::discussion::docs::parts::DiscussionPartsDoc)
/// * [`DiscussionComposerDoc`](crate::products::discussion::docs::composer::DiscussionComposerDoc)
///
/// # Examples
///
/// ## Citation list with affordance, menu extras, and action log
/// Open a menu action to update the log; citation-ref links appear in markdown bodies.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::thread_with_custom_citations;
/// use crate::{
///     CitationAffordanceView, CitationMenuExtrasView, CitationRenderContext,
///     DiscussionCitationAffordance, DiscussionCitationMenuExtras, DiscussionEvents,
///     DiscussionFeatures, DiscussionFocus, DiscussionMenuItem, DiscussionThread,
///     DiscussionViewMode,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance, ButtonType};
/// use std::collections::HashSet;
/// use std::sync::Arc;
/// let affordance_view: CitationAffordanceView = Arc::new(|ctx: CitationRenderContext| {
///     Some(view! {
///         <Button
///             appearance=ButtonAppearance::Subtle
///             button_type=ButtonType::Button
///             attr:data-testid=format!("discussion-citation-affordance-btn-{}", ctx.citation.id)
///         >
///             "Agree"
///         </Button>
///     }.into_any())
/// });
/// let menu_extras: CitationMenuExtrasView = Arc::new(|_ctx: CitationRenderContext| {
///     vec![
///         DiscussionMenuItem {
///             id: "mark_agree".into(),
///             label: "Mark agree".into(),
///             disabled: None,
///         },
///         DiscussionMenuItem {
///             id: "mark_dispute".into(),
///             label: "Mark dispute".into(),
///             disabled: None,
///         },
///     ]
/// });
/// let (replies, _set_replies) = signal(thread_with_custom_citations());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let (log, set_log) = signal(String::new());
/// let events = DiscussionEvents {
///     on_citation_action: Some(Callback::new(move |(citation, action): (crate::DiscussionCitation, String)| {
///         set_log.set(format!("{}:{}", citation.id, action));
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::CITATIONS;
/// view! {
///     <div data-testid="discussion-citations-preview">
///         <pre data-testid="discussion-citations-action-log">{move || log.get()}</pre>
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
///             <DiscussionCitationAffordance slot render=affordance_view />
///             <DiscussionCitationMenuExtras slot items=menu_extras />
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-citations",
    preview_label = "Discussion Citations",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionCitationsDoc() -> impl IntoView {
    let affordance_view: CitationAffordanceView = Arc::new(|ctx: CitationRenderContext| {
        Some(
            view! {
                <Button
                    appearance=ButtonAppearance::Subtle
                    button_type=ButtonType::Button
                    attr:data-testid=format!("discussion-citation-affordance-btn-{}", ctx.citation.id)
                >
                    "Agree"
                </Button>
            }
            .into_any(),
        )
    });
    let menu_extras: CitationMenuExtrasView = Arc::new(|_ctx: CitationRenderContext| {
        vec![
            DiscussionMenuItem {
                id: "mark_agree".into(),
                label: "Mark agree".into(),
                disabled: None,
            },
            DiscussionMenuItem {
                id: "mark_dispute".into(),
                label: "Mark dispute".into(),
                disabled: None,
            },
        ]
    });

    let (replies, _set_replies) = signal(thread_with_custom_citations());
    let (focus, set_focus) = signal(DiscussionFocus::Root);
    let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
    let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());
    let (log, set_log) = signal(String::new());

    let events = DiscussionEvents {
        on_citation_action: Some(Callback::new(
            move |(citation, action): (crate::DiscussionCitation, String)| {
                set_log.set(format!("{}:{}", citation.id, action));
            },
        )),
        ..Default::default()
    };

    let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::CITATIONS;

    view! {
        <div data-testid="discussion-citations-preview">
            <pre data-testid="discussion-citations-action-log">{move || log.get()}</pre>
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
                <DiscussionCitationAffordance slot render=affordance_view />
                <DiscussionCitationMenuExtras slot items=menu_extras />
            </DiscussionThread>
        </div>
    }
}
