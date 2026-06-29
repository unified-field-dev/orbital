mod styles;
mod textarea;
mod types;

pub use textarea::Textarea;
#[cfg(feature = "preview")]
pub use textarea::TEXTAREA_PREVIEW_REGISTRATION;
pub use types::{TextareaAppearance, TextareaBind, TextareaEvents};

pub use orbital_base_components::{
    TextareaRef, TextareaResize, TextareaRule, TextareaRuleTrigger, TextareaSize,
};
