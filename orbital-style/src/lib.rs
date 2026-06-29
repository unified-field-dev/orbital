//! Per-component style injection and SSR style collection for Orbital.

mod inject;
mod style_registry;

pub use inject::{inject_dynamic_style, inject_style};
pub use style_registry::{StyleRegistry, StyleRegistryContext};
