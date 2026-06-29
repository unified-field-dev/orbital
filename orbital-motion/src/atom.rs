//! Motion atoms: one enter or exit leg of a presence transition.
//!
//! Each [`MotionAtom`] maps to a CSS keyframe family (`orbital-motion-fade`, `orbital-motion-scale`,
//! `orbital-motion-slide-*`, etc.). Pair atoms with [`crate::PresenceMotion`] for enter+exit presets,
//! or compose manually via [`PresenceMotion::new`](crate::PresenceMotion::new).

use crate::tokens::{MotionCurve, MotionDuration};

/// Slide direction for [`MotionAtom::slide`] and [`crate::PresenceMotion::slide`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SlideFrom {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

impl SlideFrom {
    pub const fn keyframes_suffix(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// One-shot motion descriptor (enter or exit leg of a presence transition).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotionAtom {
    pub keyframes: &'static str,
    pub duration: MotionDuration,
    pub curve: MotionCurve,
    pub delay: Option<MotionDuration>,
}

impl MotionAtom {
    pub const fn new(
        keyframes: &'static str,
        duration: MotionDuration,
        curve: MotionCurve,
    ) -> Self {
        Self {
            keyframes,
            duration,
            curve,
            delay: None,
        }
    }

    pub const fn with_delay(mut self, delay: MotionDuration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub const fn fade() -> Self {
        Self::new(
            "orbital-motion-fade",
            MotionDuration::Normal,
            MotionCurve::EasyEase,
        )
    }

    pub const fn scale() -> Self {
        Self::new(
            "orbital-motion-scale",
            MotionDuration::Normal,
            MotionCurve::EasyEase,
        )
    }

    pub const fn slide(from: SlideFrom) -> Self {
        Self::new(
            match from {
                SlideFrom::Bottom => "orbital-motion-slide-bottom",
                SlideFrom::Top => "orbital-motion-slide-top",
                SlideFrom::Left => "orbital-motion-slide-left",
                SlideFrom::Right => "orbital-motion-slide-right",
            },
            MotionDuration::Normal,
            MotionCurve::DecelerateMid,
        )
    }

    pub const fn collapse() -> Self {
        Self::new(
            "orbital-motion-collapse",
            MotionDuration::Normal,
            MotionCurve::EasyEaseMax,
        )
    }

    pub const fn blur() -> Self {
        Self::new(
            "orbital-motion-blur",
            MotionDuration::Normal,
            MotionCurve::EasyEase,
        )
    }

    pub const fn rotate() -> Self {
        Self::new(
            "orbital-motion-rotate",
            MotionDuration::Normal,
            MotionCurve::EasyEase,
        )
    }

    pub const fn with_duration(mut self, duration: MotionDuration) -> Self {
        self.duration = duration;
        self
    }

    pub const fn with_curve(mut self, curve: MotionCurve) -> Self {
        self.curve = curve;
        self
    }
}
