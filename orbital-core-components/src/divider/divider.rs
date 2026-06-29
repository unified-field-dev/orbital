use leptos::prelude::*;
use orbital_base_components::BaseDivider;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::divider_styles;

/// Visual separator between sections or inline clusters.
///
/// Set `vertical=true` for a vertical rule between items in a horizontal row. Optional children render a centered label on the line. The component sets `role="separator"` internally.
///
/// # When to use
///
/// - Separate form sections, settings groups, or toolbar clusters - Vertical dividers between inline actions in a [`Flex`] row
///
/// # Usage
///
/// 1. Default horizontal divider between stacked blocks. 2. Set `vertical=true` inside a horizontal flex row. 3. Pass optional children for a labeled divider ("OR", section title).
///
/// # Best Practices
///
/// ## Do's
///
/// * Use horizontal dividers between stacked sections * Use vertical dividers in toolbars with [`Flex`] * Provide labeled text via optional children ("OR", section title) * `role="separator"` is handled by the component — do not set it again on children
///
/// ## Don'ts
///
/// * Do not use dividers as the only visual grouping — pair with headings
///
/// # Examples
///
/// ## Horizontal between blocks
/// A horizontal rule separates stacked content blocks in forms and settings pages.
/// <!-- preview -->
/// ```rust
/// use crate::{Divider, Flex, FlexAlign};
/// view! {
///     <div data-testid="divider-preview" style="width: 100%;">
///         <Flex vertical=true align=FlexAlign::Stretch full_width=true>
///             <p>"Above"</p>
///             <Divider />
///             <p>"Below"</p>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Vertical in Flex toolbar
/// Vertical separator between inline items in a horizontal flex row.
/// <!-- preview -->
/// ```rust
/// use crate::{Divider, Flex, FlexAlign};
/// view! {
///     <div data-testid="divider-vertical" style="width: 100%;">
///         <div style="height: 48px; display: flex; align-items: stretch;">
///             <Flex align=FlexAlign::Stretch fill=true>
///                 <span>"Left"</span>
///                 <Divider vertical=true />
///                 <span>"Right"</span>
///             </Flex>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Theme stroke token
/// Divider lines use stroke tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::{Divider, Flex, FlexAlign};
/// view! {
///     <div data-testid="divider-theme" style="width: 100%;">
///         <Flex vertical=true align=FlexAlign::Stretch full_width=true>
///             <Divider />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Labeled divider
/// Optional children render centered text on the separator line.
/// <!-- preview -->
/// ```rust
/// use crate::{Divider, Flex, FlexAlign};
/// view! {
///     <div data-testid="divider-labeled" style="width: 100%;">
///         <Flex vertical=true align=FlexAlign::Stretch full_width=true>
///             <Divider>"OR"</Divider>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Toolbar cluster
/// Combine [`Flex`], [`Button`](crate::Button), and vertical [`Divider`] for compact toolbars.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Divider, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="divider-toolbar" style="width: 100%;">
///         <div style="height: 40px; display: flex; align-items: stretch;">
///             <Flex gap=FlexGap::Small align=FlexAlign::Stretch fill=true>
///                 <Button appearance=ButtonAppearance::Subtle>"Save"</Button>
///                 <Divider vertical=true />
///                 <Button appearance=ButtonAppearance::Subtle>"Cancel"</Button>
///             </Flex>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "divider",
    preview_label = "Divider",
    preview_icon = icondata::AiLineOutlined,
)]
#[component]
pub fn Divider(
    /// When true, renders a vertical separator.
    #[prop(optional)]
    vertical: bool,
    /// Optional CSS class merged onto the separator root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional label content centered on the line.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-divider", divider_styles());

    view! {
        <BaseDivider class=class vertical=Signal::from(vertical) labeled=children.is_some()>
            {children.map(|c| c())}
        </BaseDivider>
    }
}
