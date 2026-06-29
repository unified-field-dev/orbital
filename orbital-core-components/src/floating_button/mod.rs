mod floating_button;
mod styles;
mod types;

pub use floating_button::FloatingButton;
pub use types::{
    FloatingButtonColor, FloatingButtonConfig, FloatingButtonSize, FloatingButtonVariant,
};

#[cfg(feature = "preview")]
pub use floating_button::{
    FloatingButtonPreview, FLOATINGBUTTON_BEST_PRACTICES, FLOATINGBUTTON_DESCRIPTION,
    FLOATINGBUTTON_DOC, FLOATINGBUTTON_PREVIEW_REGISTRATION, FLOATINGBUTTON_PROPS,
};

pub use styles::floating_button_styles;
