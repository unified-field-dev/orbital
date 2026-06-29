mod divider;
mod styles;

pub use divider::Divider;

#[cfg(feature = "preview")]
pub use divider::{DIVIDER_DESCRIPTION, DIVIDER_DOC, DIVIDER_PREVIEW_REGISTRATION, DIVIDER_PROPS};
