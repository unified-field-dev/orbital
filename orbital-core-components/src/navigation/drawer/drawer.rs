use leptos::prelude::*;
use orbital_base_components::{DrawerModalType, DrawerPosition, DrawerSize, OpenBind};
use orbital_macros::component_doc;
use orbital_style::inject_style;

#[cfg(feature = "preview")]
use super::inline::InlineDrawer;
use super::overlay::OverlayDrawer;
#[cfg(feature = "preview")]
use super::parts::{DrawerBody, DrawerHeader, DrawerHeaderTitle};
use super::styles::drawer_styles;

/// Drawers slide content in from an edge — filters, settings, or auxiliary nav.
/// [`OverlayDrawer`] (alias [`Drawer`]) teleports a modal panel with backdrop;
/// [`InlineDrawer`] keeps the panel in your layout. Bind `open` with [`OpenBind`]; set
/// `position`, `size`, and whether backdrop click or Escape dismisses.
///
/// Prefer composed scrims inside [`Dialog`](crate::Dialog) and [`Backdrop`](crate::Backdrop)
/// unless you need a standalone loading overlay. Bind `open` with [`OpenBind`] at the app
/// root and pass signals down — share one `open` signal between trigger, backdrop dismiss,
/// and footer buttons rather than relying on implicit internal-only state.
///
/// # When to use
///
/// - Filters, settings, or detail panels triggered from a page action
/// - Modal overlay from an edge — `OverlayDrawer` / `Drawer`
/// - Persistent in-layout panel — `InlineDrawer`
///
/// # Usage
///
/// 1. Bind `open` with [`OpenBind`] (typically `RwSignal<bool>`).
/// 2. Choose `OverlayDrawer` for teleported modal panels or `InlineDrawer` for in-layout panels.
/// 3. Set `mask_closeable=false` when backdrop clicks should not dismiss.
/// 4. Compose [`DrawerHeader`] and [`DrawerBody`]; add [`DrawerHeaderTitleAction`] for header actions.
///
/// # Examples
///
/// ## Default drawer (inline)
/// Inline drawer stays in the preview card; an overlay trigger exercises teleported panels for migration smoke tests.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open_inline = RwSignal::new(true);
/// let open_overlay = RwSignal::new(false);
/// let overlay_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="drawer-preview" style="min-height: 120px;" node_ref=overlay_mount>
///         <InlineDrawer open=open_inline>
///             <DrawerHeader>
///                 <DrawerHeaderTitle>"Title"</DrawerHeaderTitle>
///             </DrawerHeader>
///             <DrawerBody>"Drawer body"</DrawerBody>
///         </InlineDrawer>
///         <Button on_click=Callback::new(move |_| open_overlay.set(true))>"Open drawer"</Button>
///         <OverlayDrawer open=open_overlay mount=Some(overlay_mount)>
///             <DrawerBody>
///                 <p data-testid="drawer-overlay-body">"Overlay drawer body"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Right overlay position
/// Modal panel that slides in from the right edge—the default drawer placement for detail panels, filters, and secondary workflows triggered by a button.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-right">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open right"</Button>
///         <OverlayDrawer open=open position=DrawerPosition::Right>
///             <DrawerBody>
///                 <p data-testid="drawer-right-content">"Right panel"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Left overlay position
/// Panel anchored to the left edge, suited for navigation rails or persistent secondary content that mirrors a sidebar pattern.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-left">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open left"</Button>
///         <OverlayDrawer open=open position=DrawerPosition::Left>
///             <DrawerBody>
///                 <p data-testid="drawer-left-content">"Left panel"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Top overlay position
/// Panel that drops from the top of the viewport—useful for banners, quick filters, or mobile-style sheets that occupy horizontal space.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-top">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open top"</Button>
///         <OverlayDrawer open=open position=DrawerPosition::Top>
///             <DrawerBody>
///                 <p data-testid="drawer-top-content">"Top panel"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Bottom overlay position
/// Panel that rises from the bottom edge, a common pattern for action sheets, pickers, and thumb-friendly mobile overlays.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-bottom">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open bottom"</Button>
///         <OverlayDrawer open=open position=DrawerPosition::Bottom>
///             <DrawerBody>
///                 <p data-testid="drawer-bottom-content">"Bottom panel"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Escape closes
/// Drawer that dismisses when the user presses Escape via `close_on_esc=true`. Enable this when keyboard users should be able to exit without clicking the backdrop.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-esc">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open drawer"</Button>
///         <OverlayDrawer open=open close_on_esc=true>
///             <DrawerBody>
///                 <p data-testid="drawer-esc-content">"Press Escape to close"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Large size
/// Wider panel preset using `size=DrawerSize::Large` for forms, multi-column detail, or content that needs more room than the default small width.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-size">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open large"</Button>
///         <OverlayDrawer open=open size=DrawerSize::Large>
///             <DrawerBody>
///                 <p data-testid="drawer-size-content">"Large drawer panel"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Backdrop not dismissible
/// Drawer that ignores backdrop clicks via `mask_closeable=false`. Use when dismissal must happen through an explicit close action.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="drawer-mask-not-closeable">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open drawer"</Button>
///         <OverlayDrawer open=open mask_closeable=false>
///             <DrawerBody>
///                 <p data-testid="drawer-mask-not-closeable-content">"Backdrop click does not close"</p>
///             </DrawerBody>
///         </OverlayDrawer>
///     </div>
/// }
/// ```
///
/// ## Header title action
/// Drawer header with a trailing action slot beside the title.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, DrawerHeaderTitleAction};
/// let open = RwSignal::new(true);
/// view! {
///     <div data-testid="drawer-title-action" style="min-height: 120px;">
///         <InlineDrawer open=open>
///             <DrawerHeader>
///                 <DrawerHeaderTitle>
///                     "Settings"
///                     <DrawerHeaderTitleAction slot>
///                         <Button appearance=ButtonAppearance::Subtle>"Reset"</Button>
///                     </DrawerHeaderTitleAction>
///                 </DrawerHeaderTitle>
///             </DrawerHeader>
///             <DrawerBody>"Drawer body"</DrawerBody>
///         </InlineDrawer>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "drawer",
    preview_label = "Drawer",
    preview_icon = icondata::AiMenuOutlined,
)]
#[component]
pub fn Drawer(
    /// Extra CSS class names merged onto the drawer panel surface.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Extra CSS class names merged onto the portal container wrapping the drawer.
    #[prop(optional, into)]
    container_class: MaybeProp<String>,
    /// Two-way open state binding — parent owns whether the drawer is visible.
    #[prop(into)]
    open: OpenBind,
    /// When true, clicking the backdrop mask closes the drawer.
    #[prop(default = true.into(), into)]
    mask_closeable: Signal<bool>,
    /// When true, pressing Escape closes the drawer.
    #[prop(optional)]
    close_on_esc: bool,
    /// Edge the panel slides from — `Right`, `Left`, `Top`, or `Bottom`.
    #[prop(default = DrawerPosition::Right.into(), into)]
    position: Signal<DrawerPosition>,
    /// Panel width or height preset — `Small`, `Medium`, `Large`, or `Full`.
    #[prop(default = DrawerSize::Small.into(), into)]
    size: Signal<DrawerSize>,
    /// Modal blocks page interaction; non-modal allows clicking content behind the panel.
    #[prop(default = DrawerModalType::Modal)]
    modal_type: DrawerModalType,
    /// Drawer body content — forms, detail panels, or navigation lists.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-drawer", drawer_styles());

    view! {
        <OverlayDrawer
            class=class
            container_class=container_class
            open=open
            mask_closeable=mask_closeable
            close_on_esc=close_on_esc
            position=position
            size=size
            modal_type=modal_type
        >
            {children()}
        </OverlayDrawer>
    }
}
