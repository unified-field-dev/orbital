mod space;
mod types;

pub use space::Space;
pub use types::{SpaceConfig, SpaceGap};

#[cfg(feature = "preview")]
pub use space::{SPACE_DESCRIPTION, SPACE_DOC, SPACE_PREVIEW_REGISTRATION, SPACE_PROPS};
