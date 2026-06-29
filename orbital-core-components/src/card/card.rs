use leptos::prelude::*;

use crate::{Flex, FlexGap, Material, MaterialCorners, MaterialElevation, MaterialVariant};
use orbital_macros::component_doc;

use super::styles::card_layout_styles;

/// Groups related content on an Orbital [`Material`] surface.
///
/// # Usage
///
/// Stack parts in document order:
/// 1. [`CardHeader`] — title, description, and optional header action
/// 2. [`CardMedia`] or [`CardPreview`] — hero image or preview region
/// 3. [`CardContent`] — body copy
/// 4. [`CardButtonArea`] — optional whole-card primary action region
/// 5. [`CardFooter`] — independent secondary actions
///
/// Insert [`CardSectionBorder`] between stacked slots when a flush horizontal rule is needed inside the card. Keep spacing on the neighboring slots; the border itself is a zero-padding rule.
///
/// **CardButtonArea vs CardFooter:** Wrap the hero and body in [`CardButtonArea`] when the **entire card** navigates or opens details — one primary click target. Keep footer actions in [`CardFooter`] **outside** the button area so secondary commands (Reply, Share, Cancel) do not share the card-level click handler.
///
/// # Examples
///
/// ## Default (body only)
/// Basic card with body content on a solid surface at resting elevation.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent};
/// use turf::inline_style_sheet_values;
/// view! {
///     <div data-testid="card-preview" style="max-width: 360px;">
///         {
///             let (style_sheet, class_names) = inline_style_sheet_values! {
///                 .PaddedContent {
///                     --orbital-card-content-padding: 16px;
///                 }
///             };
///             view! {
///                 <style>{style_sheet}</style>
///                 <Card>
///                     <CardContent class=class_names.padded_content>
///                         "Card body content"
///                     </CardContent>
///                 </Card>
///             }
///         }
///     </div>
/// }
/// ```
///
/// ## Full compound
/// Header, image hero, body, and footer actions in one card.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, ButtonAppearance, Card, CardContent, CardFooter, CardHeader,
///     CardHeaderAction, CardHeaderDescription, CardMedia, Subtitle1,
/// };
/// view! {
///     <div data-testid="card-compound-preview" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Team workspace"</Subtitle1>
///                 <CardHeaderDescription slot>
///                     <span>"Shared files and tasks"</span>
///                 </CardHeaderDescription>
///                 <CardHeaderAction slot>
///                     <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
///                 </CardHeaderAction>
///             </CardHeader>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///                 height=140
///             />
///             <CardContent>"Card body copy."</CardContent>
///             <CardFooter>
///                 <Button>"Reply"</Button>
///                 <Button appearance=ButtonAppearance::Secondary>"Share"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## With header
/// Title header and body content regions.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardHeader, Subtitle1};
/// view! {
///     <div data-testid="card-with-header" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Overview"</Subtitle1>
///             </CardHeader>
///             <CardContent>"Metrics and summary content."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Media card
/// Photo hero with body copy and footer actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Card, CardContent, CardFooter, CardMedia};
/// view! {
///     <div data-testid="card-media-preview" style="max-width: 360px;">
///         <Card>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///                 height=140
///             />
///             <CardContent>"Lizards are a widespread group of squamate reptiles."</CardContent>
///             <CardFooter>
///                 <Button>"Share"</Button>
///                 <Button appearance=ButtonAppearance::Secondary>"Learn More"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Interactive media card
/// Primary click region wraps media and body; footer actions stay outside.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::{Button, Card, CardButtonArea, CardContent, CardFooter, CardMedia};
/// let clicked = RwSignal::new(false);
/// view! {
///     <div data-testid="card-button-area-preview" style="max-width: 360px;">
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
///                     {move || if clicked.get() { "Card clicked!" } else { "Tap the card body to open details." }}
///                 </CardContent>
///             </CardButtonArea>
///             <CardFooter>
///                 <Button on:click=|ev| { ev.stop_propagation(); }>"Share"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Custom hero
/// Arbitrary preview content above the card body.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardPreview};
/// view! {
///     <div data-testid="card-custom-hero-preview" style="max-width: 360px;">
///         <Card>
///             <CardPreview>
///                 <div style="height: 120px; background: var(--orb-color-surface-subtle); display: flex; align-items: center; justify-content: center;">
///                     "Custom hero"
///                 </div>
///             </CardPreview>
///             <CardContent>"Body below custom hero."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Raised elevation
/// Emphasized card surface using raised elevation.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, MaterialElevation};
/// use turf::inline_style_sheet_values;
/// view! {
///     <div data-testid="card-raised" style="max-width: 360px;">
///         {
///             let (style_sheet, class_names) = inline_style_sheet_values! {
///                 .PaddedContent {
///                     --orbital-card-content-padding: 16px;
///                 }
///             };
///             view! {
///                 <style>{style_sheet}</style>
///                 <Card elevation=MaterialElevation::Raised>
///                     <CardContent class=class_names.padded_content>
///                         "Hero card content"
///                     </CardContent>
///                 </Card>
///             }
///         }
///     </div>
/// }
/// ```
///
/// ## Outlined
/// Flat bordered card surface with no shadow.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, MaterialElevation, MaterialVariant};
/// use turf::inline_style_sheet_values;
/// view! {
///     <div data-testid="card-outlined" style="max-width: 360px;">
///         {
///             let (style_sheet, class_names) = inline_style_sheet_values! {
///                 .PaddedContent {
///                     --orbital-card-content-padding: 16px;
///                 }
///             };
///             view! {
///                 <style>{style_sheet}</style>
///                 <Card variant=MaterialVariant::Outlined elevation=MaterialElevation::Flat>
///                     <CardContent class=class_names.padded_content>
///                         "Outlined card content"
///                     </CardContent>
///                 </Card>
///             }
///         }
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card",
    preview_label = "Card",
    preview_icon = icondata::AiBorderOutlined,
)]
#[component]
pub fn Card(
    /// Optional CSS class merged onto the Material root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Surface material treatment forwarded to [`Material`].
    #[prop(default = MaterialVariant::Solid)]
    variant: MaterialVariant,
    /// Surface elevation forwarded to [`Material`].
    #[prop(default = MaterialElevation::Resting)]
    elevation: MaterialElevation,
    /// Corner treatment forwarded to [`Material`].
    #[prop(default = MaterialCorners::Rounded)]
    corners: MaterialCorners,
    /// Row gap between stacked slot regions. Set [`FlexGap::Size(0)`] for flush sections separated only by [`CardSectionBorder`](crate::CardSectionBorder).
    #[prop(default = FlexGap::Medium)]
    gap: FlexGap,
    /// Compound slot regions — [`CardHeader`], [`CardMedia`], [`CardPreview`], [`CardContent`], [`CardButtonArea`], and [`CardFooter`] in document order.
    children: Children,
) -> impl IntoView {
    let (layout_sheet, layout_classes) = card_layout_styles();
    let root_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card".to_string()
        } else {
            format!("orbital-card {extra}")
        }
    });

    view! {
        <style>{layout_sheet}</style>
        <Material variant=variant elevation=elevation corners=corners class=root_class>
            <Flex vertical=true full_width=true gap=gap class=layout_classes.inner.clone()>
                {children()}
            </Flex>
        </Material>
    }
}
