mod info_label;
mod styles;
mod types;

pub use info_label::InfoLabel;
pub use styles::info_label_styles;
pub use types::{InfoLabelInfo, InfoLabelSize, InfoLabelWeight};

#[cfg(feature = "preview")]
pub use info_label::INFOLABEL_PREVIEW_REGISTRATION;
