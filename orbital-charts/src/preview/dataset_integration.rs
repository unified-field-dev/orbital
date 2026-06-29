//! Preview demonstrating live charts consumption of a DataTable processed [`Dataset`].

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Live binding between [`orbital_datatable::DataTable`] and [`crate::BarChart`] via
/// [`orbital_datatable::ChartBinding`] context.
///
/// # When to use
///
/// - Dashboard slots where a chart must redraw when table sort, filter, or grouping changes.
/// - Sibling chart panels placed inside the table subtree (context binding).
/// - Imperative one-shot reads via [`orbital_datatable::DataTableHandle::get_processed_dataset`]
///   when the chart lives outside the table.
///
/// # Usage
///
/// 1. Enable [`orbital_datatable::DataTableFeatures::CHARTS_INTEGRATION`] on [`orbital_datatable::DataTable`].
/// 2. Render a chart child that calls [`orbital_datatable::use_chart_binding`] and reads
///    `binding.dataset.get()` reactively.
/// 3. Build [`orbital_data::ChartFieldBinding`] from `binding.suggested_field_binding()` or explicit keys.
/// 4. For out-of-tree charts, capture the handle with `on_handle` and call `get_processed_dataset`.
///
/// For static dataset binding without a live table, see the Bar Chart **Dataset binding** example.
/// For the datatable-side API, see **Charts Integration** under Data Table docs.
///
/// # Examples
///
/// ## Live table and chart binding
/// Enable `CHARTS_INTEGRATION`, then read the live processed [`orbital_data::Dataset`] from a chart
/// child via [`orbital_datatable::use_chart_binding`]. Filter and sort updates propagate to the chart.
/// <!-- default -->
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use leptos::prelude::*;
/// use orbital_data::{DataRecord, DataValue};
/// use orbital_datatable::{
///     ColumnType, DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode,
///     use_chart_binding,
/// };
/// use crate::BarChart;
/// #[component]
/// fn ChartsIntegrationBoundChart() -> impl IntoView {
///     let binding = use_chart_binding().expect("ChartBinding when CHARTS_INTEGRATION is set");
///     view! {
///         <div data-testid="charts-integration-chart">
///             <span data-testid="charts-integration-bar-count">
///                 {move || binding.dataset.get().records.len()}
///             </span>
///             {move || {
///                 let dataset = binding.dataset.get();
///                 let Some(field_binding) = binding.suggested_field_binding() else {
///                     return view! { <div data-testid="charts-integration-chart-empty" /> }.into_any();
///                 };
///                 view! {
///                     <BarChart
///                         dataset=dataset
///                         binding=field_binding
///                         width=520.0
///                         height=320.0
///                     />
///                 }
///                 .into_any()
///             }}
///         </div>
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
///     DataTableRowModel::new(DataRecord::new(
///         "3",
///         HashMap::from([
///             ("name".into(), DataValue::Text("Gamma".into())),
///             ("score".into(), DataValue::Number(50.0)),
///         ]),
///     )),
///     DataTableRowModel::new(DataRecord::new(
///         "4",
///         HashMap::from([
///             ("name".into(), DataValue::Text("Delta".into())),
///             ("score".into(), DataValue::Number(70.0)),
///         ]),
///     )),
/// ]);
/// view! {
///     <div data-testid="data-table-charts-integration-preview">
///         <DataTable
///             features=DataTableFeatures::CHARTS_INTEGRATION
///             paging=PagingMode::None
///             page_size_options=None
///             sortable=true
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("score", "Score").with_col_type(ColumnType::Number),
///             ]
///             items=items
///         >
///             <ChartsIntegrationBoundChart />
///         </DataTable>
///     </div>
/// }
/// ```
///
/// ## Handle-based processed dataset
/// When the chart renders outside the table subtree, capture [`orbital_datatable::DataTableHandle`]
/// and call `get_processed_dataset` for a point-in-time snapshot.
/// <!-- code-only -->
/// ```rust,ignore
/// use leptos::prelude::*;
/// use orbital_datatable::{DataTable, DataTableFeatures, DataTableHandle};
/// let handle = RwSignal::new(None::<DataTableHandle>);
/// view! {
///     <DataTable
///         features=DataTableFeatures::CHARTS_INTEGRATION
///         on_handle=Callback::new(move |h| handle.set(Some(h)))
///         // columns, items, …
///     />
/// }
/// // elsewhere:
/// // let dataset = handle.get().unwrap().get_processed_dataset.run(());
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "data-table-charts-integration",
    preview_label = "Dataset Integration",
    preview_icon = icondata::AiLineChartOutlined,
)]
#[component]
pub fn DatasetIntegration() -> impl IntoView {
    view! { () }
}
