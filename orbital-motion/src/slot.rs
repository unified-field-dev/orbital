//! Optional motion override slot for overlay and list components.

use leptos::prelude::*;

use crate::presence::PresenceMotion;

/// Resolve an optional motion override against a static default preset.
pub fn resolve_presence_motion(
    slot: MotionSlot,
    default: PresenceMotion,
) -> Signal<PresenceMotion> {
    slot.unwrap_or_else(|| Signal::from(default))
}

/// Resolve an optional motion override against a derived default preset.
pub fn resolve_presence_motion_derived(
    slot: MotionSlot,
    default: impl Fn() -> PresenceMotion + Send + Sync + 'static,
) -> Signal<PresenceMotion> {
    slot.unwrap_or_else(|| Signal::derive(default))
}

/// Optional motion override slot for overlay and list components.
///
/// When `Some`, consumers use the provided signal instead of the component default.
///
/// # Examples
///
/// ```ignore
/// // Overlay components accept `motion: MotionSlot` to override the default preset.
/// let motion: MotionSlot = Some(Signal::from(PresenceMotion::fade()));
/// ```
pub type MotionSlot = Option<Signal<PresenceMotion>>;
