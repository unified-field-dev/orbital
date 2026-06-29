use leptos::prelude::*;
use orbital_base_components::{MaterialCorners, MaterialElevation, MaterialVariant};

/// Configures the [`Material`](crate::Material) surface for [`Navigation`](crate::Navigation).
///
/// At [`MaterialElevation::Flat`], the material surface gets an outlined trailing edge — flat elevation with a stroke on the rail boundary.
#[slot]
pub struct NavigationMaterial {
    #[prop(default = MaterialVariant::Solid)]
    pub variant: MaterialVariant,
    #[prop(default = MaterialElevation::Flat)]
    pub elevation: MaterialElevation,
    #[prop(default = MaterialCorners::Square)]
    pub corners: MaterialCorners,
}

/// Optional header region above the navigation body.
#[slot]
pub struct NavigationHeader {
    pub children: Children,
}

/// Scrollable navigation content region.
#[slot]
pub struct NavigationBody {
    pub children: Children,
}

/// Optional footer region below the navigation body.
#[slot]
pub struct NavigationFooter {
    pub children: Children,
}

pub fn default_navigation_material() -> (MaterialVariant, MaterialElevation, MaterialCorners) {
    (
        MaterialVariant::Solid,
        MaterialElevation::Flat,
        MaterialCorners::Square,
    )
}
