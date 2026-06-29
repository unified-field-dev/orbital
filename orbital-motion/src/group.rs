use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_macros::component_doc;

use crate::callback::MotionElementCallback;
use crate::presence::{stagger_motion, OrbitalPresence, PresenceMotion};
use crate::tokens::MotionDuration;

/// Context for [`OrbitalPresenceGroup`] stagger choreography.
#[derive(Clone, Copy)]
pub struct MotionGroupContext {
    pub motion: Signal<PresenceMotion>,
    pub stagger: Signal<MotionDuration>,
}

/// Keyed list enter/exit with staggered enter delays between children.
///
/// Wrap a keyed [`ForEnumerate`] of [`OrbitalPresenceGroupItem`] children, pass a shared `motion` preset, and tune `stagger` (default [`MotionDuration::Normal`](crate::MotionDuration::Normal)) for spacing between items. Each item needs a stable `index` signal and its own `show` gate.
///
/// Stagger delays accumulate via [`MotionDuration::stagger_step_delay`](crate::MotionDuration::stagger_step_delay) — index 0 enters immediately, index 1 waits one step, index 2 waits two steps, and so on. Use `on_after_leave` on each item to remove rows from the DOM after the exit animation finishes.
///
/// # Examples
///
/// ## Staggered list
/// <!-- preview -->
/// ```rust
/// use crate::preview::OrbitalPresenceGroupDemo;
/// view! {
///     <OrbitalPresenceGroupDemo />
/// }
/// ```
#[cfg_attr(
    feature = "preview",
    component_doc(
        category = "Motion",
        preview_slug = "orbital-presence-group",
        preview_label = "Orbital Presence Group",
        preview_icon = icondata::AiOrderedListOutlined,
    )
)]
#[component]
pub fn OrbitalPresenceGroup(
    #[prop(into)] motion: Signal<PresenceMotion>,
    #[prop(optional, into)] stagger: Option<Signal<MotionDuration>>,
    children: Children,
) -> impl IntoView {
    let stagger = stagger.unwrap_or_else(|| Signal::from(MotionDuration::Normal));
    provide_context(MotionGroupContext { motion, stagger });
    children()
}

/// One staggered child inside [`OrbitalPresenceGroup`].
#[component]
pub fn OrbitalPresenceGroupItem(
    #[prop(into)] show: Signal<bool>,
    /// Zero-based index in the keyed list (used for stagger delay).
    #[prop(into)]
    index: Signal<usize>,
    /// Fired after the leave transition completes.
    #[prop(optional)]
    on_after_leave: Option<MotionElementCallback>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<MotionGroupContext>()
        .expect("OrbitalPresenceGroupItem must be used inside OrbitalPresenceGroup");
    let motion =
        Signal::derive(move || stagger_motion(ctx.motion.get(), ctx.stagger.get(), index.get()));
    let on_after_leave =
        on_after_leave.unwrap_or_else(|| MotionElementCallback::new(|_: web_sys::HtmlElement| {}));

    view! {
        <OrbitalPresence show=show motion=motion appear=true on_after_leave=on_after_leave>
            {children()}
        </OrbitalPresence>
    }
}
