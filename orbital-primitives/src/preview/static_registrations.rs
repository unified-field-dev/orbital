//! Static preview registrations for gap placeholders in this crate.

use crate::preview::PreviewRegistration;

#[cfg(feature = "preview")]
pub fn all() -> &'static [&'static PreviewRegistration] {
    &[]
}

#[cfg(not(feature = "preview"))]
pub fn all() -> &'static [&'static PreviewRegistration] {
    &[]
}
