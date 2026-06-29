use std::sync::Arc;

use leptos::prelude::*;

use super::{DiscussionCitation, DiscussionPart, DiscussionReply};

/// Context passed to reply render callbacks.
#[derive(Clone, Debug)]
pub struct ReplyRenderContext {
    pub reply: DiscussionReply,
}

impl ReplyRenderContext {
    pub fn new(reply: DiscussionReply) -> Self {
        Self { reply }
    }
}

/// Custom meta region below the primary header row (reactions, votes, actions).
pub type ReplyMetaView = Arc<dyn Fn(ReplyRenderContext) -> AnyView + Send + Sync>;

/// Custom footer region below the reply body.
pub type ReplyFooterView = Arc<dyn Fn(ReplyRenderContext) -> AnyView + Send + Sync>;

/// Custom part renderer. Return `None` to fall through to built-in renderers.
pub type ReplyPartView =
    Arc<dyn Fn(ReplyRenderContext, DiscussionPart) -> Option<AnyView> + Send + Sync>;

/// Context passed to citation menu render callbacks.
#[derive(Clone, Debug)]
pub struct CitationRenderContext {
    pub reply: DiscussionReply,
    pub citation: DiscussionCitation,
    /// 1-based display index shown beside the citation title.
    pub index: usize,
}

impl CitationRenderContext {
    pub fn new(reply: DiscussionReply, citation: DiscussionCitation, index: usize) -> Self {
        Self {
            reply,
            citation,
            index,
        }
    }
}

/// Custom citation overflow menu. When `None`, built-in open/copy actions render.
pub type CitationMenuView = Arc<dyn Fn(CitationRenderContext) -> AnyView + Send + Sync>;

/// Host-defined menu item for reply or citation overflow menus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscussionMenuItem {
    pub id: String,
    pub label: String,
    pub disabled: Option<bool>,
}

/// Back-compat alias for citation menu extras.
pub type DiscussionCitationMenuItem = DiscussionMenuItem;

/// Custom chrome left of the citation overflow menu (agree/dispute, popover trigger, etc.).
pub type CitationAffordanceView =
    Arc<dyn Fn(CitationRenderContext) -> Option<AnyView> + Send + Sync>;

/// Extra menu items appended to the default citation menu.
pub type CitationMenuExtrasView =
    Arc<dyn Fn(CitationRenderContext) -> Vec<DiscussionMenuItem> + Send + Sync>;

/// Custom reply overflow menu. When `None`, built-in trigger + `reply_menu_extras` render.
pub type ReplyMenuView = Arc<dyn Fn(ReplyRenderContext) -> AnyView + Send + Sync>;

/// Extra menu items on the default reply overflow menu beside Reply.
pub type ReplyMenuExtrasView =
    Arc<dyn Fn(ReplyRenderContext) -> Vec<DiscussionMenuItem> + Send + Sync>;

/// Data-driven render callbacks for reply rows.
#[derive(Clone, Default)]
pub struct DiscussionRenderers {
    pub meta_view: Option<ReplyMetaView>,
    pub footer_view: Option<ReplyFooterView>,
    pub part_view: Option<ReplyPartView>,
    pub reply_menu: Option<ReplyMenuView>,
    pub reply_menu_extras: Option<ReplyMenuExtrasView>,
    pub citation_menu: Option<CitationMenuView>,
    pub citation_affordance_view: Option<CitationAffordanceView>,
    pub citation_menu_extras: Option<CitationMenuExtrasView>,
}

impl DiscussionRenderers {
    /// Build renderers from reply/citation section slots on [`DiscussionThread`](crate::DiscussionThread).
    pub fn from_slots(slots: &super::slots::DiscussionSlots) -> Self {
        Self {
            meta_view: slots.reply_meta.as_ref().map(|slot| slot.render.clone()),
            footer_view: slots.reply_footer.as_ref().map(|slot| slot.render.clone()),
            part_view: slots.reply_part.as_ref().map(|slot| slot.render.clone()),
            reply_menu: slots.reply_menu.as_ref().map(|slot| slot.render.clone()),
            reply_menu_extras: slots
                .reply_menu_extras
                .as_ref()
                .map(|slot| slot.items.clone()),
            citation_menu: slots.citation_menu.as_ref().map(|slot| slot.render.clone()),
            citation_affordance_view: slots
                .citation_affordance
                .as_ref()
                .map(|slot| slot.render.clone()),
            citation_menu_extras: slots
                .citation_menu_extras
                .as_ref()
                .map(|slot| slot.items.clone()),
        }
    }

    /// Merge slot-derived renderers with a deprecated [`DiscussionRenderers`] prop.
    /// Slot values win when both are set for the same field.
    pub fn merge_with_slots(
        slots: &super::slots::DiscussionSlots,
        deprecated: Option<DiscussionRenderers>,
    ) -> Self {
        let from_slots = Self::from_slots(slots);
        let deprecated = deprecated.unwrap_or_default();
        Self {
            meta_view: from_slots.meta_view.or(deprecated.meta_view),
            footer_view: from_slots.footer_view.or(deprecated.footer_view),
            part_view: from_slots.part_view.or(deprecated.part_view),
            reply_menu: from_slots.reply_menu.or(deprecated.reply_menu),
            reply_menu_extras: from_slots
                .reply_menu_extras
                .or(deprecated.reply_menu_extras),
            citation_menu: from_slots.citation_menu.or(deprecated.citation_menu),
            citation_affordance_view: from_slots
                .citation_affordance_view
                .or(deprecated.citation_affordance_view),
            citation_menu_extras: from_slots
                .citation_menu_extras
                .or(deprecated.citation_menu_extras),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn stub_meta(label: &'static str) -> ReplyMetaView {
        Arc::new(move |_ctx: ReplyRenderContext| view! { <span>{label}</span> }.into_any())
    }

    #[test]
    fn merge_with_slots_prefers_slot_over_deprecated() {
        let slots = super::super::slots::DiscussionSlots {
            reply_meta: Some(super::super::slots::DiscussionReplyMeta {
                render: stub_meta("slot"),
            }),
            ..Default::default()
        };
        let deprecated = DiscussionRenderers {
            meta_view: Some(stub_meta("deprecated")),
            ..Default::default()
        };
        let deprecated_meta = deprecated.meta_view.clone();
        let merged = DiscussionRenderers::merge_with_slots(&slots, Some(deprecated));
        assert!(merged.meta_view.is_some());
        // Slot renderers are distinct Arc instances from deprecated stubs.
        assert!(!Arc::ptr_eq(
            merged.meta_view.as_ref().unwrap(),
            deprecated_meta.as_ref().unwrap()
        ));
    }

    #[test]
    fn merge_with_slots_falls_back_to_deprecated() {
        let slots = super::super::slots::DiscussionSlots::default();
        let deprecated = DiscussionRenderers {
            meta_view: Some(stub_meta("deprecated")),
            ..Default::default()
        };
        let merged = DiscussionRenderers::merge_with_slots(&slots, Some(deprecated));
        assert!(merged.meta_view.is_some());
    }
}
