use leptos::prelude::*;
use orbital_base_components::BaseBackdrop;
use orbital_macros::component_doc;
use orbital_motion::{resolve_presence_motion, MotionSlot, OrbitalPresence, PresenceMotion};
use orbital_style::inject_style;

use super::styles::backdrop_styles;
use super::types::BackdropConfig;

/// `Backdrop` renders a scrim over the viewport — full dim for loading or modal blocking,
/// or a spotlight cutout that highlights one element by `anchor_id`. Bind `config.open`;
/// pass `on_click` to dismiss on scrim tap. Prefer composed scrims inside [`Dialog`](crate::Dialog)
/// and [`Drawer`](crate::Drawer) unless you need a standalone loading overlay or spotlight highlight.
/// Pair spotlight cutouts with [`SpotlightTip`](crate::SpotlightTip) and [`SpotlightTour`](crate::SpotlightTour).
///
/// # When to use
///
/// - Modal dialogs and drawers that need a dimmed page behind the surface - Loading states that block interaction until work completes - Any overlay that should signal a temporary state change
///
/// # Usage
///
/// 1. Bind `config.open` to show or hide the scrim. 2. Pass `on_click` when clicking the scrim should dismiss the overlay. 3. Put a [`Spinner`](crate::Spinner) or other content in `children` for loading overlays.
///
/// # Best Practices
///
/// ## Do's
///
/// * Compose Backdrop inside teleported overlays (Dialog, Drawer) rather than duplicating scrim CSS * Provide an explicit dismiss path when `on_click` closes the overlay
///
/// ## Don'ts
///
/// * Do not stack multiple opaque backdrops — one scrim per overlay layer
///
/// # Examples
///
/// ## Dimmed scrim, click to close
/// Toggle the scrim open, then click the dimmed layer to dismiss — the default modal scrim pattern.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Backdrop, BackdropConfig, Button};
/// use leptos::prelude::*;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="backdrop-preview" style="position: relative; min-height: 160px;">
///         <Button on_click=Callback::new(move |_: leptos::ev::MouseEvent| open.set(true))>"Show backdrop"</Button>
///         <Backdrop
///             config=BackdropConfig::new(open.read_only())
///             on_click=Callback::new(move |_: leptos::ev::MouseEvent| open.set(false))
///         />
///     </div>
/// }
/// ```
///
/// ## Loading overlay
/// Center a spinner on the scrim while async work runs.
/// <!-- preview -->
/// ```rust
/// use crate::{Backdrop, BackdropConfig, Spinner};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="backdrop-loading" style="position: relative; min-height: 160px;">
///         <p style="padding: 16px;">"Loading data…"</p>
///         <Backdrop config=BackdropConfig::new(Signal::from(true)) class="orbital-backdrop--contained".to_string()>
///             <div
///                 data-testid="backdrop-spinner"
///                 style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; pointer-events: none;"
///             >
///                 <Spinner />
///             </div>
///         </Backdrop>
///     </div>
/// }
/// ```
///
/// ## Bounded demo frame
/// Scrim inside a positioned frame for catalog previews.
/// <!-- preview -->
/// ```rust
/// use crate::{Backdrop, BackdropConfig};
/// use leptos::prelude::*;
/// view! {
///     <div
///         data-testid="backdrop-framed"
///         style="position: relative; width: 320px; height: 180px; overflow: hidden; border: 1px solid var(--orb-color-border-subtle);"
///     >
///         <p style="padding: 16px;">"Content behind the scrim"</p>
///         <Backdrop
///             config=BackdropConfig::new(Signal::from(true))
///             class="backdrop-framed-scrim orbital-backdrop--contained".to_string()
///         />
///     </div>
/// }
/// ```
///
/// ## Spotlight cutout
/// Dim the viewport except a padded hole around a target element by `id`.
/// <!-- preview -->
/// ```rust
/// use crate::{Backdrop, BackdropConfig, Button};
/// use leptos::prelude::*;
/// use orbital_base_components::BackdropMode;
/// let open = RwSignal::new(false);
/// let anchor_id = RwSignal::new(Some("backdrop-spotlight-target".to_string()));
/// view! {
///     <div data-testid="backdrop-spotlight" style="position: relative; min-height: 200px;">
///         <Button on_click=Callback::new(move |_: leptos::ev::MouseEvent| open.set(true))>"Highlight"</Button>
///         <div
///             id="backdrop-spotlight-target"
///             data-testid="backdrop-spotlight-target"
///             style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);"
///         >
///             "Important control"
///         </div>
///         <Backdrop
///             config=BackdropConfig::new(open.read_only()).with_mode(BackdropMode::Spotlight {
///                 anchor_id: anchor_id.read_only().into(),
///                 padding: 8,
///             })
///             on_click=Callback::new(move |_: leptos::ev::MouseEvent| open.set(false))
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "backdrop",
    preview_label = "Backdrop",
    preview_icon = icondata::AiBgColorsOutlined,
)]
#[component]
pub fn Backdrop(
    /// Open state and behavior.
    config: BackdropConfig,
    /// Optional CSS class merged onto the scrim root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Called when the scrim receives a click — omit when clicks should pass through.
    #[prop(optional)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    /// Optional foreground content (e.g. a loading spinner).
    #[prop(optional)]
    children: Option<Children>,
    /// Optional enter/exit motion override.
    #[prop(optional)]
    motion: MotionSlot,
) -> impl IntoView {
    inject_style("orbital-backdrop", backdrop_styles());

    let open = config.open;
    let mode = config.mode;
    let motion = resolve_presence_motion(motion, PresenceMotion::fade());

    view! {
        <OrbitalPresence appear=true show=open motion=motion>
            <BaseBackdrop class=class mode=mode nostrip:on_click=on_click>
                {children.map(|c| c())}
            </BaseBackdrop>
        </OrbitalPresence>
    }
}
