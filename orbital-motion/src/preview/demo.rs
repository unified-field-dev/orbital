//! Shared demo tile for motion catalog pages.

use leptos::prelude::*;

use crate::{OrbitalPresence, PresenceMotion};

use super::components::PreviewButton;

pub fn demo_tile_styles() -> &'static str {
    r#"
.orbital-motion-demo-cell {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--orb-space-block-sm);
    min-width: 140px;
}

.orbital-motion-demo-shape {
    width: 80px;
    height: 80px;
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-brand-bg);
}

.orbital-motion-demo-label {
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
}

.orbital-motion-demo-grid {
    display: flex;
    flex-wrap: wrap;
    gap: var(--orb-space-inline-lg);
}
"#
}

/// Single atom/preset demo cell with Show/Hide toggle.
#[component]
pub fn MotionDemoCell(
    /// Display label for the preset.
    label: &'static str,
    /// `data-testid` on the demo shape.
    testid: &'static str,
    /// Presence motion preset under test.
    motion: PresenceMotion,
) -> impl IntoView {
    let show = RwSignal::new(true);
    let motion = Signal::from(motion);

    view! {
        <div class="orbital-motion-demo-cell">
            <span class="orbital-motion-demo-label">{label}</span>
            <PreviewButton on_click=Callback::new(move |_| show.update(|v| *v = !*v))>
                {move || if show.get() { "Hide" } else { "Show" }}
            </PreviewButton>
            <OrbitalPresence show=show motion=motion>
                <div class="orbital-motion-demo-shape" data-testid=testid></div>
            </OrbitalPresence>
        </div>
    }
}
