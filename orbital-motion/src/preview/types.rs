//! Shared preview types (avoids cyclic dependency on orbital-base-components).

/// Matches [`orbital_base_components::PreviewRenderMode`] without a cyclic crate dependency.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PreviewRenderMode {
    BareDefault,
}

/// Matches [`orbital_base_components::ComponentPropDoc`] for generated preview pages.
pub struct ComponentPropDoc {
    pub name: &'static str,
    pub type_name: &'static str,
    pub description: &'static str,
}
