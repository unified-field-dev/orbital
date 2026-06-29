//! Charts integration — live processed [`Dataset`] context for downstream renderers.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Stable data path from [`DataTable`](crate::DataTable) to chart renderers via
/// [`ChartBinding`](crate::ChartBinding) and [`DataTableFeatures::CHARTS_INTEGRATION`](crate::DataTableFeatures::CHARTS_INTEGRATION).
///
/// Datatable does not import `orbital-charts` — consumers subscribe to the shared
/// [`orbital_data::Dataset`] signal and build their own renderer binding.
///
/// # When to use
///
/// - Dashboard slots where charts must redraw on sort, filter, grouping, or pivot changes.
/// - Multi-chart layouts with chart panels as [`DataTable`] children.
/// - Imperative reads when the chart lives outside the table subtree.
///
/// # Usage
///
/// 1. Enable `features=DataTableFeatures::CHARTS_INTEGRATION` on [`DataTable`].
/// 2. In a child component, call [`use_chart_binding`] and read `binding.dataset.get()` reactively.
/// 3. Build [`orbital_data::ChartFieldBinding`] from `binding.suggested_field_binding()` or explicit field keys.
/// 4. For out-of-tree charts, capture [`DataTableHandle`] via `data_table_events.on_handle` and call `get_processed_dataset`.
///
/// Full cross-crate wiring with [`orbital_charts::BarChart`] lives on the
/// **Dataset Integration** charts preview (`/data-table-charts-integration`).
///
/// # Best Practices
///
/// ## Do's
///
/// * Place chart consumers inside the [`DataTable`] subtree for context binding.
/// * Match `x_field` / `y_fields` keys to visible column [`FieldDef`](crate::FieldDef) keys.
/// * Use `get_processed_dataset` for one-shot exports outside the reactive subtree.
///
/// ## Don'ts
///
/// * Do not import `orbital-charts` from data table core — bind the shared [`Dataset`] only.
/// * Do not read processed output when `CHARTS_INTEGRATION` is disabled (returns empty [`Dataset`]).
///
/// # Examples
///
/// ## Live chart binding context
/// Enable `CHARTS_INTEGRATION` and read the reactive processed record count from a table child.
/// Sort and filter changes update `binding.dataset` on every pipeline run.
/// <!-- default -->
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use leptos::prelude::*;
/// use orbital_data::{DataRecord, DataValue};
/// use crate::{
///     ColumnType, DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode,
///     use_chart_binding,
/// };
/// #[component]
/// fn ChartBindingRecordCount() -> impl IntoView {
///     let binding = use_chart_binding().expect("ChartBinding when CHARTS_INTEGRATION is set");
///     view! {
///         <span data-testid="data-table-charts-binding-count">
///             {move || binding.dataset.get().records.len()}
///         </span>
///     }
/// }
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new(
///         "1",
///         HashMap::from([
///             ("name".into(), DataValue::Text("Alpha".into())),
///             ("score".into(), DataValue::Number(10.0)),
///         ]),
///     )),
///     DataTableRowModel::new(DataRecord::new(
///         "2",
///         HashMap::from([
///             ("name".into(), DataValue::Text("Beta".into())),
///             ("score".into(), DataValue::Number(30.0)),
///         ]),
///     )),
/// ]);
/// view! {
///     <div data-testid="data-table-charts-integration-preview">
///         <DataTable
///             features=DataTableFeatures::CHARTS_INTEGRATION
///             paging=PagingMode::None
///             sortable=true
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("score", "Score").with_col_type(ColumnType::Number),
///             ]
///             items=items
///         >
///             <ChartBindingRecordCount />
///         </DataTable>
///     </div>
/// }
/// ```
///
/// ## Bar chart consumer (cross-crate)
/// Wire `orbital-charts` in your app crate. See the live **Dataset Integration** preview for the full table + chart example.
/// <!-- code-only -->
/// ```rust,ignore
/// use leptos::prelude::*;
/// use orbital_charts::BarChart;
/// use orbital_datatable::use_chart_binding;
/// #[component]
/// fn BoundBarChart() -> impl IntoView {
///     let binding = use_chart_binding().expect("ChartBinding");
///     view! {
///         {move || {
///             let dataset = binding.dataset.get();
///             let Some(field_binding) = binding.suggested_field_binding() else {
///                 return ().into_any();
///             };
///             view! {
///                 <BarChart dataset=dataset binding=field_binding width=520.0 height=320.0 />
///             }
///             .into_any()
///         }}
///     }
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 95,
    preview_slug = "data-table-charts-integration",
    preview_label = "Charts Integration",
    preview_icon = icondata::AiLineChartOutlined,
)]
#[component]
pub fn DataTableChartsIntegrationDoc() -> impl IntoView {
    view! { () }
}
