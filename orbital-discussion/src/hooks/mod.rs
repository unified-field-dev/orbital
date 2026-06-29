//! Discussion hooks for reading provider context.

use leptos::prelude::*;

pub use crate::context::{use_discussion, ComposerContext};
use crate::DiscussionFocus;

/// Read the current focus anchor signal.
pub fn use_discussion_focus() -> Signal<DiscussionFocus> {
    use_discussion().focus
}

/// Read composer draft state from the nearest [`DiscussionProvider`].
pub fn use_discussion_composer() -> ComposerContext {
    expect_context::<ComposerContext>()
}
