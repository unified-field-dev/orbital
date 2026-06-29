mod badge;
mod styles;
mod types;

pub use badge::Badge;
pub use types::{BadgeAppearance, BadgeColor, BadgeSize};

#[cfg(feature = "preview")]
pub use badge::BADGE_PREVIEW_REGISTRATION;
