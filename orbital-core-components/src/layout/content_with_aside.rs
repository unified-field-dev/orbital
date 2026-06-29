use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::content_with_aside_slots::{Aside, Content};
use super::styles::content_with_aside_styles;

/// Two-column layout for documentation-style pages: growing content + sticky aside.
///
/// Place [`ContentWithAside`] inside [`LayoutMain`](crate::LayoutMain) when the main region needs a primary reading column and a secondary rail (table of contents, metadata, or related links). The content column grows; the aside minimally fits and stays sticky on wide viewports (typically when the viewport is wide enough for the two-column grid — the aside stops being sticky on narrow breakpoints).
///
/// # When to use
///
/// - Long documentation or design-language pages with an in-page anchor rail - Settings or guide pages where secondary navigation sits beside prose - Any main content area that pairs scrollable copy with a compact right column
///
/// # Usage
///
/// 1. Nest inside [`Layout`](crate::Layout) → [`LayoutMain`](crate::LayoutMain). 2. Put the page title, lead copy, and body sections in [`Content`]. 3. Put the table of contents or utility rail in [`Aside`] — often an [`Anchor`](crate::Anchor). 4. Slot source order does not matter; CSS places content left and aside right.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep the hero (title + lead) inside [`Content`] so the aside top-aligns with the page title * Use [`Anchor`](crate::Anchor) + [`AnchorLink`](crate::AnchorLink) in the aside for in-page navigation * Reserve the aside for short labels — long copy belongs in the content column
///
/// ## Don'ts
///
/// * Do not place full-width chrome above [`ContentWithAside`] if the aside should align with the title * Do not use the aside for primary page actions — keep CTAs in the content column
///
/// # Examples
///
/// ## Basic content and aside
/// Two-column layout with filled Material regions labeling each column.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Aside, Content, ContentWithAside, Material, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="content-with-aside-preview" style="width: 100%;">
///         <ContentWithAside>
///             <Content slot>
///                 <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///                     <div style="background: var(--orb-color-surface-subtle); min-height: 120px; display: flex; align-items: center; justify-content: center;">
///                         "Content"
///                     </div>
///                 </Material>
///             </Content>
///             <Aside slot>
///                 <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
///                     <div style="background: var(--orb-color-surface-overlay); min-height: 72px; display: flex; align-items: center; justify-content: center;">
///                         "Aside"
///                     </div>
///                 </Material>
///             </Aside>
///         </ContentWithAside>
///     </div>
/// }
/// ```
///
/// ## Sticky aside while scrolling
/// Tall content with a bounded scrollport; the aside stays sticky and carries a typical doc-page anchor rail.
/// <!-- preview -->
/// ```rust
/// use crate::{Anchor, AnchorLink, Aside, Content, ContentWithAside, SectionTitle};
/// view! {
///     <div data-testid="content-with-aside-sticky" style="height: 240px; overflow: auto; width: 100%; max-width: 560px;">
///         <ContentWithAside>
///             <Content slot>
///                 <h3 id="top">"Top"</h3>
///                 <p style="height: 200px">"Scrollable section."</p>
///                 <h3 id="bottom">"Bottom"</h3>
///                 <p style="height: 120px">"More content."</p>
///             </Content>
///             <Aside slot>
///                 <SectionTitle>"On this page"</SectionTitle>
///                 <Anchor>
///                     <AnchorLink title="Top".to_string() href="#top" />
///                     <AnchorLink title="Bottom".to_string() href="#bottom" />
///                 </Anchor>
///             </Aside>
///         </ContentWithAside>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "content-with-aside",
    preview_label = "Content With Aside",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn ContentWithAside(
    /// Extra CSS class names merged onto the two-column grid root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Primary content slot — main article or canvas region (flex-grows to fill space).
    #[prop(optional)]
    content: Option<Content>,
    /// Secondary aside slot — sidebar, inspector, or metadata panel on the trailing edge.
    #[prop(optional)]
    aside: Option<Aside>,
) -> impl IntoView {
    inject_style("orbital-content-with-aside", content_with_aside_styles());

    let root_class = move || {
        let mut parts = vec!["orbital-content-with-aside".to_string()];
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    };

    view! {
        <div class=root_class>
            {content.map(|slot| view! {
                <div class="orbital-content-with-aside__content">
                    {(slot.children)()}
                </div>
            })}
            {aside.map(|slot| view! {
                <aside class="orbital-content-with-aside__aside">
                    {(slot.children)()}
                </aside>
            })}
        </div>
    }
}
