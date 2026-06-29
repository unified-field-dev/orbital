use leptos::prelude::*;
use orbital_base_components::BaseStack;
use orbital_macros::component_doc;

use super::types::StackConfig;

/// One-dimensional layout with a consistent gap between every child.
///
/// Convenience wrapper over [`Flex`](crate::Flex) with even-gap, full-width defaults. Equivalent to `Flex(vertical=true, full_width=true)` with optional direction override. Set `config.horizontal=true` for a row direction.
///
/// Default presets: column direction, `FlexGap::Medium`, `full_width=true`, no justify override. Reach for [`Space`](crate::Space) to push children to opposite edges. Reach for [`Flex`](crate::Flex) when you need wrap, inline, fill, or inset padding.
///
/// # When to use
///
/// - Vertical form sections, settings blocks, and button rows - Even gaps between every child on one axis - Opinionated defaults over raw flex props
///
/// # Usage
///
/// 1. Pass `config=StackConfig::vertical(FlexGap::Medium)` or customize fields on `StackConfig`. 2. Set `config.horizontal=true` for row direction. 3. Tune `config.align` / `config.justify` for distribution.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `Stack` for simple vertical sections and button groups with even gaps * Reach for [`Space`](crate::Space) for opposite-edge distribution (space-between) * Reach for [`Flex`](crate::Flex) when you need wrap, inline, fill, or padding props
///
/// ## Don'ts
///
/// * Do not use `Stack` for two-dimensional page grids — prefer [`Grid`](crate::Grid)
///
/// # Examples
///
/// ## Horizontal cluster
/// Default row direction spaces inline items with medium gap.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Stack, StackConfig, FlexGap};
/// view! {
///     <div data-testid="stack-preview">
///         <Stack config=StackConfig::horizontal(FlexGap::Medium)>
///             <DemoBox data_testid="stack-item-1">"One"</DemoBox>
///             <DemoBox data_testid="stack-item-2">"Two"</DemoBox>
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Vertical stack
/// Column direction stacks children with explicit medium gap.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Stack, StackConfig, FlexGap};
/// view! {
///     <div data-testid="stack-vertical">
///         <Stack config=StackConfig::vertical(FlexGap::Medium)>
///             <DemoBox data_testid="stack-v-1">"First"</DemoBox>
///             <DemoBox data_testid="stack-v-2">"Second"</DemoBox>
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Gap preset matrix
/// Compare small and large gap presets side by side.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Stack, StackConfig, FlexGap};
/// view! {
///     <div data-testid="stack-gap-matrix" style="width: 100%; max-width: 560px;">
///         <Stack config=StackConfig { gap: FlexGap::Small, horizontal: true, ..Default::default() }>
///             <DemoBox data_testid="stack-gap-small">"Small"</DemoBox>
///             <DemoBox>"gap"</DemoBox>
///         </Stack>
///         <Stack config=StackConfig { gap: FlexGap::Large, horizontal: true, ..Default::default() }>
///             <DemoBox data_testid="stack-gap-large">"Large"</DemoBox>
///             <DemoBox>"gap"</DemoBox>
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Align center
/// Cross-axis centering with mixed-height children in a bounded frame.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Stack, StackConfig, FlexGap, FlexAlign};
/// view! {
///     <div data-testid="stack-align" style="width: 100%; max-width: 560px; height: 120px; border: 1px solid var(--orb-color-border-default); padding: 8px;">
///         <Stack config=StackConfig { gap: FlexGap::Medium, horizontal: true, align: Some(FlexAlign::Center), ..Default::default() }>
///             <DemoBox height="32px">"Short"</DemoBox>
///             <DemoBox height="72px" data_testid="stack-align-tall">"Tall"</DemoBox>
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Justify space-between
/// Main-axis distribution pushes items to opposite edges.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Stack, StackConfig, FlexGap, FlexJustify};
/// view! {
///     <div data-testid="stack-justify" style="width: 100%; max-width: 560px;">
///         <Stack config=StackConfig { gap: FlexGap::Medium, horizontal: true, justify: Some(FlexJustify::SpaceBetween), ..Default::default() }>
///             <DemoBox data_testid="stack-justify-start">"Start"</DemoBox>
///             <DemoBox data_testid="stack-justify-end">"End"</DemoBox>
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Mixed child types
/// Buttons, text, and badges share the same stack gap rhythm.
/// <!-- preview -->
/// ```rust
/// use crate::{Stack, StackConfig, FlexGap, Button, ButtonAppearance, Badge};
/// view! {
///     <div data-testid="stack-mixed">
///         <Stack config=StackConfig::horizontal(FlexGap::Medium)>
///             <Button appearance=ButtonAppearance::Primary>"Action"</Button>
///             <span>"Status"</span>
///             <Badge>"New"</Badge>
///         </Stack>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "stack",
    preview_label = "Stack",
    preview_icon = icondata::AiColumnHeightOutlined,
)]
#[component]
pub fn Stack(
    /// Gap, direction, and alignment settings for the stack.
    #[prop(default = StackConfig::default())]
    config: StackConfig,
    /// Extra CSS class names merged onto the flex stack container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Flex item children laid out with even gaps between each sibling.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseStack
            class=class
            gap=config.gap
            horizontal=config.horizontal
            align=MaybeProp::from(config.align)
            justify=MaybeProp::from(config.justify)
        >
            {children()}
        </BaseStack>
    }
}
