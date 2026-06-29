//! Chart capability flags.

use bitflags::bitflags;

bitflags! {
    /// Opt-in chart capabilities. No license checks — flags are product configuration only.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ChartFeatures: u32 {
        /// Zoom and pan interaction (CH-24).
        const ZOOM_PAN = 1 << 0;
        /// Enter/update transitions (default on; honor reduced motion).
        const ANIMATION = 1 << 1;
        /// Keyboard navigation between marks (CH-22). Keyboard zoom (CH-24) is deferred.
        const KEYBOARD_NAV = 1 << 2;
    }
}

/// Default capability flags for chart containers (`ANIMATION` + `KEYBOARD_NAV`).
pub const CHART_FEATURES_DEFAULT: ChartFeatures =
    ChartFeatures::ANIMATION.union(ChartFeatures::KEYBOARD_NAV);
