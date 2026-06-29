mod affix;
mod input;
mod styles;
mod types;

pub use affix::{InputPrefix, InputSuffix};
pub use input::Input;
#[cfg(feature = "preview")]
pub use input::INPUT_PREVIEW_REGISTRATION;
pub use styles::input_styles;
pub use types::{InputAppearance, InputBind, InputEvents};

pub use orbital_base_components::InputRef;
