use leptos::prelude::*;
use orbital_base_components::BaseTable;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::table_styles;

/// Semantic table shell for static or server-rendered data rows and columns.
///
/// Compose explicit header, body, row, and cell parts you control. Reach for [`DataTable`](orbital_datatable::DataTable) when you need built-in sort, filter, row selection, pagination, or virtualization — it renders through these parts internally.
///
/// # When to use
///
/// - Static or lightly interactive tabular content with full control over markup - Small-to-medium row counts, server-rendered lists, summary/report tables - Paginated lists and admin grids where you own sorting/filtering in app code
///
/// # When to use DataTable instead
///
/// Reach for [`DataTable`](orbital_datatable::DataTable) when you need a built-in data engine: client-side sort, filter, row selection, pagination, or feature flags such as virtualization and column pinning. [`DataTable`] renders through these `Table` parts internally — they are not competing engines. **Rule of thumb:** start with `Table`; adopt `DataTable` only when interactive data features justify the heavier API.
///
/// # Usage
///
/// 1. Wrap everything in `Table`. 2. Add a [`TableHeader`] with one [`TableRow`] of [`TableHeaderCell`] labels. 3. Add a [`TableBody`] with one [`TableRow`] per data record. 4. Wrap cell content in [`TableCell`] → [`TableCellLayout`] for truncation.
///
/// # Best Practices
///
/// ## Do's
///
/// * Compose header, body, rows, and cells explicitly * Use [`TableCellLayout`] with `truncate=true` in narrow columns * Keep header labels in [`TableHeaderCell`], data in [`TableCell`]
///
/// ## Don'ts
///
/// * Do not use tables for page layout — prefer [`crate::Flex`] or [`crate::Grid`] * Do not skip [`TableHeader`] when columns have semantic meaning
///
/// # Examples
///
/// ## Simple semantic table
/// Minimal header plus one body row—the starting point before adding pagination or row actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout};
/// view! {
///     <div data-testid="table-preview">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell>"Name"</TableHeaderCell>
///                     <TableHeaderCell>"Role"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout><span data-testid="table-cell-name">"Ada"</span></TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout><span data-testid="table-cell-role">"Admin"</span></TableCellLayout></TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Full compound (3 columns, 2 rows)
/// Full compound structure with header labels, body rows, cells, and layout wrappers for admin grids.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout};
/// view! {
///     <div data-testid="table-compound">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell>"Name"</TableHeaderCell>
///                     <TableHeaderCell>"Role"</TableHeaderCell>
///                     <TableHeaderCell>"Status"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"Ada Lovelace"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"Admin"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"Active"</TableCellLayout></TableCell>
///                 </TableRow>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"Grace Hopper"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"Editor"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"Active"</TableCellLayout></TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Multiple body rows
/// Multiple body rows with a single column—useful for simple ordered lists.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout};
/// view! {
///     <div data-testid="table-rows">
///         <Table>
///             <TableHeader>
///                 <TableRow><TableHeaderCell>"ID"</TableHeaderCell></TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow><TableCell><TableCellLayout><span data-testid="table-row-id-1">"1"</span></TableCellLayout></TableCell></TableRow>
///                 <TableRow><TableCell><TableCellLayout><span data-testid="table-row-id-2">"2"</span></TableCellLayout></TableCell></TableRow>
///                 <TableRow><TableCell><TableCellLayout><span data-testid="table-row-id-3">"3"</span></TableCellLayout></TableCell></TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Truncated long text
/// `truncate=true` ellipsizes long text in narrow columns without breaking table layout.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableBody, TableRow, TableCell, TableCellLayout, TableCellLayoutConfig};
/// view! {
///     <div data-testid="table-truncate" style="max-width: 200px;">
///         <Table>
///             <TableBody>
///                 <TableRow>
///                     <TableCell>
///                         <TableCellLayout config=TableCellLayoutConfig { truncate: true }>
///                             <span data-testid="table-truncate-text">"Long description that truncates in narrow columns"</span>
///                         </TableCellLayout>
///                     </TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Resizable columns
/// Drag header edges to resize columns; min/max width keep columns within a usable range.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout, TableHeaderCellConfig};
/// view! {
///     <div data-testid="table-resizable">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell config=TableHeaderCellConfig::resizable(80.0, 240.0)>
///                         "Name"
///                     </TableHeaderCell>
///                     <TableHeaderCell config=TableHeaderCellConfig::resizable(60.0, 180.0)>
///                         "Role"
///                     </TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"Ada Lovelace"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"Admin"</TableCellLayout></TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Theme tokens
/// Header borders and row dividers inherit stroke tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout};
/// view! {
///     <div data-testid="table-theme">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell>"Column"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"Themed row"</TableCellLayout></TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Status badges in cells
/// Compose [`Badge`](crate::Badge) inside [`TableCellLayout`] for status columns.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout, Badge, BadgeAppearance, BadgeColor};
/// view! {
///     <div data-testid="table-badges">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell>"Name"</TableHeaderCell>
///                     <TableHeaderCell>"Status"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"Ada"</TableCellLayout></TableCell>
///                     <TableCell>
///                         <TableCellLayout>
///                             <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Success>
///                                 <span data-testid="table-badge-active">"Active"</span>
///                             </Badge>
///                         </TableCellLayout>
///                     </TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Wide resizable admin grid
/// Three resizable columns with distinct min/max constraints for dense admin tables.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout, TableHeaderCellConfig};
/// view! {
///     <div data-testid="table-resizable-wide">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell config=TableHeaderCellConfig::resizable(120.0, 300.0)>"A"</TableHeaderCell>
///                     <TableHeaderCell config=TableHeaderCellConfig::resizable(120.0, 300.0)>"B"</TableHeaderCell>
///                     <TableHeaderCell config=TableHeaderCellConfig::resizable(120.0, 300.0)>"C"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"1"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"2"</TableCellLayout></TableCell>
///                     <TableCell><TableCellLayout>"3"</TableCellLayout></TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
///
/// ## Row actions with Button
/// Interactive controls inside cells for edit/delete row actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Table, TableHeader, TableHeaderCell, TableBody, TableRow, TableCell, TableCellLayout, Button, ButtonAppearance};
/// view! {
///     <div data-testid="table-actions">
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell>"Name"</TableHeaderCell>
///                     <TableHeaderCell>"Actions"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <TableRow>
///                     <TableCell><TableCellLayout>"Ada"</TableCellLayout></TableCell>
///                     <TableCell>
///                         <TableCellLayout>
///                             <span data-testid="table-action-edit"><Button appearance=ButtonAppearance::Subtle>"Edit"</Button></span>
///                         </TableCellLayout>
///                     </TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "table",
    preview_label = "Table",
    preview_icon = icondata::AiTableOutlined,
)]
#[component]
pub fn Table(
    /// Optional CSS class merged onto the `<table>` element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Table sections: [`TableHeader`] and/or [`TableBody`] with nested rows and cells.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-table", table_styles());

    view! {
        <BaseTable class=class>
            {children()}
        </BaseTable>
    }
}
