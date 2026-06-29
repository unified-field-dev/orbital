mod spinner;
mod styles;

pub use orbital_base_components::SpinnerSize;
pub use spinner::Spinner;

#[cfg(feature = "preview")]
pub use spinner::SPINNER_PREVIEW_REGISTRATION;
