use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::tokens::{MotionCurve, MotionDuration};
use crate::{OrbitalPresence, PresenceMotion};

use super::components::PreviewButton;
use super::demo::demo_tile_styles;

/// Interactive playground for tuning [`PresenceMotion`] presets before wiring them into [`OrbitalPresence`].
///
/// Pick a duration step and easing curve, optionally add enter delay, then toggle the demo shape to preview the combined transition. Values map to theme `--orb-motion-duration-*` and `--orb-motion-ease-*` variables — prefer these tokens over hard-coded milliseconds so timing stays consistent across surfaces.
///
/// # Examples
///
/// ## Motion settings
/// <!-- preview -->
/// ```rust
/// use crate::preview::MotionSettingsDemo;
/// view! {
///     <div data-testid="motion-settings-demo">
///         <MotionSettingsDemo />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Motion",
    preview_slug = "motion-settings",
    preview_label = "Motion Settings",
    preview_icon = icondata::AiSettingOutlined,
)]
#[component]
pub fn MotionSettingsDemo() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    let show = RwSignal::new(true);
    let duration = RwSignal::new(MotionDuration::Normal);
    let curve = RwSignal::new(MotionCurve::EasyEase);
    let with_delay = RwSignal::new(false);
    let motion = Signal::derive(move || {
        let mut m = PresenceMotion::fade_scale()
            .with_duration(duration.get())
            .with_curve(curve.get());
        if with_delay.get() {
            m = m.with_enter_delay(MotionDuration::Slow);
        }
        m
    });

    view! {
        <div data-testid="motion-settings-demo">
            <div style="display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 12px;">
                <span data-testid="motion-settings-duration-fast"><PreviewButton on_click=Callback::new(move |_| duration.set(MotionDuration::UltraFast))>"UltraFast"</PreviewButton></span>
                <span data-testid="motion-settings-duration-normal"><PreviewButton on_click=Callback::new(move |_| duration.set(MotionDuration::Normal))>"Normal"</PreviewButton></span>
                <span data-testid="motion-settings-duration-slow"><PreviewButton on_click=Callback::new(move |_| duration.set(MotionDuration::Slow))>"Slow"</PreviewButton></span>
            </div>
            <div style="display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 12px;">
                <PreviewButton on_click=Callback::new(move |_| curve.set(MotionCurve::Linear))>{MotionCurve::Linear.display_label()}</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| curve.set(MotionCurve::EasyEase))>{MotionCurve::EasyEase.display_label()}</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| curve.set(MotionCurve::DecelerateMid))>{MotionCurve::DecelerateMid.display_label()}</PreviewButton>
            </div>
            <div style="margin-bottom: 12px;">
                <PreviewButton on_click=Callback::new(move |_| with_delay.update(|v| *v = !*v))>
                    {move || if with_delay.get() { "Delay on" } else { "Delay off" }}
                </PreviewButton>
            </div>
            <p data-testid="motion-settings-readout">
                {move || {
                    let d = duration.get();
                    let c = curve.get();
                    format!(
                        "duration={} ({})  curve={} ({})",
                        d.scale_label(),
                        d.css_var_name(),
                        c.display_label(),
                        c.css_var_name()
                    )
                }}
            </p>
            <PreviewButton on_click=Callback::new(move |_| show.update(|v| *v = !*v))>"Toggle shape"</PreviewButton>
            <OrbitalPresence show=show motion=motion>
                <div class="orbital-motion-demo-shape" data-testid="motion-settings-shape"></div>
            </OrbitalPresence>
        </div>
    }
}
