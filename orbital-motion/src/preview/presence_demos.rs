//! Live demos for [`crate::OrbitalPresence`] catalog examples.

use leptos::prelude::*;
use orbital_style::inject_style;

use crate::{OrbitalPresence, PresenceMotion};

use super::components::PreviewButton;
use super::demo::demo_tile_styles;

#[component]
pub fn OrbitalPresenceBasicFadeDemo() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    let show = RwSignal::new(true);
    let motion = Signal::from(PresenceMotion::fade());

    view! {
        <div data-testid="orbital-presence-preview" class="orbital-motion-demo-cell">
            <PreviewButton on_click=Callback::new(move |_| show.update(|v| *v = !*v))>
                {move || if show.get() { "Hide" } else { "Show" }}
            </PreviewButton>
            <OrbitalPresence show=show motion=motion>
                <div class="orbital-motion-demo-shape" data-testid="orbital-presence-shape"></div>
            </OrbitalPresence>
        </div>
    }
}

#[component]
pub fn OrbitalPresenceAppearDemo() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    view! {
        <div data-testid="orbital-presence-appear" class="orbital-motion-demo-cell">
            <OrbitalPresence
                appear=true
                show=Signal::from(true)
                motion=Signal::from(PresenceMotion::fade_scale())
            >
                <div class="orbital-motion-demo-shape" data-testid="orbital-presence-appear-shape"></div>
            </OrbitalPresence>
        </div>
    }
}
