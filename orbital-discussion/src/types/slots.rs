use leptos::prelude::*;

use super::renderers::{
    CitationAffordanceView, CitationMenuExtrasView, CitationMenuView, ReplyFooterView,
    ReplyMenuExtrasView, ReplyMenuView, ReplyMetaView, ReplyPartView,
};

/// Internal slot content consumed by thread subcomponents.
#[derive(Default)]
pub struct DiscussionSlots {
    pub composer: Option<DiscussionComposerSlot>,
    pub empty: Option<DiscussionEmptyView>,
    pub thread_toolbar: Option<DiscussionThreadToolbar>,
    pub composer_tools: Option<DiscussionComposerTools>,
    pub composer_hint: Option<DiscussionComposerHint>,
    pub reply_meta: Option<DiscussionReplyMeta>,
    pub reply_footer: Option<DiscussionReplyFooterRegion>,
    pub reply_part: Option<DiscussionReplyPart>,
    pub reply_menu: Option<DiscussionReplyMenu>,
    pub reply_menu_extras: Option<DiscussionReplyMenuExtras>,
    pub citation_affordance: Option<DiscussionCitationAffordance>,
    pub citation_menu_extras: Option<DiscussionCitationMenuExtras>,
    pub citation_menu: Option<DiscussionCitationMenu>,
}

impl DiscussionSlots {
    #[allow(clippy::too_many_arguments)]
    pub fn from_slot_props(
        composer: Option<DiscussionComposerSlot>,
        empty: Option<DiscussionEmptyView>,
        thread_toolbar: Option<DiscussionThreadToolbar>,
        composer_tools: Option<DiscussionComposerTools>,
        composer_hint: Option<DiscussionComposerHint>,
        reply_meta: Option<DiscussionReplyMeta>,
        reply_footer: Option<DiscussionReplyFooterRegion>,
        reply_part: Option<DiscussionReplyPart>,
        reply_menu: Option<DiscussionReplyMenu>,
        reply_menu_extras: Option<DiscussionReplyMenuExtras>,
        citation_affordance: Option<DiscussionCitationAffordance>,
        citation_menu_extras: Option<DiscussionCitationMenuExtras>,
        citation_menu: Option<DiscussionCitationMenu>,
    ) -> Self {
        Self {
            composer,
            empty,
            thread_toolbar,
            composer_tools,
            composer_hint,
            reply_meta,
            reply_footer,
            reply_part,
            reply_menu,
            reply_menu_extras,
            citation_affordance,
            citation_menu_extras,
            citation_menu,
        }
    }
}

/// Custom composer region at the thread bottom.
#[slot]
pub struct DiscussionComposerSlot {
    pub(crate) children: ChildrenFn,
}

/// Custom empty-state region when the reply list is empty.
#[slot]
pub struct DiscussionEmptyView {
    pub(crate) children: ChildrenFn,
}

/// Custom toolbar region above the reply list.
#[slot]
pub struct DiscussionThreadToolbar {
    pub(crate) children: ChildrenFn,
}

/// Custom tools region inside the composer (Phase 3).
#[slot]
pub struct DiscussionComposerTools {
    pub(crate) children: ChildrenFn,
}

/// Custom hint region below the composer input (Phase 3).
#[slot]
pub struct DiscussionComposerHint {
    pub(crate) children: ChildrenFn,
}

/// Custom meta region below the primary header row on each reply.
#[slot]
pub struct DiscussionReplyMeta {
    #[prop(into)]
    pub render: ReplyMetaView,
}

/// Custom footer region below the reply body on each reply.
#[slot]
pub struct DiscussionReplyFooterRegion {
    #[prop(into)]
    pub render: ReplyFooterView,
}

/// Custom part renderer for reply body parts. Return `None` to fall through to built-ins.
#[slot]
pub struct DiscussionReplyPart {
    #[prop(into)]
    pub render: ReplyPartView,
}

/// Fully replaces the default reply overflow menu beside Reply.
#[slot]
pub struct DiscussionReplyMenu {
    #[prop(into)]
    pub render: ReplyMenuView,
}

/// Extra menu items on the default reply overflow menu beside Reply.
#[slot]
pub struct DiscussionReplyMenuExtras {
    #[prop(into)]
    pub items: ReplyMenuExtrasView,
}

/// Custom chrome left of the citation overflow menu.
#[slot]
pub struct DiscussionCitationAffordance {
    #[prop(into)]
    pub render: CitationAffordanceView,
}

/// Extra menu items appended to the default citation menu.
#[slot]
pub struct DiscussionCitationMenuExtras {
    #[prop(into)]
    pub items: CitationMenuExtrasView,
}

/// Fully replaces the default citation overflow menu.
#[slot]
pub struct DiscussionCitationMenu {
    #[prop(into)]
    pub render: CitationMenuView,
}
