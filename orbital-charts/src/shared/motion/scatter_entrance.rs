//! Scatter chart entrance motion preset.

use orbital_motion::{MotionCurve, PresenceMotion};

use crate::ChartMotion;

/// Default scatter point fade-scale entrance preset.
pub fn scatter_entrance_motion(motion: &ChartMotion) -> PresenceMotion {
    motion
        .entrance
        .unwrap_or_else(|| PresenceMotion::fade_scale().with_curve(MotionCurve::DecelerateMax))
}
