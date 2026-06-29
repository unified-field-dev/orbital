mod back_to_top;
mod styles;
mod types;

pub use back_to_top::BackToTop;
pub use types::BackToTopLabel;

#[cfg(feature = "preview")]
pub use back_to_top::{
    BackToTopPreview, BACKTOTOP_BEST_PRACTICES, BACKTOTOP_DESCRIPTION, BACKTOTOP_DOC,
    BACKTOTOP_PREVIEW_REGISTRATION, BACKTOTOP_PROPS,
};
