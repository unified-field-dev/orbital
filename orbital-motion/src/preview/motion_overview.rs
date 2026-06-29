use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::tokens::MotionDuration;
use crate::{OrbitalPresence, PresenceMotion};

use super::components::PreviewButton;

pub(crate) fn overview_demo_styles() -> &'static str {
    r#"
.orbital-motion-overview {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xl);
    width: 100%;
    max-width: 720px;
}

.orbital-motion-overview__stack {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    width: 100%;
}

.orbital-motion-overview__layer {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: var(--orb-space-inline-md);
    align-items: start;
    padding: var(--orb-space-block-md) var(--orb-space-inline-md);
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-shell);
}

.orbital-motion-overview__layer-title {
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
    margin: 0;
}

.orbital-motion-overview__layer-body {
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
    color: var(--orb-color-text-secondary);
    margin: 0;
}

.orbital-motion-overview__try {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--orb-space-block-md);
}

.orbital-motion-overview__try-title {
    font-size: var(--orb-type-size-md);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
    margin: 0;
}

.orbital-motion-overview__stage {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 160px;
    min-height: 120px;
    padding: var(--orb-space-block-md) var(--orb-space-inline-md);
    border: 1px dashed var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-subtle);
}

.orbital-motion-overview__shape {
    width: 96px;
    height: 96px;
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-brand-bg);
}
"#
}

const DEMO_MOTION: PresenceMotion = PresenceMotion::fade().with_duration(MotionDuration::Slower);

/// Orbital Motion is the shared animation layer for enter, exit, and reorder transitions across dialogs, drawers, toasts, menus, and other UI that mounts and unmounts in place.
///
/// Motion is organized in layers you compose bottom-up: **tokens** define timing, **atoms** map to CSS keyframe classes, **presets** ([`PresenceMotion`](crate::PresenceMotion)) pair enter/exit atoms, [`OrbitalPresence`](crate::OrbitalPresence) applies them to conditional children, and [`OrbitalPresenceGroup`](crate::OrbitalPresenceGroup) staggers keyed lists. Product components (dialog, drawer, toast, and others) consume these primitives internally — you rarely wire atoms directly.
///
/// # When to use
///
/// - **Surfaces that open and close** — dialogs, drawers, popovers, toasts, and anchored panels - **Conditional content** — empty states, inline alerts, expandable regions, and step transitions - **Dynamic lists** — items added, removed, or filtered where enter/exit should feel connected
///
/// Reach for motion when visibility changes should feel intentional. Skip it for static layout, always-visible chrome, or high-frequency updates where animation would distract.
///
/// # Usage
///
/// 1. **Pick a preset** — start with [`PresenceMotion::fade`](crate::PresenceMotion::fade) or [`PresenceMotion::slide`](crate::PresenceMotion::slide) for most surfaces. 2. **Wrap conditional children** in [`OrbitalPresence`](crate::OrbitalPresence) with a `show` signal and `motion` preset. 3. **Tune timing** with [`PresenceMotion::with_duration`](crate::PresenceMotion::with_duration), [`PresenceMotion::with_curve`](crate::PresenceMotion::with_curve), or theme tokens from the [Motion Tokens](/motion-tokens) page. 4. **Stagger lists** with [`OrbitalPresenceGroup`](crate::OrbitalPresenceGroup) and keyed [`OrbitalPresenceGroupItem`](crate::OrbitalPresenceGroupItem) children when several elements enter or leave together. 5. **Respect accessibility** — keep `respect_reduced_motion` enabled (default) so `prefers-reduced-motion` users get instant transitions.
///
/// Browse the catalog sections under **Motion** for atoms, presence APIs, choreography, tokens, and reduced-motion behavior.
///
/// # Best Practices
///
/// ## Do's
///
/// * Start from a [`PresenceMotion`](crate::PresenceMotion) preset before customizing atoms or durations * Keep enter/exit durations short for tooltips and menus; use slower timing for hero or onboarding panels * Use [`OrbitalPresenceGroup`](crate::OrbitalPresenceGroup) with keyed children when animating lists — unkeyed lists remount and break transitions * Test with reduced motion enabled in your OS or browser devtools
///
/// ## Don'ts
///
/// * Do not animate every state change — reserve motion for meaningful visibility transitions * Do not bypass [`OrbitalPresence`](crate::OrbitalPresence) with ad-hoc CSS transitions on product components that already integrate motion * Do not stack multiple presence wrappers on the same element without a clear reason * Do not rely on motion alone to communicate critical information
///
/// # Examples
///
/// ## Motion stack
/// How tokens, atoms, presets, presence, and choreography fit together — with a live fade toggle.
/// <!-- preview -->
/// ```rust
/// use crate::preview::MotionOverview;
/// view! {
///     <MotionOverview />
/// }
/// ```
#[component_doc(
    category = "Motion",
    preview_slug = "motion",
    preview_label = "Overview",
    preview_icon = icondata::AiPlayCircleOutlined,
)]
#[component]
pub fn MotionOverview() -> impl IntoView {
    inject_style("orbital-motion-overview", overview_demo_styles());

    let show = RwSignal::new(true);
    let motion = Signal::from(DEMO_MOTION);

    view! {
        <div data-testid="motion-preview" class="orbital-motion-overview">
            <div class="orbital-motion-overview__stack" data-testid="motion-overview-stack">
                <div class="orbital-motion-overview__layer">
                    <p class="orbital-motion-overview__layer-title">"Tokens"</p>
                    <p class="orbital-motion-overview__layer-body">"Durations and easing curves from theme `--orb-motion-duration-*` and `--orb-motion-ease-*` variables."</p>
                </div>
                <div class="orbital-motion-overview__layer">
                    <p class="orbital-motion-overview__layer-title">"Atoms"</p>
                    <p class="orbital-motion-overview__layer-body">"CSS keyframe classes for fade, slide, scale, collapse, blur, and rotate."</p>
                </div>
                <div class="orbital-motion-overview__layer">
                    <p class="orbital-motion-overview__layer-title">"Presets"</p>
                    <p class="orbital-motion-overview__layer-body">"`PresenceMotion` bundles matching enter/exit atoms — the API most components use."</p>
                </div>
                <div class="orbital-motion-overview__layer">
                    <p class="orbital-motion-overview__layer-title">"Presence"</p>
                    <p class="orbital-motion-overview__layer-body">"`OrbitalPresence` mounts/unmounts children with transitions; used by dialogs, drawers, toasts, and overlays."</p>
                </div>
                <div class="orbital-motion-overview__layer">
                    <p class="orbital-motion-overview__layer-title">"Choreography"</p>
                    <p class="orbital-motion-overview__layer-body">"`OrbitalPresenceGroup` staggers keyed lists so items enter and leave in sequence."</p>
                </div>
            </div>
            <div class="orbital-motion-overview__try" data-testid="motion-overview-try">
                <p class="orbital-motion-overview__try-title">"Try presence"</p>
                <PreviewButton on_click=Callback::new(move |_| show.update(|v| *v = !*v))>
                    {move || if show.get() { "Hide surface" } else { "Show surface" }}
                </PreviewButton>
                <div class="orbital-motion-overview__stage">
                    <OrbitalPresence show=show motion=motion>
                        <div class="orbital-motion-overview__shape" data-testid="motion-overview-shape"></div>
                    </OrbitalPresence>
                </div>
            </div>
        </div>
    }
}
