use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::tokens::{MotionCurve, MotionDuration};

fn token_demo_styles() -> &'static str {
    r#"
.orbital-motion-tokens-table {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    width: 100%;
}

.orbital-motion-tokens-row {
    display: grid;
    grid-template-columns: 140px 1fr 1fr 120px;
    gap: var(--orb-space-inline-md);
    align-items: center;
    font-size: var(--orb-type-size-xs);
}

.orbital-motion-tokens-bar {
    height: 8px;
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-brand-bg);
    width: 0%;
}
"#
}

/// Typed duration and easing tokens for enter/exit transitions across Orbital surfaces.
///
/// Pick a [`MotionDuration`](crate::MotionDuration) step (50–500ms) and a [`MotionCurve`](crate::MotionCurve) for acceleration. Prefer these enums over hard-coded milliseconds in [`PresenceMotion::with_duration`](crate::PresenceMotion::with_duration) and [`PresenceMotion::with_curve`](crate::PresenceMotion::with_curve) so theme overrides propagate consistently through dialogs, drawers, toasts, and menus.
///
/// # Examples
///
/// ## Motion tokens
/// <!-- preview -->
/// ```rust
/// use crate::preview::MotionTokensReference;
/// view! {
///     <div data-testid="motion-tokens-preview">
///         <MotionTokensReference />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Motion",
    preview_slug = "motion-tokens",
    preview_label = "Motion Tokens",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn MotionTokensReference() -> impl IntoView {
    inject_style("orbital-motion-tokens", token_demo_styles());

    view! {
        <div>
            <h3>"Durations"</h3>
            <div class="orbital-motion-tokens-table" data-testid="motion-tokens-durations">
                {MotionDuration::ALL.iter().map(|&duration| {
                    let label = format!("{duration:?}");
                    let css_var = duration.css_var_name();
                    let ms = duration.ms();
                    view! {
                        <div class="orbital-motion-tokens-row">
                            <span>{label}</span>
                            <code>{css_var}</code>
                            <code>{ms}</code>
                            <div
                                class="orbital-motion-tokens-bar"
                                style=format!("transition: width {ms} ease; width: 100%;")
                            ></div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <h3>"Curves"</h3>
            <div class="orbital-motion-tokens-table" data-testid="motion-tokens-curves">
                {[
                    MotionCurve::Linear,
                    MotionCurve::AccelerateMax,
                    MotionCurve::AccelerateMid,
                    MotionCurve::AccelerateMin,
                    MotionCurve::DecelerateMax,
                    MotionCurve::DecelerateMid,
                    MotionCurve::DecelerateMin,
                    MotionCurve::EasyEase,
                    MotionCurve::EasyEaseMax,
                ].into_iter().map(|curve| {
                    let label = curve.display_label();
                    view! {
                        <div class="orbital-motion-tokens-row">
                            <span>{label}</span>
                            <code>{curve.css_var_name()}</code>
                            <code>{curve.cubic_bezier()}</code>
                            <span></span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
