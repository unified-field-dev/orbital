mod avatar;
mod styles;
mod types;

pub use avatar::Avatar;
pub use types::{AvatarColor, AvatarConfig, AvatarShape};

#[cfg(feature = "preview")]
pub use avatar::{AVATAR_DESCRIPTION, AVATAR_DOC, AVATAR_PREVIEW_REGISTRATION, AVATAR_PROPS};
