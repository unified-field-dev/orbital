mod styles;
mod switch;
mod types;

pub use switch::Switch;
#[cfg(feature = "preview")]
pub use switch::SWITCH_PREVIEW_REGISTRATION;
pub use types::{SwitchBind, SwitchLabel};

pub use orbital_base_components::{SwitchRule, SwitchRuleTrigger};
