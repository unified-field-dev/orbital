use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::{OrbitalPresence, PresenceMotion};

use super::components::PreviewButton;
use super::demo::demo_tile_styles;

/// Demonstrates how [`OrbitalPresence`](crate::OrbitalPresence) honors the OS `prefers-reduced-motion` setting.
///
/// By default (`respect_reduced_motion=true`) transitions collapse to instant show/hide when the user has reduced motion enabled. Toggle the side-by-side demos to compare default behavior vs forced animation.
///
/// **Testing:** enable "Reduce motion" in your OS accessibility settings (Windows: Settings → Accessibility → Visual effects; macOS: System Settings → Accessibility → Display → Reduce motion), then reload the preview. Marketing surfaces like [`HeroSection`](../../orbital/src/components/patterns/marketing/hero_section.rs) use the same hook for parallax.
///
/// # Examples
///
/// ## Reduced motion
/// <!-- preview -->
/// ```rust
/// use crate::preview::MotionReducedMotionDemo;
/// view! {
///     <div data-testid="motion-reduced-motion-preview">
///         <MotionReducedMotionDemo />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Motion",
    preview_slug = "motion-reduced-motion",
    preview_label = "Reduced Motion",
    preview_icon = icondata::AiEyeInvisibleOutlined,
)]
#[component]
pub fn MotionReducedMotionDemo() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    let show = RwSignal::new(true);
    let motion = Signal::from(PresenceMotion::fade_scale());

    view! {
        <div data-testid="motion-reduced-motion-preview" style="display: flex; gap: 24px;">
            <div>
                <p>"Respects prefers-reduced-motion (default)"</p>
                <PreviewButton on_click=Callback::new(move |_| show.update(|v| *v = !*v))>"Toggle"</PreviewButton>
                <OrbitalPresence show=show motion=motion respect_reduced_motion=true>
                    <div class="orbital-motion-demo-shape" data-testid="motion-reduced-respect"></div>
                </OrbitalPresence>
            </div>
            <div>
                <p>"Ignores reduced motion"</p>
                <OrbitalPresence show=show motion=motion respect_reduced_motion=false>
                    <div class="orbital-motion-demo-shape" data-testid="motion-reduced-ignore"></div>
                </OrbitalPresence>
            </div>
        </div>
    }
}
