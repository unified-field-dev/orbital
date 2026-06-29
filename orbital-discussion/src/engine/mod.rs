//! Pure reply graph engine.

mod anchor;
mod composer;
mod file;
mod graph;
mod markdown;
mod markdown_edit;
mod scroll;
mod selectors;
mod streaming;
mod surface;
mod time;

pub use anchor::{
    ancestor_chain, is_reply_visible_in_tree, parse_reply_anchor_hash, resolve_focus_for_reply,
};
pub use composer::{
    add_attachment_draft, add_citation_draft, can_submit, remove_attachment_draft,
    remove_citation_draft, validate_attachment_metadata,
};
pub use file::{format_file_size, is_image_mime};
pub use graph::{
    pop_focus, push_focus, should_show_more_children, DiscussionReplyGraph,
    DEFAULT_MAX_VISIBLE_DEPTH,
};
pub use markdown::render_markdown;
pub use markdown_edit::{
    apply_markdown_wrap, insert_at_caret, insert_citation_ref, insert_markdown_image,
    insert_markdown_link, insert_markdown_prefix, wrap_markdown_selection,
};
#[cfg(feature = "hydrate")]
pub use scroll::{schedule_scroll_reply_into_view, scroll_reply_into_view};
pub use selectors::{select_reply_by_id, select_visible_branch};
pub use streaming::thread_has_streaming_reply;
pub use surface::resolve_reply_surface;
pub use time::format_relative_time;
