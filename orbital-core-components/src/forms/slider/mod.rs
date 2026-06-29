mod slider;
mod styles;
mod types;

#[cfg(feature = "preview")]
pub use slider::SLIDER_PREVIEW_REGISTRATION;
pub use slider::{Slider, SliderLabel};
pub use types::{SliderAppearance, SliderBind};

pub use orbital_base_components::{SliderRule, SliderRuleTrigger};
