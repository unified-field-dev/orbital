use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::engine::column::resolve_value;
use crate::types::{
    AggregationFn, AggregationModel, DataTableColumnDef, DataTableRowKind, DataTableRowModel,
    GetRowId,
};

/// Extract numeric/text values for a field from data rows.
pub fn values_for_field(
    rows: &[DataTableRowModel],
    field: &str,
    columns: &[DataTableColumnDef],
    _get_row_id: Option<&GetRowId>,
) -> Vec<DataValue> {
    rows.iter()
        .filter(|r| r.is_data_row())
        .map(|row| {
            columns
                .iter()
                .find(|c| c.field == field)
                .map(|c| resolve_value(c, row))
                .unwrap_or(DataValue::Null)
        })
        .collect()
}

/// Compute a single aggregate over values.
pub fn aggregate_values(values: &[DataValue], func: AggregationFn) -> DataValue {
    match func {
        AggregationFn::Count => DataValue::Number(values.len() as f64),
        AggregationFn::Sum => {
            let sum: f64 = values
                .iter()
                .filter_map(|v| match v {
                    DataValue::Number(n) => Some(*n),
                    _ => None,
                })
                .sum();
            DataValue::Number(sum)
        }
        AggregationFn::Avg => {
            let nums: Vec<f64> = values
                .iter()
                .filter_map(|v| match v {
                    DataValue::Number(n) => Some(*n),
                    _ => None,
                })
                .collect();
            if nums.is_empty() {
                DataValue::Null
            } else {
                DataValue::Number(nums.iter().sum::<f64>() / nums.len() as f64)
            }
        }
        AggregationFn::Min => values
            .iter()
            .filter_map(|v| match v {
                DataValue::Number(n) => Some(*n),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(DataValue::Number)
            .unwrap_or(DataValue::Null),
        AggregationFn::Max => values
            .iter()
            .filter_map(|v| match v {
                DataValue::Number(n) => Some(*n),
                _ => None,
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(DataValue::Number)
            .unwrap_or(DataValue::Null),
    }
}

pub fn compute_field_aggregate(values: &[DataValue], func: AggregationFn) -> Option<DataValue> {
    if values.is_empty() && func != AggregationFn::Count {
        return None;
    }
    Some(aggregate_values(values, func))
}

/// Build footer aggregate row for visible data rows.
pub fn build_footer_row(
    data_rows: &[DataTableRowModel],
    model: &AggregationModel,
    columns: &[DataTableColumnDef],
    get_row_id: Option<&GetRowId>,
) -> Option<DataTableRowModel> {
    if !model.is_active() {
        return None;
    }
    let mut values = HashMap::new();
    for rule in &model.rules {
        let field_values = values_for_field(data_rows, &rule.field, columns, get_row_id);
        if let Some(value) = compute_field_aggregate(&field_values, rule.func) {
            values.insert(rule.field.clone(), value);
        }
    }
    if values.is_empty() {
        return None;
    }
    Some(DataTableRowModel::with_kind(
        DataTableRowKind::AggregateFooter,
        DataRecord::new("footer:aggregate".to_string(), values),
    ))
}

/// Build synthetic aggregate records for chart export.
pub fn build_aggregate_records(
    data_rows: &[DataTableRowModel],
    model: &AggregationModel,
    columns: &[DataTableColumnDef],
    get_row_id: Option<&GetRowId>,
) -> Vec<DataRecord> {
    if !model.is_active() {
        return Vec::new();
    }
    let footer = build_footer_row(data_rows, model, columns, get_row_id);
    footer.map(|r| r.record).into_iter().collect()
}
