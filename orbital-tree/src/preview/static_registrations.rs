//! Static preview registrations — Tree preview lives in orbital-core-components.

use crate::preview::PreviewRegistration;

#[cfg(feature = "preview")]
pub fn all() -> &'static [&'static PreviewRegistration] {
    &[]
}

#[cfg(not(feature = "preview"))]
pub fn all() -> &'static [&'static PreviewRegistration] {
    &[]
}
