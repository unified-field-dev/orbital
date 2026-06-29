use std::collections::HashSet;

use orbital_data::Dataset;

use crate::engine::aggregation::build_aggregate_records;
use crate::engine::apply_row_order;
use crate::engine::build_export_dataset;
use crate::engine::filter_by_rules;
use crate::engine::filter_rows;
use crate::engine::grouping::build_group_rows;
use crate::engine::paginate_rows;
use crate::engine::pivot::{pivot_dataset, pivot_rows, PivotResult};
use crate::engine::sort_rows_multi;
use crate::engine::tree::visible_tree_rows;
use crate::types::{
    AggregationModel, AggregationPosition, DataTableColumnDef, DataTableFeatures,
    DataTablePivotModel, DataTableRowGrouping, DataTableRowKind, DataTableRowModel, GetRowId,
    GetTreePath, PagingMode,
};

/// Input to the unified processed pipeline.
#[derive(Clone)]
pub struct ProcessedPipelineInput<'a> {
    pub rows: &'a [DataTableRowModel],
    pub all_columns: &'a [DataTableColumnDef],
    pub visible_columns: &'a [DataTableColumnDef],
    pub quick_search: &'a str,
    pub filter: &'a crate::types::DataTableFilter,
    pub sort: &'a crate::types::DataTableSort,
    pub features: DataTableFeatures,
    pub row_order: &'a [String],
    pub get_row_id: Option<&'a GetRowId>,
    pub get_tree_path: Option<&'a GetTreePath>,
    pub expanded_tree_nodes: &'a HashSet<String>,
    pub row_grouping: &'a DataTableRowGrouping,
    pub expanded_groups: &'a HashSet<String>,
    pub aggregation: &'a AggregationModel,
    pub aggregation_position: AggregationPosition,
    pub pivot: &'a DataTablePivotModel,
    pub paging: PagingMode,
    pub page: usize,
    pub page_size: usize,
}

/// Output of the unified processed pipeline.
#[derive(Clone, Debug)]
pub struct ProcessedPipelineResult {
    pub display_rows: Vec<DataTableRowModel>,
    pub all_matching_rows: Vec<DataTableRowModel>,
    pub footer_row: Option<DataTableRowModel>,
    pub pivot_result: Option<PivotResult>,
    pub active_columns: Vec<DataTableColumnDef>,
}

/// Run filter → sort → tree → group → pivot → reorder → paginate.
pub fn run_processed_pipeline(input: ProcessedPipelineInput<'_>) -> ProcessedPipelineResult {
    let quick_filtered = filter_rows(input.rows, input.all_columns, input.quick_search);
    let filtered = filter_by_rules(&quick_filtered, input.all_columns, input.filter);
    let mut sorted = sort_rows_multi(filtered, input.visible_columns, input.sort);

    if input.features.contains(DataTableFeatures::TREE_DATA) {
        if let Some(get_tree_path) = input.get_tree_path {
            sorted = visible_tree_rows(
                sorted,
                get_tree_path,
                input.get_row_id,
                input.expanded_tree_nodes,
            );
        }
    }

    let pivot_active =
        input.features.contains(DataTableFeatures::PIVOTING) && input.pivot.is_active();
    let grouping_active =
        input.features.contains(DataTableFeatures::ROW_GROUPING) && input.row_grouping.is_active();

    let (mut display_rows, active_columns, pivot_result) = if pivot_active {
        let pivot_result = pivot_rows(
            sorted.clone(),
            input.pivot,
            input.all_columns,
            input.get_row_id,
        );
        (
            pivot_result.rows.clone(),
            pivot_result.columns.clone(),
            Some(pivot_result),
        )
    } else if grouping_active {
        let grouped = build_group_rows(
            sorted.clone(),
            input.row_grouping,
            input.expanded_groups,
            input.all_columns,
            input.get_row_id,
            if input.features.contains(DataTableFeatures::AGGREGATION) {
                Some(input.aggregation)
            } else {
                None
            },
            input.aggregation_position,
        );
        (grouped, input.all_columns.to_vec(), None)
    } else {
        (sorted.clone(), input.all_columns.to_vec(), None)
    };

    let footer_row = if input.features.contains(DataTableFeatures::AGGREGATION)
        && input.aggregation.is_active()
        && input.aggregation_position == AggregationPosition::Footer
        && !pivot_active
    {
        let data_rows: Vec<_> = sorted.iter().filter(|r| r.is_data_row()).cloned().collect();
        crate::engine::aggregation::build_footer_row(
            &data_rows,
            input.aggregation,
            input.all_columns,
            input.get_row_id,
        )
    } else {
        None
    };

    let all_matching_rows = display_rows.clone();

    if input.features.contains(DataTableFeatures::ROW_REORDER) {
        display_rows = apply_row_order(display_rows, input.row_order, input.get_row_id);
    }

    let display_rows = match input.paging {
        PagingMode::Paged => paginate_rows(&display_rows, input.page, input.page_size),
        PagingMode::None | PagingMode::InfiniteScroll => display_rows,
    };

    ProcessedPipelineResult {
        display_rows,
        all_matching_rows,
        footer_row,
        pivot_result,
        active_columns,
    }
}

/// Build chart-ready dataset from pipeline result (pre-pagination rows).
pub fn build_processed_dataset(
    result: &ProcessedPipelineResult,
    source_columns: &[DataTableColumnDef],
    aggregation: &AggregationModel,
    pivot_active: bool,
    get_row_id: Option<&GetRowId>,
) -> Dataset {
    if pivot_active {
        if let Some(pivot_result) = &result.pivot_result {
            return pivot_dataset(pivot_result);
        }
    }

    let data_rows: Vec<_> = result
        .all_matching_rows
        .iter()
        .filter(|r| matches!(r.kind, DataTableRowKind::Data))
        .cloned()
        .collect();

    let mut dataset = build_export_dataset(&data_rows, source_columns);

    let agg_records = build_aggregate_records(&data_rows, aggregation, source_columns, get_row_id);
    dataset.records.extend(agg_records);

    dataset
}
