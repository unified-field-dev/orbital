mod item;
mod skeleton;
mod styles;

pub use item::SkeletonItem;
pub use orbital_base_components::{SkeletonItemShape, SkeletonItemSize};
pub use skeleton::Skeleton;

#[cfg(feature = "preview")]
pub use skeleton::SKELETON_PREVIEW_REGISTRATION;
