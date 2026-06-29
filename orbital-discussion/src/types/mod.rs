//! Discussion data types.

mod adapter;
mod agent;
mod appearance;
mod author;
mod citation;
mod composer;
mod error;
mod events;
mod features;
mod focus;
mod locale;
mod metadata;
mod part;
mod renderers;
mod reply;
mod slots;
mod sort;
mod status;
mod view_mode;

pub use adapter::DiscussionAdapter;
pub use agent::{
    DiscussionReasoningPart, DiscussionReasoningStatus, DiscussionStepPart, DiscussionStepStatus,
    DiscussionToolPart, DiscussionToolRunStatus, ToolApprovalDecision,
};
pub use appearance::{DiscussionAppearance, DiscussionReplySurface};
pub use author::{DiscussionAuthor, DiscussionAuthorRole};
pub use citation::DiscussionCitation;
pub use composer::{
    DiscussionAttachmentDraft, DiscussionAttachmentValidation, DiscussionComposerSubmit,
    DiscussionUploadedFile,
};
pub use error::{DiscussionError, DiscussionErrorCode, DiscussionErrorSource};
pub use events::DiscussionEvents;
pub use features::DiscussionFeatures;
pub use focus::DiscussionFocus;
pub use locale::{
    locale_signal, resolve_discussion_locale, DiscussionLocale, DiscussionRelativeTimeLocale,
};
pub use metadata::{DiscussionLabel, DiscussionMetadata};
pub use part::DiscussionPart;
pub use renderers::{
    CitationAffordanceView, CitationMenuExtrasView, CitationMenuView, CitationRenderContext,
    DiscussionCitationMenuItem, DiscussionMenuItem, DiscussionRenderers, ReplyFooterView,
    ReplyMenuExtrasView, ReplyMenuView, ReplyMetaView, ReplyPartView, ReplyRenderContext,
};
pub use reply::DiscussionReply;
pub use slots::{
    DiscussionCitationAffordance, DiscussionCitationMenu, DiscussionCitationMenuExtras,
    DiscussionComposerHint, DiscussionComposerSlot, DiscussionComposerTools, DiscussionEmptyView,
    DiscussionReplyFooterRegion, DiscussionReplyMenu, DiscussionReplyMenuExtras,
    DiscussionReplyMeta, DiscussionReplyPart, DiscussionSlots, DiscussionThreadToolbar,
};
pub use sort::DiscussionSort;
pub use status::DiscussionReplyStatus;
pub use view_mode::DiscussionViewMode;
