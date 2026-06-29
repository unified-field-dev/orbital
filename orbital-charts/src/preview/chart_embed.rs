//! Chart embed preview — scroll host and dialog scenarios.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Charts embedded in scroll and dialog hosts — overlay chrome escapes clip via
/// [`ChartOverlayLayer`](crate::ChartOverlayLayer) and `position: fixed` tooltips.
///
/// # When to use
///
/// - Dashboard tiles inside [`ScrollArea`](orbital_core_components::ScrollArea) where tooltips must not clip.
/// - Modal analytics with [`Dialog`](orbital_core_components::Dialog) and chart tooltips.
///
/// # Usage
///
/// 1. Set `embed_mode=ChartEmbedMode::ScrollHost` when the chart sits in a scroll parent.
/// 2. Set `embed_mode=ChartEmbedMode::DialogHost` for charts inside dialogs.
/// 3. Enable `tooltip` so the overlay layer demonstrates fixed positioning.
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap scroll hosts with explicit height so the scrollport has bounds.
/// * Use `data-orbital-chart-host` on table cells with `ChartEmbedMode::TableCell`.
///
/// ## Don'ts
///
/// * Do not rely on default inline embed when the parent applies `overflow: hidden`.
///
/// # Examples
///
/// ## Scroll and dialog embed
/// Bar chart in a clipped scroll area and inside a dialog — tooltips use fixed positioning.
/// <!-- preview -->
/// ```rust,ignore
/// use leptos::prelude::*;
/// use orbital_core_components::{
///     Button, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface, DialogTitle,
///     ScrollArea,
/// };
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::shared::BarPlot;
/// use crate::{ChartContainer, ChartEmbedMode, TooltipConfig};
/// let dialog_open = RwSignal::new(false);
/// view! {
///     <div data-testid="chart-embed-preview" style="display:flex;flex-direction:column;gap:1.5rem;">
///         <div data-testid="chart-embed-scroll-preview">
///             <ScrollArea style="display: block; width: 100%; height: 200px; box-sizing: border-box; border: 1px solid var(--orb-color-border-default); overflow: hidden;">
///                 <ChartContainer
///                     series=Some(vec![revenue_series(), cost_series()])
///                     x_axis=Some(vec![quarter_x_axis()])
///                     y_axis=Some(vec![revenue_y_axis()])
///                     grid=Some(full_grid())
///                     tooltip=Some(TooltipConfig::item())
///                     embed_mode=ChartEmbedMode::ScrollHost
///                     width=Some(560.0)
///                     height=Some(320.0)
///                 >
///                     <BarPlot />
///                 </ChartContainer>
///             </ScrollArea>
///         </div>
///         <div data-testid="chart-embed-dialog-preview">
///             <Button on:click=move |_| dialog_open.set(true)>"Open chart dialog"</Button>
///             <Dialog open=dialog_open>
///                 <DialogSurface>
///                     <DialogBody>
///                         <DialogTitle>"Chart in dialog"</DialogTitle>
///                         <DialogContent>
///                             <ChartContainer
///                                 series=Some(vec![revenue_series(), cost_series()])
///                                 x_axis=Some(vec![quarter_x_axis()])
///                                 y_axis=Some(vec![revenue_y_axis()])
///                                 grid=Some(full_grid())
///                                 tooltip=Some(TooltipConfig::item())
///                                 embed_mode=ChartEmbedMode::DialogHost
///                                 width=Some(480.0)
///                                 height=Some(280.0)
///                             >
///                                 <BarPlot />
///                             </ChartContainer>
///                         </DialogContent>
///                         <DialogActions>
///                             <Button on:click=move |_| dialog_open.set(false)>"Close"</Button>
///                         </DialogActions>
///                     </DialogBody>
///                 </DialogSurface>
///             </Dialog>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "chart-embed",
    preview_label = "Chart Embed",
    preview_icon = icondata::AiBlockOutlined,
)]
#[component]
pub fn ChartEmbedPreview() -> impl IntoView {
    view! { () }
}
