use leptos::prelude::*;
use orbital_macros::component_doc;

use super::styles::{card_bleed_styles, card_preview_slot_styles};

/// Custom hero region inside a card for non-image content.
///
/// Use for charts, skeletons, or arbitrary layout. For photo heroes use [`CardMedia`]. Do not nest [`CardMedia`] inside this slot.
///
/// # Examples
///
/// ## Placeholder hero
/// Custom hero block above card body content.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardPreview};
/// view! {
///     <div data-testid="card-preview-preview">
///         <div data-testid="card-preview-slot-default" style="max-width: 360px;">
///             <Card>
///                 <CardPreview>
///                     <div style="height: 120px; background: var(--orb-color-surface-subtle); display: flex; align-items: center; justify-content: center;">
///                         "Preview"
///                     </div>
///                 </CardPreview>
///                 <CardContent>"Card body"</CardContent>
///             </Card>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Inside card
/// Preview slot as the top region of a compound card.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardPreview};
/// view! {
///     <div data-testid="card-preview-slot-in-card" style="max-width: 360px;">
///         <Card>
///             <CardPreview>
///                 <div style="height: 100px; background: var(--orb-color-surface-subtle);" />
///             </CardPreview>
///             <CardContent>"Body below custom hero."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card-preview",
    preview_label = "Card Preview",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn CardPreview(
    /// Extra CSS class names merged onto the bleed hero slot. Use Turf classes to override `--orbital-card-content-padding` when the slot needs custom inset.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Custom hero content — charts, skeletons, placeholders, or arbitrary layout. For photo heroes use [`CardMedia`] instead; do not nest [`CardMedia`] inside this slot.
    children: Children,
) -> impl IntoView {
    let bleed_sheet = card_bleed_styles();
    let preview_sheet = card_preview_slot_styles();
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card-preview orbital-card-bleed".to_string()
        } else {
            format!("orbital-card-preview orbital-card-bleed {extra}")
        }
    });

    view! {
        <style>{bleed_sheet}</style>
        <style>{preview_sheet}</style>
        <div class=root_class>
            {children()}
        </div>
    }
}
