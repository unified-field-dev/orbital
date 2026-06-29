//! Deferred visibility wrapper for coachmarks (popover / dialog content built by caller).

use super::coachmark_dismiss::is_session_dismissed;
use crate::components::motions::use_reduced_motion;
use crate::components::tokens::MotionPreset;
use leptos::prelude::*;
use std::time::Duration;

/// How long dismissal should be remembered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RememberMode {
    /// No persistence — shows again on every full navigation.
    None,
    /// `sessionStorage` flag for the current tab/session (`coachmark_dismiss` helpers).
    Session(&'static str),
}

/// Wraps arbitrary coachmark UI (typically a [`Popover`](Popover)) with deferred entrance.
#[component]
pub fn OrbitalCoachmark(
    /// Delay before the shell becomes visible (skipped when reduced motion is on).
    #[prop(default = 600_u32)]
    defer_ms: u32,
    remember: RememberMode,
    children: Children,
) -> impl IntoView {
    let prefers_reduced = use_reduced_motion();
    let ready = RwSignal::new(false);
    let skip = match remember {
        RememberMode::Session(k) => is_session_dismissed(k),
        RememberMode::None => false,
    };

    Effect::new(move |_| {
        if skip {
            return;
        }
        let ms = if prefers_reduced.get() {
            0u64
        } else {
            defer_ms as u64
        };
        if let Ok(handle) =
            set_timeout_with_handle(move || ready.set(true), Duration::from_millis(ms))
        {
            on_cleanup(move || handle.clear());
        }
    });

    let enter = MotionPreset::EnterPopover.transition_timing();

    view! {
        {if skip {
            ().into_any()
        } else {
            view! {
                <div
                    class="orbital-coachmark-root"
                    style=move || {
                        if ready.get() {
                            format!("opacity: 1; transition: opacity {enter};")
                        } else {
                            "opacity: 0; pointer-events: none;".to_string()
                        }
                    }
                >
                    {children()}
                </div>
            }
            .into_any()
        }}
    }
}
