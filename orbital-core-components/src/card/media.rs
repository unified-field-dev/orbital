use leptos::prelude::*;
use orbital_macros::component_doc;

use super::styles::{card_bleed_styles, card_media_styles};

/// Typed image hero region for photo or illustration cards.
///
/// Renders a full-width `<img>` with cover fit. Do not nest inside [`CardPreview`].
///
/// # Examples
///
/// ## Default height
/// Image hero at the default 140px height.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::CardMedia;
/// view! {
///     <div data-testid="card-media-preview">
///         <div data-testid="card-media-default">
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///             />
///         </div>
///     </div>
/// }
/// ```
///
/// ## Taller image
/// Custom hero height via the `height` prop.
/// <!-- preview -->
/// ```rust
/// use crate::CardMedia;
/// view! {
///     <div data-testid="card-media-tall">
///         <CardMedia
///             src="https://picsum.photos/seed/orbital-card/360/140"
///             alt="Sample card illustration"
///             height=200
///         />
///     </div>
/// }
/// ```
///
/// ## Inside card stack
/// Media hero as the top region of a compound card.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardMedia};
/// view! {
///     <div data-testid="card-media-in-card" style="max-width: 360px;">
///         <Card>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///             />
///             <CardContent>"Body below the image."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card-media",
    preview_label = "Card Media",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn CardMedia(
    /// Extra CSS class names merged onto the bleed image hero. Override height via Turf class setting `--orbital-card-media-height`.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Image URL displayed in the hero region.
    #[prop(into)]
    src: String,
    /// Accessible label for the image (`alt` attribute).
    #[prop(into)]
    alt: String,
    /// Fixed height in pixels. Default `140`. Override via `--orbital-card-media-height` Turf class.
    #[prop(optional, default = 140)]
    height: u32,
) -> impl IntoView {
    let bleed_sheet = card_bleed_styles();
    let media_sheet = card_media_styles();
    let media_height = move || format!("{height}px");
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card-media orbital-card-bleed".to_string()
        } else {
            format!("orbital-card-media orbital-card-bleed {extra}")
        }
    });

    view! {
        <style>{bleed_sheet}</style>
        <style>{media_sheet}</style>
        <img
            class=root_class
            src=src
            alt=alt
            style=move || format!("--orbital-card-media-height: {}", media_height())
        />
    }
}
