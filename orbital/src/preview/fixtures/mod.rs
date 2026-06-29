//! Shared preview fixtures for `#[component_doc]` live examples.
//!
//! Feature-gated helpers that supply mock data, context providers, and styled
//! demo sections so component previews stay self-contained in the owning crate.

#[cfg(feature = "preview")]
mod infinite_scroll;
#[cfg(feature = "preview")]
mod navigation_sample;

#[cfg(feature = "preview")]
pub use infinite_scroll::mock_fetch_items;
#[cfg(feature = "preview")]
pub use navigation_sample::sample_navigation_view;
