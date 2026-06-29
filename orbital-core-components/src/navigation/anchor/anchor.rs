use leptos::prelude::*;
use orbital_base_components::BaseAnchor;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::anchor_styles;
use super::types::AnchorConfig;

/// In-page section rail — a scroll-spy table of contents beside long content.
///
/// Add [`AnchorLink`](crate::AnchorLink) children with `href="#section-id"` matching element ids on the page. The active link updates as the user scrolls; clicking a link scrolls the target into view. Set `config.offset_target` when scrolling a bounded container instead of the document. For cross-route navigation use [`Link`](crate::Link) or the router — not `Anchor`.
///
/// # When to use
///
/// - Long documentation or settings pages with in-page section navigation - Table-of-contents rails beside scrollable content - Nested section hierarchies via nested [`AnchorLink`](crate::AnchorLink) children
///
/// # Usage
///
/// 1. Render [`AnchorLink`](crate::AnchorLink) children with `href="#section-id"` matching element ids. 2. Place target sections in the page with matching `id` attributes. 3. Set `config.offset_target` when scrolling a bounded container instead of the document.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep link titles short — they truncate with ellipsis * Use nested links for subsection hierarchies
///
/// ## Don'ts
///
/// * Do not use Anchor for cross-route navigation — prefer [`Link`](crate::Link) or the router
///
/// # Examples
///
/// ## Basic rail
/// Side rail with two section links beside scrollable content.
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorLink};
/// view! {
///     <div data-testid="anchor-preview" style="display: flex; gap: 24px; width: 100%; max-width: 560px;">
///         <Anchor>
///             <AnchorLink title="Intro".to_string() href="#intro" />
///             <AnchorLink title="Details".to_string() href="#details" />
///         </Anchor>
///         <div style="height: 240px; overflow: auto; flex: 1;">
///             <h3 id="intro" data-testid="anchor-link-1">"Intro"</h3>
///             <p style="height: 120px">"Intro section content."</p>
///             <h3 id="details" data-testid="anchor-link-2">"Details"</h3>
///             <p data-testid="anchor-link-3">"Details section content."</p>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Nested links
/// Subsection links nest under a parent [`AnchorLink`](crate::AnchorLink).
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorLink};
/// view! {
///     <div data-testid="anchor-nested">
///         <Anchor>
///             <AnchorLink title="Overview".to_string() href="#overview" />
///             <AnchorLink title="API".to_string() href="#api">
///                 <AnchorLink title="REST".to_string() href="#rest" />
///             </AnchorLink>
///         </Anchor>
///     </div>
/// }
/// ```
///
/// ## Custom scroll container
/// `offset_target` scopes scroll-spy calculations to a bounded scrollport.
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorConfig, AnchorLink};
/// view! {
///     <div data-testid="anchor-offset" style="display: flex; gap: 16px; width: 100%; max-width: 560px;">
///         <Anchor config=AnchorConfig { offset_target: Some("#anchor-scroll".into()), ..Default::default() }>
///             <AnchorLink title="Top".to_string() href="#top" />
///             <AnchorLink title="Bottom".to_string() href="#bottom" />
///         </Anchor>
///         <div id="anchor-scroll" style="height: 200px; overflow: auto; flex: 1;">
///             <h4 id="top">"Top"</h4>
///             <p style="height: 180px">"…"</p>
///             <h4 id="bottom">"Bottom"</h4>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Active on scroll
/// Scrolling the content region updates the active link class on the rail.
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorConfig, AnchorLink};
/// view! {
///     <div data-testid="anchor-active" style="display: flex; gap: 16px; width: 100%; max-width: 560px;">
///         <Anchor config=AnchorConfig { offset_target: Some("#anchor-active-scroll".into()), ..Default::default() }>
///             <AnchorLink title="First".to_string() href="#first" />
///             <AnchorLink title="Second".to_string() href="#second" />
///         </Anchor>
///         <div id="anchor-active-scroll" data-testid="anchor-active-scroll" style="height: 180px; overflow: auto; flex: 1;">
///             <h4 id="first">"First"</h4>
///             <p style="height: 160px">"…"</p>
///             <h4 id="second">"Second"</h4>
///             <p style="height: 80px">"…"</p>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Layout composition
/// Rail sits beside a multi-section article layout.
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorLink};
/// view! {
///     <div data-testid="anchor-layout" style="display: flex; gap: 24px; width: 100%; max-width: 640px;">
///         <Anchor>
///             <AnchorLink title="Setup".to_string() href="#setup" />
///             <AnchorLink title="Deploy".to_string() href="#deploy" />
///         </Anchor>
///         <article style="flex: 1;">
///             <section id="setup"><h3>"Setup"</h3><p>"Install and configure."</p></section>
///             <section id="deploy"><h3>"Deploy"</h3><p>"Ship to production."</p></section>
///         </article>
///     </div>
/// }
/// ```
///
/// ## Theme tokens
/// Active rail bar uses brand background tokens from the Orbital theme.
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorLink};
/// view! {
///     <div data-testid="anchor-theme" style="display: flex; gap: 16px; width: 100%; max-width: 560px;">
///         <Anchor>
///             <AnchorLink title="Theme".to_string() href="#theme-section" />
///         </Anchor>
///         <div>
///             <h4 id="theme-section" data-testid="anchor-theme-target">"Theme section"</h4>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "anchor",
    preview_label = "Anchor",
    preview_icon = icondata::AiLinkOutlined,
)]
#[component]
pub fn Anchor(
    /// Scroll offset target and other anchor behavior settings.
    #[prop(default = AnchorConfig::default())]
    config: AnchorConfig,
    /// Extra CSS class names merged onto the anchor rail root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// [`AnchorLink`] children — one per in-page section target.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-anchor", anchor_styles());

    view! {
        <BaseAnchor class=class nostrip:offset_target=config.offset_target>
            {children()}
        </BaseAnchor>
    }
}
