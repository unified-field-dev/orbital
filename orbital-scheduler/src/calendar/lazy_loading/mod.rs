//! Calendar lazy loading — re-exports shared fetch + calendar preview (SC-11).

pub use crate::shared::lazy_loading::*;

#[cfg(feature = "preview")]
mod preview;

#[cfg(feature = "preview")]
pub use preview::*;

#[cfg(not(feature = "preview"))]
mod preview_stub;

#[cfg(not(feature = "preview"))]
#[allow(unused_imports)]
pub use preview_stub::*;
