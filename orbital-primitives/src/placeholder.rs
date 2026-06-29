//! Shared helpers for gap placeholder primitives.

/// Label text for [`orbital_base_components::DemoBox`] gap stubs.
pub fn placeholder_label(name: &str) -> String {
    format!("Todo: {name}")
}
