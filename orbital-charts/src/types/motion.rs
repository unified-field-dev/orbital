//! Chart animation configuration.

use orbital_motion::{MotionDuration, PresenceMotion};

/// Chart-level animation configuration. Default: enabled, respects reduced motion.
#[derive(Clone, Debug)]
pub struct ChartMotion {
    /// When true, skip all enter/update animations.
    /// Also forced when [`orbital_motion::use_reduced_motion`] is true.
    pub skip_animation: bool,
    /// Stagger step between grouped items (bars, pie slices).
    pub stagger: MotionDuration,
    /// Override entrance preset per plot kind.
    pub entrance: Option<PresenceMotion>,
}

impl Default for ChartMotion {
    fn default() -> Self {
        Self {
            skip_animation: false,
            stagger: MotionDuration::Normal,
            entrance: None,
        }
    }
}

/// Resolve whether animations should be skipped.
///
/// Combines explicit `skip_animation` prop, [`ChartMotion::skip_animation`],
/// and the user's reduced-motion preference.
pub fn effective_skip_animation(
    skip_animation: Option<bool>,
    motion: Option<&ChartMotion>,
    prefers_reduced_motion: bool,
) -> bool {
    skip_animation.unwrap_or(false)
        || motion.map(|m| m.skip_animation).unwrap_or(false)
        || prefers_reduced_motion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_skip_animation_respects_all_sources() {
        assert!(!effective_skip_animation(None, None, false));
        assert!(effective_skip_animation(Some(true), None, false));
        assert!(effective_skip_animation(
            None,
            Some(&ChartMotion {
                skip_animation: true,
                ..Default::default()
            }),
            false
        ));
        assert!(effective_skip_animation(None, None, true));
    }
}
