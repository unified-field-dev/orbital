#[cfg(feature = "hydrate")]
mod download_impl;

#[cfg(feature = "hydrate")]
pub use download_impl::{download_bytes, print_html, write_clipboard_text};

#[cfg(not(feature = "hydrate"))]
pub fn download_bytes(_filename: &str, _bytes: &[u8], _mime: &str) {}

#[cfg(not(feature = "hydrate"))]
pub fn print_html(_html: &str) {}

#[cfg(not(feature = "hydrate"))]
pub fn write_clipboard_text(_text: &str) {}
