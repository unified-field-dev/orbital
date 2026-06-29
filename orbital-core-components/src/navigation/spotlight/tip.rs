use leptos::prelude::*;
use orbital_base_components::{
    build_anchor_arrow, AnchoredSurface, Handler, OpenBind, OverlayAppearance, OverlayDismiss,
    Placement,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::navigation::popover::popover_styles;
use crate::overlay::{overlay_surface_class, FloatingPanel};
use crate::{Backdrop, PopoverAppearance, PopoverPosition, PopoverSize};

use super::anatomy::spotlight_anatomy;
use super::arrow::spotlight_arrow;
use super::backdrop_map::spotlight_backdrop_config;
use super::slots_to_views::anatomy_from_slots;
use super::styles::spotlight_styles;
use super::types::{
    SpotlightActions, SpotlightBackdrop, SpotlightBody, SpotlightFooter, SpotlightHeader,
    SpotlightMedia,
};

/// `SpotlightTip` shows a coaching panel anchored to the element whose `id` matches `anchor_id`.
/// Bind `open` to control visibility from your app — no trigger slot. Assign stable DOM `id`
/// values before anchoring — tips match on `anchor_id`, not component refs. Set `backdrop` to
/// [`SpotlightBackdrop::Spotlight`](crate::SpotlightBackdrop) to dim the page and cut out
/// the target; use [`SpotlightPopover`](super::popover::SpotlightPopover) when the user opens
/// help from a button instead. Show one programmatic tip at a time; re-anchor when scroll or resize moves targets.
///
/// # Spotlight coaching
///
/// - **Single programmatic step** — `SpotlightTip` (this component)
/// - **Multi-step walkthrough** — [`SpotlightTour`](super::tour::tour::SpotlightTour)
/// - **Spotlight cutout dimming** — [`Backdrop`](crate::Backdrop) Spotlight mode
///
/// # Examples
///
/// ## Controlled anchor
/// A start callback sets the anchor `id` and opens the tip beside the target.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, ButtonAppearance, SpotlightActions, SpotlightBody, SpotlightHeader, SpotlightTip,
/// };
/// use leptos::prelude::*;
/// use orbital_base_components::OpenBind;
/// let open = RwSignal::new(false);
/// let anchor_id = RwSignal::new(None::<String>);
/// let start = move |_| {
///     anchor_id.set(Some("spotlight-target-1".into()));
///     open.set(true);
/// };
/// view! {
///     <div data-testid="spotlight-tip-controlled">
///         <Button on:click=start>"Start"</Button>
///         <div id="spotlight-target-1" data-testid="spotlight-target-1" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             "Filter toolbar"
///         </div>
///         <SpotlightTip open=open anchor_id=anchor_id>
///             <SpotlightHeader slot>"Filters"</SpotlightHeader>
///             <SpotlightBody slot>"Filter events from the toolbar."</SpotlightBody>
///             <SpotlightActions slot>
///                 <Button appearance=ButtonAppearance::Primary on:click=move |_| open.set(false)>"Got it"</Button>
///             </SpotlightActions>
///         </SpotlightTip>
///     </div>
/// }
/// ```
///
/// ## Spotlight backdrop
/// Dim the page except a padded cutout around the active anchor.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, ButtonAppearance, SpotlightActions, SpotlightBackdrop, SpotlightBody,
///     SpotlightHeader, SpotlightTip,
/// };
/// use leptos::prelude::*;
/// let open = RwSignal::new(false);
/// let anchor_id = RwSignal::new(None::<String>);
/// let start = move |_| {
///     anchor_id.set(Some("spotlight-target-spotlight".into()));
///     open.set(true);
/// };
/// view! {
///     <div data-testid="spotlight-tip-spotlight">
///         <Button on:click=start>"Highlight feature"</Button>
///         <div id="spotlight-target-spotlight" data-testid="spotlight-target-spotlight" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             "Important control"
///         </div>
///         <SpotlightTip
///             open=open
///             anchor_id=anchor_id
///             backdrop=SpotlightBackdrop::Spotlight { padding: 8, dismiss_on_click: false }
///         >
///             <SpotlightHeader slot>"Spotlight"</SpotlightHeader>
///             <SpotlightBody slot>"The rest of the page is dimmed."</SpotlightBody>
///             <SpotlightActions slot>
///                 <Button appearance=ButtonAppearance::Primary on:click=move |_| open.set(false)>"Done"</Button>
///             </SpotlightActions>
///         </SpotlightTip>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Spotlight",
    preview_slug = "spotlight-tip",
    preview_label = "Spotlight Tip",
    preview_icon = icondata::AiBulbOutlined,
)]
#[component]
pub fn SpotlightTip(
    #[prop(into)] open: OpenBind,
    #[prop(into)] anchor_id: Signal<Option<String>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] position: PopoverPosition,
    #[prop(optional, into)] appearance: MaybeProp<PopoverAppearance>,
    #[prop(optional, into)] size: Signal<PopoverSize>,
    #[prop(optional)] on_open_change: Option<Handler<bool>>,
    #[prop(optional, default = SpotlightBackdrop::None)] backdrop: SpotlightBackdrop,
    #[prop(optional)] spotlight_header: Option<SpotlightHeader>,
    #[prop(optional)] spotlight_body: Option<SpotlightBody>,
    #[prop(optional)] spotlight_media: Option<SpotlightMedia>,
    #[prop(optional)] spotlight_actions: Option<SpotlightActions>,
    #[prop(optional)] spotlight_footer: Option<SpotlightFooter>,
) -> impl IntoView {
    inject_style("orbital-spotlight", spotlight_styles());
    inject_style("orbital-popover", popover_styles());

    let show = open.signal();
    let open_bind = open;

    if let Some(handler) = on_open_change {
        Effect::new(move |_| {
            handler.run(show.get());
        });
    }

    provide_context(OverlayDismiss {
        close: Callback::new(move |_| open_bind.set(false)),
    });

    let placement = Signal::from(Placement::from(position));
    let overlay_appearance = appearance
        .get()
        .map(|a: PopoverAppearance| a.into())
        .unwrap_or(OverlayAppearance::Default);
    let panel_size = StoredValue::new(size.get_untracked());
    let surface_class = Signal::derive(move || {
        let mut parts = vec![overlay_surface_class(
            "orbital-popover-surface",
            overlay_appearance,
            Some(panel_size.get_value().as_str()),
        )];
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

    let arrow_ref = NodeRef::<leptos::html::Div>::new();
    let anchor_arrow = build_anchor_arrow(arrow_ref);
    let backdrop_layer = spotlight_backdrop_config(open_bind, backdrop, anchor_id);

    view! {
        {backdrop_layer.map(|(config, on_click)| match on_click {
            Some(handler) => view! {
                <Backdrop
                    class="orbital-spotlight-portal__backdrop".to_string()
                    config=config
                    on_click=handler
                />
            }.into_any(),
            None => view! {
                <Backdrop
                    class="orbital-spotlight-portal__backdrop".to_string()
                    config=config
                />
            }.into_any(),
        })}
        <AnchoredSurface
            show=show
            anchor_id=anchor_id
            placement=placement
            arrow=anchor_arrow
        >
            <div class="orbital-popover-shell orbital-spotlight">
                {spotlight_arrow(arrow_ref)}
                <FloatingPanel
                    class=surface_class
                    body_class="orbital-popover-body"
                >
                    {spotlight_anatomy(anatomy)}
                </FloatingPanel>
            </div>
        </AnchoredSurface>
    }
}
