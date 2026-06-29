use leptos::prelude::*;
use orbital_base_components::{AvatarGroupLayout, AvatarGroupSize, BaseAvatarGroup};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::avatar_group_styles;

/// Lays out multiple [`Avatar`](crate::Avatar) children in stacked or spread arrangements.
///
/// Set `layout=Stack` for overlap and `overflow` to show a `+N` chip for hidden members — popover overflow is not implemented. For a single person glyph use [`Avatar`](crate::Avatar) directly. For avatar plus text lines use [`Persona`](crate::Persona).
///
/// # Examples
///
/// ## Stacked group
/// Overlapping avatars with distinct palette colors for clear separation.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, AvatarGroup, AvatarGroupLayout, AvatarGroupSize};
/// view! {
///     <div data-testid="avatar-group-preview">
///     <div data-testid="avatar-group-stack">
///         <AvatarGroup layout=AvatarGroupLayout::Stack size=AvatarGroupSize::S32>
///             <Avatar config=AvatarConfig { name: Some("Alice Adams".into()), color: AvatarColor::Crimson, size: Some(32), ..Default::default() } />
///             <Avatar config=AvatarConfig { name: Some("Bob Baker".into()), color: AvatarColor::Azure, size: Some(32), ..Default::default() } />
///             <Avatar config=AvatarConfig { name: Some("Cal Carter".into()), color: AvatarColor::Tangerine, size: Some(32), ..Default::default() } />
///         </AvatarGroup>
///     </div>
///     </div>
/// }
/// ```
///
/// ## Spread layout
/// Avatars with spacing instead of overlap.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, AvatarGroup, AvatarGroupLayout};
/// view! {
///     <div data-testid="avatar-group-spread">
///         <AvatarGroup layout=AvatarGroupLayout::Spread>
///             <Avatar config=AvatarConfig::with_color("Dev 1", AvatarColor::Plum) />
///             <Avatar config=AvatarConfig::with_color("Dev 2", AvatarColor::Marigold) />
///             <Avatar config=AvatarConfig::with_color("Dev 3", AvatarColor::Ruby) />
///         </AvatarGroup>
///     </div>
/// }
/// ```
///
/// ## Overflow indicator
/// Show three avatars plus a `+2` chip for additional members.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, AvatarGroup, AvatarGroupLayout, AvatarGroupSize};
/// view! {
///     <div data-testid="avatar-group-overflow">
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
    preview_slug = "avatar-group",
    preview_label = "Avatar Group",
    preview_icon = icondata::AiTeamOutlined,
)]
#[component]
pub fn AvatarGroup(
    /// Optional CSS class on the group root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Avatar size preset applied to layout spacing.
    #[prop(optional, into)]
    size: Signal<AvatarGroupSize>,
    /// Overlap (`Stack`) or spaced (`Spread`) layout.
    #[prop(optional, into)]
    layout: Signal<AvatarGroupLayout>,
    /// Hidden member count rendered as a `+N` overflow chip.
    #[prop(optional, into)]
    overflow: MaybeProp<u32>,
    /// Child avatars in the group.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-avatar-group", avatar_group_styles());

    view! {
        <BaseAvatarGroup class=class layout=layout size=size overflow=overflow>
            {children()}
        </BaseAvatarGroup>
    }
}
