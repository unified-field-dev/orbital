use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::atom::{MotionAtom, SlideFrom};
use crate::tokens::MotionDuration;
use crate::PresenceMotion;

use super::demo::{demo_tile_styles, MotionDemoCell};

/// Longer than production presets so gallery Hide/Show toggles are easy to read.
const GALLERY_DEMO_DURATION: MotionDuration = MotionDuration::Slower;

const fn gallery_demo_motion(motion: PresenceMotion) -> PresenceMotion {
    motion.with_duration(GALLERY_DEMO_DURATION)
}

/// Gallery of CSS motion atom presets — the building blocks behind [`PresenceMotion`](crate::PresenceMotion).
///
/// Each atom maps to an `orbital-motion-*` CSS class family (fade, slide, scale, collapse, blur, rotate). Product code should start from a [`PresenceMotion`](crate::PresenceMotion) preset rather than wiring atoms directly.
///
/// | Atom | Typical use |
/// |------|-------------|
/// | Fade | Overlays, tooltips, inline alerts |
/// | Slide | Menus, popovers, anchored panels |
/// | Scale / fade+scale | Dialogs, toasts, compact pickers |
/// | Collapse | Expandable regions, accordion bodies |
/// | Blur / rotate | Orbital-specific emphasis transitions |
///
/// # Examples
///
/// ## Atoms gallery
/// <!-- preview -->
/// ```rust
/// use crate::preview::MotionAtomsGallery;
/// view! {
///     <div data-testid="motion-atoms-preview">
///         <MotionAtomsGallery />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Motion",
    preview_slug = "motion-atoms",
    preview_label = "Motion Atoms",
    preview_icon = icondata::AiAppstoreOutlined,
)]
#[component]
pub fn MotionAtomsGallery() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    view! {
        <div class="orbital-motion-demo-grid" data-testid="motion-atoms-gallery">
            <MotionDemoCell label="Fade" testid="motion-atom-fade" motion=PresenceMotion::fade() />
            <MotionDemoCell label="Fade + scale" testid="motion-atom-fade-scale" motion=gallery_demo_motion(PresenceMotion::fade_scale()) />
            <MotionDemoCell label="Scale" testid="motion-atom-scale" motion=gallery_demo_motion(PresenceMotion::new(MotionAtom::scale(), MotionAtom::scale())) />
            <MotionDemoCell label="Slide bottom" testid="motion-atom-slide-bottom" motion=gallery_demo_motion(PresenceMotion::slide(SlideFrom::Bottom)) />
            <MotionDemoCell label="Slide top" testid="motion-atom-slide-top" motion=gallery_demo_motion(PresenceMotion::slide(SlideFrom::Top)) />
            <MotionDemoCell label="Slide left" testid="motion-atom-slide-left" motion=gallery_demo_motion(PresenceMotion::slide(SlideFrom::Left)) />
            <MotionDemoCell label="Slide right" testid="motion-atom-slide-right" motion=gallery_demo_motion(PresenceMotion::slide(SlideFrom::Right)) />
            <MotionDemoCell label="Collapse" testid="motion-atom-collapse" motion=gallery_demo_motion(PresenceMotion::collapse()) />
            <MotionDemoCell label="Blur" testid="motion-atom-blur" motion=PresenceMotion::blur() />
            <MotionDemoCell label="Rotate" testid="motion-atom-rotate" motion=gallery_demo_motion(PresenceMotion::rotate()) />
        </div>
    }
}
