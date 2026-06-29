//! Responsive CSS auto-fill grid layout.

use leptos::prelude::*;
use orbital_macros::component_doc;

use super::spacing::SpacingSize;
use crate::components::Card;
use crate::primitives::{Flex, FlexJustify, MaterialElevation};

/// A responsive grid that reflows columns based on available width.
///
/// Uses `repeat(auto-fill, minmax(min, 1fr))` — **`auto-fill`**, not `auto-fit` — so column count adapts without media queries. Complements fixed-column [`Grid`](crate::primitives::Grid).
///
/// # When to use
///
/// - Card grids and icon tiles that should fill available width - Dashboard app pickers and catalog layouts - Prefer [`Grid`](crate::primitives::Grid) when you need an exact column count
///
/// # Examples
///
/// ## Default
/// Responsive card grid that reflows column count from a minimum tile width.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use {Card, Flex, MaterialElevation};
///
/// view! {
///     <div data-testid="auto-grid-preview">
///     <AutoGrid min="200px" gap=SpacingSize::Size160>
///         <Card elevation=MaterialElevation::Raised>
///             <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
///                 "Item 1"
///             </Flex>
///         </Card>
///         <Card elevation=MaterialElevation::Raised>
///             <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
///                 "Item 2"
///             </Flex>
///         </Card>
///         <Card elevation=MaterialElevation::Raised>
///             <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
///                 "Item 3"
///             </Flex>
///         </Card>
///         <Card elevation=MaterialElevation::Raised>
///             <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
///                 "Item 4"
///             </Flex>
///         </Card>
///     </AutoGrid>
///     </div>
/// }
/// ```
///
/// ## Card Grid (270px min)
/// Dashboard-style tiles at a wider minimum column width for richer card content.
/// <!-- preview -->
/// ```rust
/// use {Card, Flex, MaterialElevation};
///
/// view! {
///     <AutoGrid min="270px" gap=SpacingSize::Size160>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 100px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Dashboard"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 100px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Analytics"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 100px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Settings"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 100px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Users"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 100px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Reports"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 100px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Billing"</Flex></div></Card>
///     </AutoGrid>
/// }
/// ```
///
/// ## Narrow Items (120px min)
/// Compact icon or label tiles with a tighter minimum column width.
/// <!-- preview -->
/// ```rust
/// use {Card, Flex, FlexJustify, MaterialElevation};
///
/// view! {
///     <AutoGrid min="120px" gap=SpacingSize::Size120>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Alpha"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Bravo"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Charlie"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Delta"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Echo"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Foxtrot"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Golf"</Flex></Card>
///         <Card elevation=MaterialElevation::Raised><Flex justify=FlexJustify::Center padding=SpacingSize::Size120.inset()>"Hotel"</Flex></Card>
///     </AutoGrid>
/// }
/// ```
///
/// ## Custom Gap (24px)
/// Wider grid gap for layouts that need more breathing room between tiles.
/// <!-- preview -->
/// ```rust
/// use {Card, Flex, MaterialElevation};
///
/// view! {
///     <AutoGrid min="200px" gap=SpacingSize::Size240>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 80px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Wider spacing"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 80px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"Between items"</Flex></div></Card>
///         <Card elevation=MaterialElevation::Raised><div style="min-height: 80px;"><Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>"For breathing room"</Flex></div></Card>
///     </AutoGrid>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "auto-grid",
    preview_label = "Auto Grid",
    preview_icon = icondata::AiTableOutlined,
)]
#[component]
pub fn AutoGrid(
    /// Minimum column width (e.g. `"270px"`, `"20rem"`). Columns will never be narrower than this value; extra space is distributed equally across all columns.
    #[prop(into)]
    min: Signal<String>,
    /// Gap between grid items, specified as an Orbital spacing ramp value. Defaults to `SpacingSize::Size160` (16 px).
    #[prop(default = SpacingSize::Size160)]
    gap: SpacingSize,
    /// Optional CSS class to merge onto the grid container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Child content (grid items).
    children: Children,
) -> impl IntoView {
    let gap_px = gap.px();

    let style = Memo::new(move |_| {
        format!(
            "display:grid;grid-template-columns:repeat(auto-fill,minmax({},1fr));gap:{}px;",
            min.get(),
            gap_px,
        )
    });

    let grid_class = move || match class.get() {
        Some(extra) if !extra.trim().is_empty() => extra,
        _ => String::new(),
    };

    view! {
        <div class=grid_class style=move || style.get()>
            {children()}
        </div>
    }
}
