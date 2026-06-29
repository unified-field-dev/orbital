use leptos::prelude::*;
use orbital_base_components::BaseCardButtonArea;
use orbital_macros::component_doc;

use super::styles::card_button_area_styles;

/// Primary clickable region inside a card — wraps hero and body as one activation target.
///
/// Use for navigable or expandable cards. Keep footer actions outside this region so supplemental buttons do not share the primary click handler.
///
/// # Examples
///
/// ## Default click handler
/// Button region wraps body copy and fires a click handler.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::{Card, CardButtonArea, CardContent};
/// let clicked = RwSignal::new(false);
/// view! {
///     <div data-testid="card-button-area-preview">
///         <div data-testid="card-button-area-default" style="max-width: 360px;">
///         <Card>
///             <CardButtonArea on_click=Callback::new(move |_| {
///                 clicked.set(true);
///                 #[cfg(target_arch = "wasm32")]
///                 if let Some(window) = web_sys::window() {
///                     let _ = window.alert_with_message("Card clicked!");
///                 }
///             })>
///                 <CardContent>
///                     {move || if clicked.get() { "Card clicked!" } else { "Tap to open details." }}
///                 </CardContent>
///             </CardButtonArea>
///         </Card>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Media and content
/// Interactive card with image hero and body inside the button region.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::{Card, CardButtonArea, CardContent, CardMedia};
/// let clicked = RwSignal::new(false);
/// view! {
///     <div data-testid="card-button-area-media" style="max-width: 360px;">
///         <Card>
///             <CardButtonArea on_click=Callback::new(move |_| {
///                 clicked.set(true);
///                 #[cfg(target_arch = "wasm32")]
///                 if let Some(window) = web_sys::window() {
///                     let _ = window.alert_with_message("Card clicked!");
///                 }
///             })>
///                 <CardMedia
///                     src="https://picsum.photos/seed/orbital-card/360/140"
///                     alt="Sample card illustration"
///                     height=140
///                 />
///                 <CardContent>
///                     {move || if clicked.get() { "Card clicked!" } else { "Tap anywhere on the hero or body." }}
///                 </CardContent>
///             </CardButtonArea>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Hover and focus states
/// Button region shows hover and focus-visible treatment on keyboard navigation.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardButtonArea, CardContent};
/// view! {
///     <div data-testid="card-button-area-states" style="max-width: 360px;">
///         <Card>
///             <CardButtonArea>
///                 <CardContent>"Hover or focus this region."</CardContent>
///             </CardButtonArea>
///         </Card>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card-button-area",
    preview_label = "Card Button Area",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn CardButtonArea(
    /// Extra CSS class names merged onto the full-width action strip.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// When set, renders the area as a link (`<a>`) instead of a `<button>`.
    #[prop(optional, into)]
    href: MaybeProp<String>,
    /// When true, the action strip does not respond to clicks or navigation.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Click handler for button-mode areas (omit when `href` is set).
    #[prop(optional)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    /// Action label — typically a short verb phrase such as "View details".
    children: Children,
) -> impl IntoView {
    let style_sheet = card_button_area_styles();
    let root_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card-button-area".to_string()
        } else {
            format!("orbital-card-button-area {extra}")
        }
    });

    view! {
        <style>{style_sheet}</style>
        <BaseCardButtonArea
            class=root_class
            href=href
            disabled=disabled
            nostrip:on_click=on_click
        >
            {children()}
        </BaseCardButtonArea>
    }
}
