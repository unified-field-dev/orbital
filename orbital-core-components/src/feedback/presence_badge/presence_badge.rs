use leptos::prelude::*;
use orbital_base_components::{BasePresenceBadge, PresenceBadgeSize, PresenceStatus};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::presence_badge_styles;

/// Availability dot on an avatar or by itself.
///
/// `PresenceBadge` shows availability as a colored dot using Orbital status tokens (`--orb-color-status-success-fg`, `--orb-color-status-warning-fg`, `--orb-color-status-danger-fg`). Set `status` to `Available`, `Away`, `Busy`, `Offline`, `OutOfOffice`, or `Unknown`. Wrap an [`Avatar`](crate::Avatar) as a child to anchor the dot at the bottom-right — common in people lists and messaging UI.
///
/// For inline text labels use [`Badge`](crate::Badge); for numeric overlays use [`CounterBadge`](crate::CounterBadge).
///
/// # When to use
///
/// - Contact lists and chat headers where a person glyph needs an availability signal - Status legends that explain dot colors - Standalone dots when no avatar host is needed
///
/// # Examples
///
/// ## Default presence dot
/// Standalone presence indicator for status legends.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{PresenceBadge, PresenceStatus};
/// view! {
///     <div data-testid="presence-badge-preview" style="display: inline-flex; min-width: 32px; min-height: 32px; align-items: center; justify-content: center;">
///         <PresenceBadge status=PresenceStatus::Available />
///     </div>
/// }
/// ```
///
/// ## On avatar
/// Presence dot anchored to an avatar host — common in contact lists and chat.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, PresenceBadge, PresenceStatus};
/// view! {
///     <div data-testid="presence-badge-avatar">
///         <PresenceBadge status=PresenceStatus::Available>
///             <Avatar config=AvatarConfig::colored("Jane Doe") />
///         </PresenceBadge>
///     </div>
/// }
/// ```
///
/// ## Status matrix
/// Supported availability states side by side, including out-of-office and unknown.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, AvatarColor, Flex, FlexAlign, FlexGap, PresenceBadge, PresenceStatus};
/// view! {
///     <div data-testid="presence-badge-statuses">
///         <Flex gap=FlexGap::Medium align=FlexAlign::Center>
///             <div data-testid="presence-available">
///                 <PresenceBadge status=PresenceStatus::Available>
///                     <Avatar config=AvatarConfig::with_color("Ava", AvatarColor::Crimson) />
///                 </PresenceBadge>
///             </div>
///             <div data-testid="presence-away">
///                 <PresenceBadge status=PresenceStatus::Away>
///                     <Avatar config=AvatarConfig::with_color("Ben", AvatarColor::Azure) />
///                 </PresenceBadge>
///             </div>
///             <div data-testid="presence-busy">
///                 <PresenceBadge status=PresenceStatus::Busy>
///                     <Avatar config=AvatarConfig::with_color("Cal", AvatarColor::Tangerine) />
///                 </PresenceBadge>
///             </div>
///             <div data-testid="presence-offline">
///                 <PresenceBadge status=PresenceStatus::Offline>
///                     <Avatar config=AvatarConfig::with_color("Dee", AvatarColor::Ash) />
///                 </PresenceBadge>
///             </div>
///             <div data-testid="presence-out-of-office">
///                 <PresenceBadge status=PresenceStatus::OutOfOffice>
///                     <Avatar config=AvatarConfig::with_color("Eve", AvatarColor::Plum) />
///                 </PresenceBadge>
///             </div>
///             <div data-testid="presence-unknown">
///                 <PresenceBadge status=PresenceStatus::Unknown>
///                     <Avatar config=AvatarConfig::with_color("Fin", AvatarColor::Marigold) />
///                 </PresenceBadge>
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "presence-badge",
    preview_label = "Presence Badge",
    preview_icon = icondata::AiWifiOutlined,
)]
#[component]
pub fn PresenceBadge(
    /// Optional CSS class on the badge wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Presence state.
    status: PresenceStatus,
    /// Size preset aligned with the host avatar.
    #[prop(optional, into)]
    size: Signal<PresenceBadgeSize>,
    /// Optional child content (avatar host).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-presence-badge", presence_badge_styles());

    let status = Signal::from(status);

    view! {
        <BasePresenceBadge class=class status=status size=size>
            {children.map(|c| c())}
        </BasePresenceBadge>
    }
}
