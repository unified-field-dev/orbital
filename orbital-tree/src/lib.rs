//! Orbital Tree — Orbital placeholder stubs.

/// Label text for gap placeholder stubs.
pub fn placeholder_label(name: &str) -> String {
    format!("Todo: {name}")
}

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {}

mod tree;
mod types;

pub use tree::*;
pub use types::*;
