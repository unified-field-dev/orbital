use leptos::prelude::*;
use orbital_base_components::BaseGrid;
use orbital_macros::component_doc;

use super::types::GridConfig;

/// CSS grid layout for two-dimensional arrangements with a fixed column count.
///
/// This is Orbital's fixed-column CSS grid for predictable column layouts. Compose with [`GridItem`](crate::GridItem) children to control span and offset per cell. For fluid card tiles that reflow with viewport width, use [AutoGrid](/auto-grid) in the orbital crate.
///
/// # When to use
///
/// - Dashboard tiles, form columns, responsive card grids - When both row and column alignment matter
///
/// # Usage
///
/// 1. Set `config.cols` to the number of grid columns. 2. Wrap each cell in [`GridItem`]. 3. Use [`GridItemConfig::span`](crate::GridItemConfig::span) to span multiple columns; `offset` to skip columns. 4. Tune `config.x_gap` / `config.y_gap` for consistent spacing rhythm.
///
/// # Best Practices
///
/// ## Do's
///
/// * Set `cols` to match your column count * Wrap every child in [`GridItem`] for span and offset control * Use `GridItemConfig::span` for full-width rows inside multi-column grids * Tune `x_gap` / `y_gap` for consistent rhythm
///
/// ## Don'ts
///
/// * Do not use Grid for simple horizontal button rows — prefer [`Flex`](crate::Flex) * Do not place raw divs directly under `Grid` — use [`GridItem`]
///
/// # Examples
///
/// ## Three-column grid
/// Three equal columns with consistent gaps—typical for dashboard tiles, metric cards, or icon grids.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Grid, GridItem};
/// view! {
///     <div data-testid="grid-preview">
///         <Grid config=GridConfig::with_gaps(3, 12, 12)>
///             <GridItem><DemoBox data_testid="grid-cell-1">"One"</DemoBox></GridItem>
///             <GridItem><DemoBox data_testid="grid-cell-2">"Two"</DemoBox></GridItem>
///             <GridItem><DemoBox data_testid="grid-cell-3">"Three"</DemoBox></GridItem>
///         </Grid>
///     </div>
/// }
/// ```
///
/// ## Two columns with gap
/// Two-column layout with separate horizontal and vertical gaps for form pairs, settings rows, or side-by-side cards.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Grid, GridItem};
/// view! {
///     <div data-testid="grid-two-col">
///         <Grid config=GridConfig::with_gaps(2, 16, 8)>
///             <GridItem><DemoBox data_testid="grid-a">"A"</DemoBox></GridItem>
///             <GridItem><DemoBox data_testid="grid-b">"B"</DemoBox></GridItem>
///         </Grid>
///     </div>
/// }
/// ```
///
/// ## Spanning item via GridItem
/// `column=2` spans multiple grid columns so one cell can be wide while neighbors stay single-column.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Grid, GridItem, GridItemConfig};
/// view! {
///     <div data-testid="grid-span">
///         <Grid config=GridConfig::with_gaps(3, 8, 8)>
///             <GridItem config=GridItemConfig::span(2)><DemoBox data_testid="grid-wide">"Wide (span 2)"</DemoBox></GridItem>
///             <GridItem><DemoBox data_testid="grid-narrow">"N"</DemoBox></GridItem>
///             <GridItem config=GridItemConfig::span(3)><DemoBox data_testid="grid-full">"Full width (span 3)"</DemoBox></GridItem>
///         </Grid>
///     </div>
/// }
/// ```
///
/// ## Offset via GridItem
/// `offset=1` leaves an empty column at the start—useful for indented rows or staggered cards.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Grid, GridItem, GridItemConfig};
/// view! {
///     <div data-testid="grid-offset">
///         <Grid config=GridConfig::with_gaps(3, 8, 8)>
///             <GridItem config=GridItemConfig::with_offset(2, 1)>
///                 <DemoBox data_testid="grid-offset-cell">"Offset 1, span 2"</DemoBox>
///             </GridItem>
///         </Grid>
///     </div>
/// }
/// ```
///
/// ## Theme token
/// Cells use theme background tokens so grid layouts inherit light/dark surfaces from the provider.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Grid, GridItem};
/// view! {
///     <div data-testid="grid-theme">
///         <Grid config=GridConfig::with_gaps(2, 12, 12)>
///             <GridItem>
///                 <DemoBox data_testid="grid-theme-cell">"Themed cell"</DemoBox>
///             </GridItem>
///             <GridItem>
///                 <DemoBox>"Themed cell"</DemoBox>
///             </GridItem>
///         </Grid>
///     </div>
/// }
/// ```
///
/// ## Form columns with Field
/// Pair Grid with native [`Field`](crate::Field) for two-column settings or registration forms.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Grid, GridItem, GridConfig, Input, InputAppearance, FormBind};
/// let first = RwSignal::new(String::new());
/// let last = RwSignal::new(String::new());
/// view! {
///     <div data-testid="grid-form">
///         <Grid config=GridConfig::with_gaps(2, 16, 12)>
///             <GridItem>
///                 <Field label="First name">
///                     <Input bind=FormBind::from(first) appearance=InputAppearance::with_placeholder("Jane") />
///                 </Field>
///             </GridItem>
///             <GridItem>
///                 <Field label="Last name">
///                     <Input bind=FormBind::from(last) appearance=InputAppearance::with_placeholder("Doe") />
///                 </Field>
///             </GridItem>
///         </Grid>
///     </div>
/// }
/// ```
///
/// ## Single column
/// Explicit single-column stack when `cols=1` — useful for narrow panels or mobile-first layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Grid, GridItem, GridConfig};
/// view! {
///     <div data-testid="grid-single-col">
///         <Grid config=GridConfig::new(1)>
///             <GridItem><DemoBox data_testid="grid-single-1">"Row 1"</DemoBox></GridItem>
///             <GridItem><DemoBox data_testid="grid-single-2">"Row 2"</DemoBox></GridItem>
///         </Grid>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "grid",
    preview_label = "Grid",
    preview_icon = icondata::AiAppstoreOutlined,
)]
#[component]
pub fn Grid(
    /// Column count and gap configuration.
    #[prop(optional, into)]
    config: GridConfig,
    /// Optional CSS class merged onto the grid container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Grid cell children — wrap each in [`GridItem`].
    children: Children,
) -> impl IntoView {
    view! {
        <BaseGrid
            class=class
            cols=Signal::from(config.cols)
            x_gap=Signal::from(config.x_gap)
            y_gap=Signal::from(config.y_gap)
        >
            {children()}
        </BaseGrid>
    }
}
