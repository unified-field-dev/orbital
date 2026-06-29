mod compound_button;
mod styles;

pub use compound_button::CompoundButton;
pub use orbital_base_components::CompoundButtonIconPosition;

#[cfg(feature = "preview")]
pub use compound_button::COMPOUNDBUTTON_PREVIEW_REGISTRATION;
