use std::collections::{HashMap, HashSet};

use orbital_data::{DataRecord, DataSchema, DataType, DataValue, Dataset, FieldDef};

use crate::engine::aggregation::aggregate_values;
use crate::types::{
    AggregationFn, ColumnType, DataTableColumnDef, DataTablePivotModel, DataTableRowModel, GetRowId,
};

/// Result of a pivot transform.
#[derive(Clone, Debug, Default)]
pub struct PivotResult {
    pub rows: Vec<DataTableRowModel>,
    pub columns: Vec<DataTableColumnDef>,
}

fn dimension_key(fields: &[String], record: &DataRecord) -> String {
    fields
        .iter()
        .map(|f| {
            record
                .get(f)
                .map(|v| v.display_string())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn pivot_column_field(col_key: &str, value_field: &str) -> String {
    format!("pivot_{col_key}__{value_field}")
}

/// Pivot flat rows into a cross-tab with dynamic columns.
pub fn pivot_rows(
    rows: Vec<DataTableRowModel>,
    pivot: &DataTablePivotModel,
    source_columns: &[DataTableColumnDef],
    _get_row_id: Option<&GetRowId>,
) -> PivotResult {
    if !pivot.is_active() {
        return PivotResult {
            rows,
            columns: source_columns.to_vec(),
        };
    }

    let data_rows: Vec<_> = rows.into_iter().filter(|r| r.is_data_row()).collect();

    let mut col_keys: Vec<String> = Vec::new();
    let mut col_key_set: HashSet<String> = HashSet::new();
    let mut buckets: HashMap<(String, String), Vec<DataTableRowModel>> = HashMap::new();

    for row in &data_rows {
        let row_key = dimension_key(&pivot.row_fields, &row.record);
        let col_key = if pivot.column_fields.is_empty() {
            "total".to_string()
        } else {
            dimension_key(&pivot.column_fields, &row.record)
        };
        if col_key_set.insert(col_key.clone()) {
            col_keys.push(col_key.clone());
        }
        buckets
            .entry((row_key.clone(), col_key))
            .or_default()
            .push(row.clone());
    }

    col_keys.sort();

    let mut dynamic_columns: Vec<DataTableColumnDef> = Vec::new();
    for row_field in &pivot.row_fields {
        if let Some(col) = source_columns.iter().find(|c| c.field == *row_field) {
            dynamic_columns.push(col.clone());
        } else {
            dynamic_columns.push(DataTableColumnDef::new(row_field, row_field));
        }
    }
    for col_key in &col_keys {
        for value_field in &pivot.value_fields {
            let field = pivot_column_field(col_key, value_field);
            let header = if col_key == "total" {
                format!("{} ({})", value_field, pivot.value_fn_label())
            } else {
                format!("{col_key} — {value_field}")
            };
            dynamic_columns
                .push(DataTableColumnDef::new(field, header).with_col_type(ColumnType::Number));
        }
    }

    let mut row_keys: Vec<String> = buckets
        .keys()
        .map(|(rk, _)| rk.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    row_keys.sort();

    let mut pivoted_rows = Vec::new();
    for row_key in row_keys {
        let mut values: HashMap<String, DataValue> = HashMap::new();
        if !pivot.row_fields.is_empty() {
            let parts: Vec<&str> = row_key.split('|').collect();
            for (i, field) in pivot.row_fields.iter().enumerate() {
                if let Some(part) = parts.get(i) {
                    values.insert(field.clone(), DataValue::Text(part.to_string()));
                }
            }
        }
        for col_key in &col_keys {
            for value_field in &pivot.value_fields {
                let members = buckets
                    .get(&(row_key.clone(), col_key.clone()))
                    .cloned()
                    .unwrap_or_default();
                let nums: Vec<DataValue> = members
                    .iter()
                    .filter_map(|r| r.get(value_field).cloned())
                    .collect();
                let agg = aggregate_values(&nums, pivot.value_fn);
                values.insert(pivot_column_field(col_key, value_field), agg);
            }
        }
        let id = format!("pivot:{row_key}");
        pivoted_rows.push(DataTableRowModel::new(DataRecord::new(id, values)));
    }

    PivotResult {
        rows: pivoted_rows,
        columns: dynamic_columns,
    }
}

/// Build chart-ready dataset from pivot result.
pub fn pivot_dataset(result: &PivotResult) -> Dataset {
    let fields: Vec<FieldDef> = result
        .columns
        .iter()
        .map(|c| FieldDef {
            key: c.field.clone(),
            label: c.header_name.clone(),
            data_type: match c.col_type {
                ColumnType::Number => DataType::Number,
                ColumnType::Boolean => DataType::Bool,
                ColumnType::Date => DataType::Date,
                _ => DataType::Text,
            },
        })
        .collect();
    let schema = DataSchema::new(fields);
    let records: Vec<DataRecord> = result.rows.iter().map(|r| r.record.clone()).collect();
    Dataset::from_records(schema, records)
}

trait PivotValueFnLabel {
    fn value_fn_label(&self) -> &'static str;
}

impl PivotValueFnLabel for DataTablePivotModel {
    fn value_fn_label(&self) -> &'static str {
        match self.value_fn {
            AggregationFn::Sum => "sum",
            AggregationFn::Avg => "avg",
            AggregationFn::Min => "min",
            AggregationFn::Max => "max",
            AggregationFn::Count => "count",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::{DataRecord, DataValue};
    use std::collections::HashMap;

    #[test]
    fn pivot_produces_dynamic_columns() {
        let rows = vec![
            DataTableRowModel::new(DataRecord::new(
                "1",
                HashMap::from([
                    ("region".into(), DataValue::Text("East".into())),
                    ("amount".into(), DataValue::Number(100.0)),
                ]),
            )),
            DataTableRowModel::new(DataRecord::new(
                "2",
                HashMap::from([
                    ("region".into(), DataValue::Text("West".into())),
                    ("amount".into(), DataValue::Number(200.0)),
                ]),
            )),
        ];
        let pivot = DataTablePivotModel {
            row_fields: vec!["region".into()],
            column_fields: vec![],
            value_fields: vec!["amount".into()],
            value_fn: AggregationFn::Sum,
        };
        let cols = vec![
            DataTableColumnDef::new("region", "Region"),
            DataTableColumnDef::new("amount", "Amount").with_col_type(ColumnType::Number),
        ];
        let result = pivot_rows(rows, &pivot, &cols, None);
        assert_eq!(result.rows.len(), 2);
        assert!(result.columns.len() >= 2);
    }
}
