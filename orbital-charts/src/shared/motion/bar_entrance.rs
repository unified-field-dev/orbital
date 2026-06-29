//! Bar chart entrance motion preset.

use orbital_motion::{MotionCurve, PresenceMotion};

use crate::ChartMotion;

/// Default bar pop-in motion preset.
pub fn bar_entrance_motion(motion: &ChartMotion) -> PresenceMotion {
    motion
        .entrance
        .unwrap_or_else(|| PresenceMotion::fade_scale().with_curve(MotionCurve::DecelerateMax))
}
