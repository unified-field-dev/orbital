mod presence_badge;
mod styles;

pub use orbital_base_components::{PresenceBadgeSize, PresenceStatus};
pub use presence_badge::PresenceBadge;

#[cfg(feature = "preview")]
pub use presence_badge::{
    PRESENCEBADGE_DESCRIPTION, PRESENCEBADGE_DOC, PRESENCEBADGE_PREVIEW_REGISTRATION,
    PRESENCEBADGE_PROPS,
};
