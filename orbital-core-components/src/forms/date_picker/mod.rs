mod component;
mod styles;
mod types;

pub use component::DatePicker;
#[cfg(feature = "preview")]
pub use component::DATEPICKER_PREVIEW_REGISTRATION;
pub use types::{DatePickerAppearance, DatePickerBind};

pub use orbital_base_components::{DatePickerRule, DatePickerRuleTrigger, DatetimeFormat};
