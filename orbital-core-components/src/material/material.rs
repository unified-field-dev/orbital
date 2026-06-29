use leptos::prelude::*;
use orbital_base_components::BaseMaterial;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::{material_modifier_classes, material_styles};
pub use orbital_base_components::{MaterialCorners, MaterialElevation, MaterialVariant};

/// Orbital surface container with material variant, elevation, and corner treatment.
///
/// Surface treatment only (background, shadow, radius, stroke) — no layout or padding. Put spacing and flex layout on inner [`Flex`] or [`Card`] slots.
///
/// Pick [`MaterialElevation`] for depth: `Flat` for co-planar shell and bordered panels, `Resting` for cards at rest, `Raised` for emphasized callouts, `Floating` for popovers, and `Modal` for dialogs. Use [`MaterialVariant::Scrim`] as a scrim/backdrop surface (often paired with [`Backdrop`]).
///
/// # When to use
///
/// - **Solid** for most layout regions, cards, and content canvases - **Outlined** for flat bordered panels — pair with [`MaterialElevation::Flat`] - **Frost** / **Shell** for shell chrome and transient panels - **Scrim** for modal overlays — often paired with [`Backdrop`]
///
/// # Usage
///
/// 1. Pick a [`MaterialVariant`] for opacity and backdrop treatment. 2. Set [`MaterialElevation`] — `Resting` for inline panels, `Raised` for emphasized cards. 3. Set [`MaterialCorners`] when square edges are required. 4. Put layout and padding on inner [`Flex`] or card compound slots, not on Material.
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer `Resting` elevation for cards at rest and `Raised` for hero callouts * Compose Material inside [`Card`] rather than duplicating surface CSS * Use Frost + Floating for popovers over busy backgrounds * Wrap preview and test hooks in a native `div` with `data-testid`
///
/// ## Don'ts
///
/// * Do not add flex layout or default padding on Material — use [`Flex`] or [`Card`] * Do not use inline `style` for one-off sizing — use Turf classes and CSS vars * Do not stack many elevated surfaces at the same tier without visual hierarchy
///
/// # Examples
///
/// ## Solid at rest
/// Default opaque surface with resting elevation for inline panels and content regions.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="material-preview">
///         <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///             <div style="padding: 16px;">"Surface content"</div>
///         </Material>
///     </div>
/// }
/// ```
///
/// ## Solid raised
/// Raised elevation emphasizes a surface above its neighbors—common for hero cards and callouts.
/// <!-- preview -->
/// ```rust
/// use crate::{Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="material-raised-preview">
///         <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Raised>
///             <div style="padding: 16px;">"Raised surface"</div>
///         </Material>
///     </div>
/// }
/// ```
///
/// ## Variant matrix
/// Compare Solid, Frost, Shell, and Scrim treatments side by side over a textured background.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, FlexWrap, Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div
///         data-testid="material-variant-matrix"
///         style="padding: 16px; background: linear-gradient(135deg, #1A6F94 0%, #6B3FA0 100%);"
///     >
///         <Flex gap=FlexGap::Medium wrap=FlexWrap::Wrap>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///                 <div style="padding: 12px;">"Solid"</div>
///             </Material>
///             <Material variant=MaterialVariant::Frost elevation=MaterialElevation::Resting>
///                 <div style="padding: 12px;">"Frost"</div>
///             </Material>
///             <Material variant=MaterialVariant::Shell elevation=MaterialElevation::Resting>
///                 <div style="padding: 12px;">"Shell"</div>
///             </Material>
///             <Material variant=MaterialVariant::Scrim elevation=MaterialElevation::Resting>
///                 <div style="padding: 12px; color: white;">"Scrim"</div>
///             </Material>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Elevation matrix
/// Shadow tiers from Flat through Modal on Solid — each tier maps to a design-token shadow scale.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, FlexWrap, Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="material-elevation-matrix">
///         <Flex gap=FlexGap::Medium wrap=FlexWrap::Wrap>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Flat>
///                 <div style="padding: 12px;">"Flat"</div>
///             </Material>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///                 <div style="padding: 12px;">"Resting"</div>
///             </Material>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Raised>
///                 <div style="padding: 12px;">"Raised"</div>
///             </Material>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Floating>
///                 <div style="padding: 12px;">"Floating"</div>
///             </Material>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Modal>
///                 <div style="padding: 12px;">"Modal"</div>
///             </Material>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Frost floating
/// Frosted glass with floating elevation for transient panels and popovers over busy content.
/// <!-- preview -->
/// ```rust
/// use crate::{Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div
///         data-testid="material-frost-preview"
///         style="padding: 24px; background: linear-gradient(135deg, #1A6F94 0%, #6B3FA0 100%);"
///     >
///         <Material variant=MaterialVariant::Frost elevation=MaterialElevation::Floating>
///             <div style="padding: 16px;">"Frosted panel"</div>
///         </Material>
///     </div>
/// }
/// ```
///
/// ## CSS var override
/// Turf `class` sets `--orbital-material-width` for one-off sizing without a `style` prop.
/// <!-- preview -->
/// ```rust
/// use crate::{Material, MaterialElevation, MaterialVariant};
/// use turf::inline_style_sheet_values;
/// view! {
///     <div data-testid="material-var-override">
///         {
///             let (style_sheet, class_names) = inline_style_sheet_values! {
///                 .NarrowMaterial {
///                     --orbital-material-width: 240px;
///                 }
///             };
///             view! {
///                 <style>{style_sheet}</style>
///                 <Material
///                     variant=MaterialVariant::Solid
///                     elevation=MaterialElevation::Resting
///                     class=class_names.narrow_material
///                 >
///                     <div style="padding: 16px;">"Fixed width surface"</div>
///                 </Material>
///             }
///         }
///     </div>
/// }
/// ```
///
/// ## Theme: elevation scale
/// Custom elevation scale in the theme scope increases resting shadow depth on the sample surface.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::{Material, MaterialElevation, MaterialVariant};
/// use orbital_theme::{ElevationScale, OrbitalThemeProvider, Theme, ThemeMode, ThemeOverrides};
/// view! {
///     <div data-testid="material-theme-elevation">
///         <OrbitalThemeProvider theme=RwSignal::new(Theme::custom(
///             ThemeMode::Light,
///             ThemeOverrides {
///                 elevation: Some(ElevationScale { multiplier: 1.75 }),
///                 ..Default::default()
///             },
///         ))>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///                 <div style="padding: 16px;">"Scaled elevation"</div>
///             </Material>
///         </OrbitalThemeProvider>
///     </div>
/// }
/// ```
///
/// ## Composed in Card
/// Card delegates surface treatment to Material — layout gaps live on inner Flex, not the surface root.
/// <!-- preview -->
/// ```rust
/// use crate::Card;
/// view! {
///     <div data-testid="material-in-card" style="max-width: 360px;">
///         <Card>
///             <div style="padding: 16px;">"Card body on Material surface"</div>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Outlined flat
/// Stroke-bordered surface with no shadow — typical for outlined panels and list items.
/// <!-- preview -->
/// ```rust
/// use crate::{Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="material-outlined-preview">
///         <Material variant=MaterialVariant::Outlined elevation=MaterialElevation::Flat>
///             <div style="padding: 16px;">"Outlined surface"</div>
///         </Material>
///     </div>
/// }
/// ```
///
/// ## Square corners
/// Square corner treatment removes border radius on the surface root.
/// <!-- preview -->
/// ```rust
/// use crate::{Material, MaterialCorners, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="material-square-preview">
///         <Material
///             variant=MaterialVariant::Solid
///             elevation=MaterialElevation::Resting
///             corners=MaterialCorners::Square
///         >
///             <div style="padding: 16px;">"Square surface"</div>
///         </Material>
///     </div>
/// }
/// ```
///
/// ## Outlined vs solid
/// Side-by-side contrast between filled solid and stroke-outlined surfaces.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="material-appearance-matrix">
///         <Flex gap=FlexGap::Medium>
///             <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///                 <div style="padding: 12px;">"Solid"</div>
///             </Material>
///             <Material variant=MaterialVariant::Outlined elevation=MaterialElevation::Flat>
///                 <div style="padding: 12px;">"Outlined"</div>
///             </Material>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "material",
    preview_label = "Material",
    preview_icon = icondata::AiFileOutlined,
)]
#[component]
pub fn Material(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Orbital material treatment (texture / translucency).
    #[prop(default = MaterialVariant::Solid)]
    variant: MaterialVariant,
    /// Orbital elevation tier (shadow depth).
    #[prop(default = MaterialElevation::Resting)]
    elevation: MaterialElevation,
    /// Corner treatment on the surface root.
    #[prop(default = MaterialCorners::Rounded)]
    corners: MaterialCorners,
    /// Surface content rendered inside the elevated or flat Material shell.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-material", material_styles());
    let modifiers = material_modifier_classes(variant, elevation, corners);
    let root_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            modifiers.clone()
        } else {
            format!("{modifiers} {extra}")
        }
    });

    view! {
        <BaseMaterial class=root_class variant=variant elevation=elevation corners=corners>
            {children()}
        </BaseMaterial>
    }
}
