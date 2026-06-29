use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::{Popover, PopoverPosition, PopoverSize, PopoverTrigger, PopoverTriggerType};

use crate::navigation::popover::popover_styles;

use super::anatomy::spotlight_anatomy;
use super::slots_to_views::anatomy_from_slots;
use super::styles::spotlight_styles;
use super::types::{
    SpotlightActions, SpotlightBody, SpotlightFooter, SpotlightHeader, SpotlightMedia,
    SpotlightTrigger,
};

/// `SpotlightPopover` opens a structured coaching panel from a trigger click — title, body,
/// optional media, actions, and footer slots for onboarding. Use it when the user initiates
/// help from a control; use [`SpotlightTip`](super::tip::SpotlightTip) or
/// [`SpotlightTour`](super::tour::tour::SpotlightTour) when the app drives visibility and anchoring.
///
/// # Spotlight coaching
///
/// - **User opens help from a button** — `SpotlightPopover` (this component)
/// - **App drives visibility by element `id`** — [`SpotlightTip`](super::tip::SpotlightTip)
/// - **Multi-step walkthrough** — [`SpotlightTour`](super::tour::tour::SpotlightTour)
///
/// # Examples
///
/// ## Default trigger
/// Click the trigger to open a spotlight panel with header, body, and actions.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, ButtonAppearance, SpotlightActions, SpotlightBody, SpotlightHeader,
///     SpotlightPopover, SpotlightTrigger,
/// };
/// view! {
///     <div data-testid="spotlight-popover-preview">
///         <SpotlightPopover>
///             <SpotlightTrigger slot>
///                 <Button>"Show tip"</Button>
///             </SpotlightTrigger>
///             <SpotlightHeader slot>"Welcome"</SpotlightHeader>
///             <SpotlightBody slot>"Use this panel to discover new features."</SpotlightBody>
///             <SpotlightActions slot>
///                 <Button appearance=ButtonAppearance::Primary>"Got it"</Button>
///             </SpotlightActions>
///         </SpotlightPopover>
///     </div>
/// }
/// ```
///
/// ## Media and footer
/// Illustration plus a step indicator in the footer.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, SpotlightBody, SpotlightFooter, SpotlightHeader, SpotlightMedia,
///     SpotlightPopover, SpotlightTrigger,
/// };
/// view! {
///     <div data-testid="spotlight-popover-media">
///         <SpotlightPopover>
///             <SpotlightTrigger slot>
///                 <Button>"Tour"</Button>
///             </SpotlightTrigger>
///             <SpotlightHeader slot>"Step 1"</SpotlightHeader>
///             <SpotlightMedia slot>
///                 <div style="height: 80px; background: var(--orb-color-surface-subtle);">"Media"</div>
///             </SpotlightMedia>
///             <SpotlightBody slot>"Filter events from the toolbar."</SpotlightBody>
///             <SpotlightFooter slot>"1 of 3"</SpotlightFooter>
///         </SpotlightPopover>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Spotlight",
    preview_slug = "spotlight-popover",
    preview_label = "Spotlight Popover",
    preview_icon = icondata::AiReadOutlined,
)]
#[component]
pub fn SpotlightPopover(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] position: PopoverPosition,
    #[prop(optional, into)] size: Signal<PopoverSize>,
    spotlight_trigger: SpotlightTrigger,
    #[prop(optional)] spotlight_header: Option<SpotlightHeader>,
    #[prop(optional)] spotlight_body: Option<SpotlightBody>,
    #[prop(optional)] spotlight_media: Option<SpotlightMedia>,
    #[prop(optional)] spotlight_actions: Option<SpotlightActions>,
    #[prop(optional)] spotlight_footer: Option<SpotlightFooter>,
) -> impl IntoView {
    inject_style("orbital-spotlight", spotlight_styles());
    inject_style("orbital-popover", popover_styles());

    let surface_class = Signal::derive(move || {
        let mut parts = vec!["orbital-spotlight".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let anatomy = anatomy_from_slots(
        spotlight_header,
        spotlight_body,
        spotlight_media,
        spotlight_actions,
        spotlight_footer,
    );

    view! {
        <Popover
            class=surface_class
            trigger_type=PopoverTriggerType::Click
            position=position
            size=size
        >
            <PopoverTrigger slot>
                {(spotlight_trigger.children)()}
            </PopoverTrigger>
            {spotlight_anatomy(anatomy)}
        </Popover>
    }
}
