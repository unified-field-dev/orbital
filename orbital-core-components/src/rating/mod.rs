mod injection;
mod rating;
mod rating_display;
mod rating_item;
mod styles;

pub use orbital_base_components::{RatingColor, RatingRule, RatingRuleTrigger, RatingSize};
pub use rating::Rating;
pub use rating_display::RatingDisplay;

#[cfg(feature = "preview")]
pub use rating::RATING_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use rating_display::RATINGDISPLAY_PREVIEW_REGISTRATION;
