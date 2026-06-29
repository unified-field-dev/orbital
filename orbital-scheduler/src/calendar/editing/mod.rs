//! Re-export shared event editing for calendar consumers.

#[cfg(feature = "preview")]
mod preview;

pub use crate::shared::editing::*;

#[cfg(feature = "preview")]
pub use preview::*;
