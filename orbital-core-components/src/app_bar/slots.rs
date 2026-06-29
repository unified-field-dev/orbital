use leptos::prelude::*;
use orbital_base_components::{MaterialCorners, MaterialElevation, MaterialVariant};

/// Configures the [`Material`] surface treatment for [`AppBar`].
///
/// At [`MaterialElevation::Flat`], the material surface gets an outlined bottom edge for co-planar shell chrome.
#[slot]
pub struct AppBarMaterial {
    #[prop(default = MaterialVariant::Solid)]
    pub variant: MaterialVariant,
    #[prop(default = MaterialElevation::Flat)]
    pub elevation: MaterialElevation,
    #[prop(default = MaterialCorners::Square)]
    pub corners: MaterialCorners,
}

/// Host-composed leading region (branding, title, breadcrumbs).
#[slot]
pub struct AppBarLeading {
    pub children: Children,
}

/// Host-composed trailing region (utilities, account controls, theme toggle).
#[slot]
pub struct AppBarTrailing {
    pub children: Children,
}

/// Default material values when [`AppBarMaterial`] slot is omitted.
pub fn default_app_bar_material() -> (MaterialVariant, MaterialElevation, MaterialCorners) {
    (
        MaterialVariant::Solid,
        MaterialElevation::Flat,
        MaterialCorners::Square,
    )
}
