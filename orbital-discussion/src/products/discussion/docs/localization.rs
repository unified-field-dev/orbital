//! [`DiscussionLocalizationDoc`] — locale strings preview.

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, Select, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::preview::fixtures::{sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
#[cfg(feature = "preview")]
use crate::{
    locale_signal, DiscussionFeatures, DiscussionFocus, DiscussionLocale, DiscussionThread,
    DiscussionViewMode,
};

/// Override discussion chrome strings for localization.
///
/// # When to use
///
/// - Multilingual forum or community products
/// - Custom product copy on composer, navigation, and view mode picker
///
/// # Usage
///
/// 1. Pass an initial [`DiscussionLocale`] on [`DiscussionThread`] via the `locale` prop, or
///    provide a reactive `locale_read` signal for live language switching.
/// 2. Read merged strings in custom slot children with [`use_discussion_locale()`].
/// 3. Override individual fields by building a full [`DiscussionLocale`] struct — partial merge
///    is not yet supported; copy from [`DiscussionLocale::english()`] and replace fields.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use discussion vocabulary (`reply`, `thread`) in default and translated strings.
/// * Provide `composer_aria_label` even when a visible placeholder is shown — screen readers
///   need an accessible name independent of placeholder text.
///
/// ## Don'ts
///
/// * Do not copy third-party locale JSON verbatim — define clean-room strings per language.
///
/// # Accessibility
///
/// Phase 5 accessibility audit (discussion thread, not modal):
///
/// | Check | Result |
/// |-------|--------|
/// Focus order | Pass — toolbar, scroll area (focus-back + reply list), then composer slot |
/// Thread semantics | Pass — reply lists use `role="list"` |
/// Collapse toggle | Pass — `aria-label` on collapse `Button`; Enter/Space activate via Orbital `Button` |
/// Show-more / go-back | Pass — visible labels plus matching `aria-label` on drill-in controls |
/// Composer | Pass — visually hidden `<label class="sr-only">` linked to textarea `id` |
/// Focus trap | N/A — thread is not a modal; no trap required |
///
/// # See also
///
/// * [`DiscussionRepliesDoc`](crate::products::discussion::docs::replies::DiscussionRepliesDoc) — reply row chrome
/// * [`DiscussionTreeNavigationDoc`](crate::products::discussion::docs::tree_navigation::DiscussionTreeNavigationDoc) — go-back and show-more strings
/// * [`DiscussionViewModesDoc`](crate::products::discussion::docs::view_modes::DiscussionViewModesDoc) — view mode picker labels
///
/// # Examples
///
/// ## English and French locale toggle
/// Language select switches composer placeholder, toolbar labels, and navigation chrome.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
/// use crate::{locale_signal, DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionLocale, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, Select, ThemeDensityStepper};
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(sample_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let locale_pick = RwSignal::new("en".to_string());
/// let (locale_rw, locale_read) = locale_signal(DiscussionLocale::english());
/// Effect::new(move |_| {
///     locale_rw.set(match locale_pick.get().as_str() {
///         "fr" => DiscussionLocale::french(),
///         _ => DiscussionLocale::english(),
///     });
/// });
/// let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-localization-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <Select bind=locale_pick attr:data-testid="discussion-locale-select">
///                 <option value="en">"English"</option>
///                 <option value="fr">"Français"</option>
///             </Select>
///             <DiscussionThread
///                 replies=Signal::derive(move || replies.get())
///                 focus=Signal::derive(move || focus.get())
///                 set_focus=set_focus
///                 view_mode=Signal::derive(move || view_mode.get())
///                 set_view_mode=set_view_mode
///                 collapsed=Signal::derive(move || collapsed.get())
///                 set_collapsed=set_collapsed
///                 current_user_id=current_user_id
///                 locale_read=locale_read
///                 features=features
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-localization",
    preview_label = "Discussion Localization",
    preview_icon = icondata::AiGlobalOutlined,
)]
#[component]
pub fn DiscussionLocalizationDoc() -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let (replies, _set_replies) = signal(sample_thread());
        let (focus, set_focus) = signal(DiscussionFocus::Root);
        let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
        let (collapsed, set_collapsed) = signal(std::collections::HashSet::<String>::new());
        let locale_pick = RwSignal::new("en".to_string());
        let (locale_rw, locale_read) = locale_signal(DiscussionLocale::english());

        Effect::new(move |_| {
            locale_rw.set(match locale_pick.get().as_str() {
                "fr" => DiscussionLocale::french(),
                _ => DiscussionLocale::english(),
            });
        });

        let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));
        let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;

        view! {
            <div data-testid="discussion-localization-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <Select bind=locale_pick attr:data-testid="discussion-locale-select">
                        <option value="en">"English"</option>
                        <option value="fr">"Français"</option>
                    </Select>
                    <DiscussionThread
                        replies=Signal::derive(move || replies.get())
                        focus=Signal::derive(move || focus.get())
                        set_focus=set_focus
                        view_mode=Signal::derive(move || view_mode.get())
                        set_view_mode=set_view_mode
                        collapsed=Signal::derive(move || collapsed.get())
                        set_collapsed=set_collapsed
                        current_user_id=current_user_id
                        locale_read=locale_read
                        features=features
                    />
                </Flex>
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        view! { () }
    }
}
