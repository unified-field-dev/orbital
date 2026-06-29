mod avatar_group;
mod styles;

pub use avatar_group::AvatarGroup;
pub use orbital_base_components::{AvatarGroupLayout, AvatarGroupSize};

#[cfg(feature = "preview")]
pub use avatar_group::{
    AVATARGROUP_DESCRIPTION, AVATARGROUP_DOC, AVATARGROUP_PREVIEW_REGISTRATION, AVATARGROUP_PROPS,
};
