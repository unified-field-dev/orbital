use leptos::prelude::*;
use orbital_base_components::BaseSpace;
use orbital_macros::component_doc;

use super::types::SpaceConfig;

/// Distribution layout that pushes children to opposite edges of a full-width row.
///
/// Also called a **distribution row** in Orbital docs. This is **not** Ant Design `Space` (even gaps between siblings) and **not** the design-token "spacing" concept — it defaults to `justify=SpaceBetween`.
///
/// `Space` is a convenience wrapper over [`Flex`](crate::Flex) with `full_width=true` and `justify=SpaceBetween` by default. Use for toolbars, page headers, and footer action bars. Reach for [`Stack`](crate::Stack) when you need even gaps between every child. Reach for [`Flex`](crate::Flex) when you need wrap, inline placement, fill, or inset padding.
///
/// Use [`SpaceConfig::even_gap`](crate::SpaceConfig::even_gap) only as an escape hatch when you need even gaps — prefer [`Stack`](crate::Stack) for that pattern.
///
/// # When to use
///
/// - Page headers and footers with content at opposite edges - Toolbar rows that separate a title cluster from trailing actions - Any full-width row where main-axis distribution matters more than even gap
///
/// # Usage
///
/// 1. Use default `Space` for a horizontal space-between row. 2. Set `config.vertical=true` for column distribution. 3. Override `config.justify` when you need center, end, or even-gap layout.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `Space` for opposite-edge distribution in full-width regions * Reach for [`Stack`](crate::Stack) for even-gap vertical sections and button rows * Reach for [`Flex`](crate::Flex) when you need wrap, inline, fill, or padding props
///
/// ## Don'ts
///
/// * Do not use `Space` for even-gap stacks — prefer [`Stack`](crate::Stack) * Do not use `Space` for two-dimensional page grids — prefer [`Grid`](crate::Grid)
///
/// # Examples
///
/// ## Default (space-between)
/// Items distribute to opposite edges of a full-width row — the default `Space` behavior.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Space};
/// view! {
///     <div data-testid="space-preview" style="width: 100%; max-width: 560px;">
///         <Space>
///             <DemoBox data_testid="space-item-1">"Start"</DemoBox>
///             <DemoBox data_testid="space-item-2">"End"</DemoBox>
///         </Space>
///     </div>
/// }
/// ```
///
/// ## Vertical distribution
/// Column direction pushes items to top and bottom edges.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Space, SpaceConfig, FlexGap};
/// view! {
///     <div data-testid="space-vertical" style="height: 120px; max-width: 200px;">
///         <Space config=SpaceConfig::vertical(FlexGap::Medium)>
///             <DemoBox data_testid="space-v-1">"Top"</DemoBox>
///             <DemoBox data_testid="space-v-2">"Bottom"</DemoBox>
///         </Space>
///     </div>
/// }
/// ```
///
/// ## Even gap override
/// Override `justify` to `None` for even gaps — prefer [`Stack`](crate::Stack) for this pattern.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Space, SpaceConfig, FlexGap};
/// view! {
///     <div data-testid="space-even-gap" style="width: 100%; max-width: 560px;">
///         <Space config=SpaceConfig::even_gap(FlexGap::Medium)>
///             <DemoBox data_testid="space-gap-a">"One"</DemoBox>
///             <DemoBox>"Two"</DemoBox>
///         </Space>
///     </div>
/// }
/// ```
///
/// ## Align center
/// Cross-axis center alignment with mixed-height children.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Space, SpaceConfig, FlexAlign};
/// view! {
///     <div data-testid="space-align" style="height: 72px; width: 100%; max-width: 560px;">
///         <Space config=SpaceConfig { align: Some(FlexAlign::Center), ..Default::default() }>
///             <DemoBox height="32px">"Short"</DemoBox>
///             <DemoBox height="56px">"Tall"</DemoBox>
///         </Space>
///     </div>
/// }
/// ```
///
/// ## Justify center
/// Override the default to center items along the main axis.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Space, SpaceConfig, FlexJustify};
/// view! {
///     <div data-testid="space-justify" style="width: 100%; max-width: 560px;">
///         <Space config=SpaceConfig { justify: Some(FlexJustify::Center), ..Default::default() }>
///             <DemoBox>"Center"</DemoBox>
///             <DemoBox>"Center"</DemoBox>
///         </Space>
///     </div>
/// }
/// ```
///
/// ## Toolbar pattern
/// Title cluster on the left, actions on the right — typical space-between usage.
/// <!-- preview -->
/// ```rust
/// use crate::{Space, Button, ButtonAppearance, Badge, Subtitle1};
/// view! {
///     <div data-testid="space-toolbar" style="width: 100%; max-width: 560px;">
///         <Space>
///             <Subtitle1>"Documents"</Subtitle1>
///             <span>
///                 <Button appearance=ButtonAppearance::Primary>"Upload"</Button>
///                 " "
///                 <Badge>"3 new"</Badge>
///             </span>
///         </Space>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "space",
    preview_label = "Space",
    preview_icon = icondata::AiColumnWidthOutlined,
)]
#[component]
pub fn Space(
    /// Distribution and alignment configuration.
    #[prop(default = SpaceConfig::default())]
    config: SpaceConfig,
    /// Extra CSS class names merged onto the full-width flex container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Flex item children distributed along the main axis (opposite edges by default).
    children: Children,
) -> impl IntoView {
    view! {
        <BaseSpace
            class=class
            gap=config.gap
            vertical=config.vertical
            align=MaybeProp::from(config.align)
            justify=MaybeProp::from(config.justify)
        >
            {children()}
        </BaseSpace>
    }
}
