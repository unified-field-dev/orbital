//! Topic guides for Orbital Date Pickers — timezone, validation, customization, and polish.

mod accessibility;
mod custom_components;
mod custom_field;
mod custom_layout;
mod custom_open;
mod overview;
mod playground;
mod shortcuts;
mod timezone;
mod validation;

pub use accessibility::DatePickersAccessibilityGuide;
pub use custom_components::{weekend_day, DatePickersCustomComponentsGuide};
pub use custom_field::{CustomRangeSummaryField, DatePickersCustomFieldGuide};
pub use custom_layout::DatePickersCustomLayoutGuide;
pub use custom_open::DatePickersCustomOpenGuide;
pub use overview::DatePickersOverviewGuide;
pub use playground::DatePickersPlaygroundGuide;
pub use shortcuts::DatePickersShortcutsGuide;
pub use timezone::DatePickersTimezoneGuide;
pub use validation::DatePickersValidationGuide;

#[cfg(feature = "preview")]
pub use accessibility::DATEPICKERSACCESSIBILITYGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use custom_components::DATEPICKERSCUSTOMCOMPONENTSGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use custom_field::DATEPICKERSCUSTOMFIELDGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use custom_layout::DATEPICKERSCUSTOMLAYOUTGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use custom_open::DATEPICKERSCUSTOMOPENGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use overview::DATEPICKERSOVERVIEWGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use playground::DATEPICKERSPLAYGROUNDGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use shortcuts::DATEPICKERSSHORTCUTSGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use timezone::DATEPICKERSTIMEZONEGUIDE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use validation::DATEPICKERSVALIDATIONGUIDE_PREVIEW_REGISTRATION;
