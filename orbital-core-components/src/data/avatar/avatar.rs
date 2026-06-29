use leptos::prelude::*;
use orbital_base_components::BaseAvatar;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::avatar_styles;
use super::types::AvatarConfig;

/// Shows a person or entity as a photo, generated initials, or name-derived palette background.
///
/// Set `config.name` for accessible labeling and auto-initials, or override with `config.initials` or `config.src`. Tune `config.size` and `config.shape` to match surrounding typography. Compose with [`PresenceBadge`](crate::PresenceBadge) or [`CounterBadge`](crate::CounterBadge) for status overlays, and [`AvatarGroup`](crate::AvatarGroup) for stacked participant lists.
///
/// # When to use
///
/// - User lists, comment threads, and profile headers - Entity representation when a photo is unavailable - Compact identity markers beside names in tables and navigation
///
/// # Usage
///
/// 1. Set `config.name` for accessible labels and auto-generated initials. 2. Set `config.src` when a profile photo is available. 3. Override with `config.initials` for custom letter pairs. 4. Tune `config.size` and `config.shape` to match surrounding typography. 5. Named avatars default to a name-derived palette via `AvatarConfig::name()`; use `AvatarColor::Neutral` explicitly when needed.
///
/// # Best Practices
///
/// ## Do's
///
/// * Always provide `config.name` for screen reader labels * Use consistent sizes within a list or comment thread * Fall back to initials when images fail to load (handled automatically) * Compose with [`Persona`](crate::Persona) when avatar plus text lines are needed
///
/// ## Don'ts
///
/// * Do not use avatars alone without adjacent name text in dense lists * Do not rely on initials alone when a photo is required for identification * An active focus ring around the avatar is not implemented — use surrounding layout cues when selection state matters
///
/// # Examples
///
/// ## Name-derived initials
/// Name-derived initials appear when no image is provided — the common pattern in lists and comments.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig};
/// view! {
///     <div data-testid="avatar-preview">
///         <Avatar config=AvatarConfig::name("Jane Doe") />
///     </div>
/// }
/// ```
///
/// ## Profile image
/// Photo avatars display the image with initials as fallback on load error.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig};
/// view! {
///     <div data-testid="avatar-image">
///         <Avatar config=AvatarConfig::image("https://picsum.photos/64", "Jane Doe") />
///     </div>
/// }
/// ```
///
/// ## Custom initials
/// Explicit initials override auto-generation from the display name.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig};
/// view! {
///     <div data-testid="avatar-initials">
///         <Avatar config=AvatarConfig::initials("AB") />
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Pixel `size` tokens align avatars with surrounding typography and layout grids.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarShape, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="avatar-sizes">
///         <Flex gap=FlexGap::Medium align=FlexAlign::Center>
///             <div data-testid="avatar-size-24"><Avatar config=AvatarConfig::sized("AB", 24) /></div>
///             <div data-testid="avatar-size-40"><Avatar config=AvatarConfig::sized("CD", 40) /></div>
///             <div data-testid="avatar-size-56"><Avatar config=AvatarConfig::sized("EF", 56) /></div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Shape variants
/// Circular and square shapes suit profile lists and compact entity chips.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarShape, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="avatar-shapes">
///         <Flex gap=FlexGap::Medium align=FlexAlign::Center>
///             <div data-testid="avatar-shape-circular"><Avatar config=AvatarConfig::shaped("AB", AvatarShape::Circular) /></div>
///             <div data-testid="avatar-shape-square"><Avatar config=AvatarConfig::shaped("CD", AvatarShape::Square) /></div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Theme background token
/// Initials fallback uses the neutral avatar surface token with a visible stroke.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor};
/// view! {
///     <div data-testid="avatar-theme">
///         <Avatar config=AvatarConfig::with_color("Theme User", AvatarColor::Neutral) />
///     </div>
/// }
/// ```
///
/// ## Name-derived palette
/// Display names map to stable palette colors when `color` is `Colorful`.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="avatar-colorful">
///         <Flex gap=FlexGap::Medium align=FlexAlign::Center>
///             <div data-testid="avatar-colorful-a">
///                 <Avatar config=AvatarConfig { name: Some("Alice Adams".into()), color: AvatarColor::Colorful, ..Default::default() } />
///             </div>
///             <div data-testid="avatar-colorful-b">
///                 <Avatar config=AvatarConfig { name: Some("Bob Baker".into()), color: AvatarColor::Colorful, ..Default::default() } />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Named color presets
/// Explicit palette colors for initials and icon fallback.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="avatar-named-colors">
///         <Flex gap=FlexGap::Medium align=FlexAlign::Center>
///             <div data-testid="avatar-color-crimson">
///                 <Avatar config=AvatarConfig::with_color("CR", AvatarColor::Crimson) />
///             </div>
///             <div data-testid="avatar-color-tangerine">
///                 <Avatar config=AvatarConfig::with_color("TL", AvatarColor::Tangerine) />
///             </div>
///             <div data-testid="avatar-color-plum">
///                 <Avatar config=AvatarConfig::with_color("PR", AvatarColor::Plum) />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Brand color
/// Brand-tinted initials for product-owned identity surfaces.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor};
/// view! {
///     <div data-testid="avatar-brand-color">
///         <Avatar config=AvatarConfig::with_color("BR", AvatarColor::Brand) />
///     </div>
/// }
/// ```
///
/// ## With presence badge
/// Availability dot anchored to an avatar — common in contact lists.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, PresenceBadge, PresenceStatus};
/// view! {
///     <div data-testid="avatar-with-presence">
///         <PresenceBadge status=PresenceStatus::Available>
///             <Avatar config=AvatarConfig::colored("Jane Doe") />
///         </PresenceBadge>
///     </div>
/// }
/// ```
///
/// ## With counter badge
/// Numeric overlay for unread counts on avatar hosts.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, CounterBadge};
/// view! {
///     <div data-testid="avatar-with-counter">
///         <CounterBadge count=7>
///             <Avatar config=AvatarConfig::colored("Jane Doe") />
///         </CounterBadge>
///     </div>
/// }
/// ```
///
/// ## Avatar group
/// Stacked avatars with overflow chip.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, AvatarGroup, AvatarGroupLayout, AvatarGroupSize};
/// view! {
///     <div data-testid="avatar-group-preview">
///         <AvatarGroup layout=AvatarGroupLayout::Stack size=AvatarGroupSize::S32 overflow=2u32>
///             <Avatar config=AvatarConfig { name: Some("Alice Adams".into()), color: AvatarColor::Crimson, size: Some(32), ..Default::default() } />
///             <Avatar config=AvatarConfig { name: Some("Bob Baker".into()), color: AvatarColor::Azure, size: Some(32), ..Default::default() } />
///             <Avatar config=AvatarConfig { name: Some("Cal Carter".into()), color: AvatarColor::Tangerine, size: Some(32), ..Default::default() } />
///         </AvatarGroup>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "avatar",
    preview_label = "Avatar",
    preview_icon = icondata::AiUserOutlined,
)]
#[component]
pub fn Avatar(
    /// Optional CSS class on the avatar root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Image, initials, shape, and size settings grouped together.
    #[prop(optional, into)]
    config: AvatarConfig,
) -> impl IntoView {
    inject_style("orbital-avatar", avatar_styles());

    let src = MaybeProp::from(config.src);
    let name = MaybeProp::from(config.name);
    let initials = MaybeProp::from(config.initials);
    let shape = Signal::from(config.shape);
    let size = MaybeProp::from(config.size);
    let color = Signal::from(config.color);
    let id_for_color = MaybeProp::from(config.id_for_color);

    view! {
        <BaseAvatar
            class=class
            src=src
            name=name
            initials=initials
            shape=shape
            size=size
            color=color
            id_for_color=id_for_color
        />
    }
}
