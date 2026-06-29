mod interaction_tag;
mod parts;
mod secondary_action_tag;

pub use interaction_tag::InteractionTag;
pub use parts::InteractionTagPrimary;
pub use secondary_action_tag::SecondaryActionTag;

#[cfg(feature = "preview")]
pub use interaction_tag::INTERACTIONTAG_PREVIEW_REGISTRATION;
