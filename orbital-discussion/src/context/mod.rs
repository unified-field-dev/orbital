//! Discussion context and provider.

mod composer;
mod provider;

pub use composer::{use_composer_reply_target, ComposerContext, ComposerReplyTarget};
pub use provider::{
    navigate_focus_back, navigate_focus_to, navigate_to_reply, toggle_collapse, use_discussion,
    use_discussion_locale, DiscussionContext, DiscussionProvider,
};
