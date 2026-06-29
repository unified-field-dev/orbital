use orbital_data::DataValue;

use crate::engine::column::resolve_value;
use crate::types::{
    ColumnType, DataTableColumnDef, DataTableFilter, DataTableRowModel, FilterLogic, FilterOperator,
};

pub fn apply_operator(
    operator: FilterOperator,
    cell: &DataValue,
    filter_value: &DataValue,
    col_type: ColumnType,
) -> bool {
    match operator {
        FilterOperator::IsEmpty => cell.is_empty_value(),
        FilterOperator::IsNotEmpty => !cell.is_empty_value(),
        FilterOperator::Contains => cell.filter_text().contains(&filter_value.filter_text()),
        FilterOperator::NotContains => !cell.filter_text().contains(&filter_value.filter_text()),
        FilterOperator::Equals => values_equal(cell, filter_value, col_type),
        FilterOperator::NotEquals => !values_equal(cell, filter_value, col_type),
        FilterOperator::StartsWith => cell.filter_text().starts_with(&filter_value.filter_text()),
        FilterOperator::EndsWith => cell.filter_text().ends_with(&filter_value.filter_text()),
        FilterOperator::GreaterThan => compare_ordering(cell, filter_value, col_type)
            .map(|o| o == std::cmp::Ordering::Greater)
            .unwrap_or(false),
        FilterOperator::GreaterThanOrEqual => compare_ordering(cell, filter_value, col_type)
            .map(|o| o != std::cmp::Ordering::Less)
            .unwrap_or(false),
        FilterOperator::LessThan => compare_ordering(cell, filter_value, col_type)
            .map(|o| o == std::cmp::Ordering::Less)
            .unwrap_or(false),
        FilterOperator::LessThanOrEqual => compare_ordering(cell, filter_value, col_type)
            .map(|o| o != std::cmp::Ordering::Greater)
            .unwrap_or(false),
        FilterOperator::Is => values_equal(cell, filter_value, col_type),
        FilterOperator::IsNot => !values_equal(cell, filter_value, col_type),
    }
}

fn values_equal(cell: &DataValue, filter_value: &DataValue, col_type: ColumnType) -> bool {
    match col_type {
        ColumnType::Number => {
            matches!((cell, filter_value), (DataValue::Number(a), DataValue::Number(b)) if (a - b).abs() < f64::EPSILON)
        }
        ColumnType::Boolean => {
            matches!((cell, filter_value), (DataValue::Bool(a), DataValue::Bool(b)) if a == b)
        }
        ColumnType::Date => {
            matches!((cell, filter_value), (DataValue::Date(a), DataValue::Date(b)) if a == b)
        }
        _ => cell.filter_text() == filter_value.filter_text(),
    }
}

fn compare_ordering(
    cell: &DataValue,
    filter_value: &DataValue,
    col_type: ColumnType,
) -> Option<std::cmp::Ordering> {
    use crate::engine::compare::compare_hint;
    cell.partial_cmp_typed(filter_value, compare_hint(col_type))
}

fn row_matches_token(row: &DataTableRowModel, columns: &[DataTableColumnDef], token: &str) -> bool {
    let needle = DataValue::Text(token.to_string());
    columns.iter().any(|col| {
        if !col.filterable || col.col_type == ColumnType::Actions {
            return false;
        }
        let cell = resolve_value(col, row);
        apply_operator(FilterOperator::Contains, &cell, &needle, col.col_type)
    })
}

/// Quick-search filter: all whitespace tokens must match (any filterable column per token).
pub fn filter_rows(
    rows: &[DataTableRowModel],
    columns: &[DataTableColumnDef],
    quick_search: &str,
) -> Vec<DataTableRowModel> {
    let tokens: Vec<&str> = quick_search
        .split_whitespace()
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.is_empty() {
        return rows.to_vec();
    }
    rows.iter()
        .filter(|row| {
            tokens
                .iter()
                .all(|token| row_matches_token(row, columns, token))
        })
        .cloned()
        .collect()
}

/// Apply structured filter rules from the column menu and filter panel.
pub fn filter_by_rules(
    rows: &[DataTableRowModel],
    columns: &[DataTableColumnDef],
    filter: &DataTableFilter,
) -> Vec<DataTableRowModel> {
    if filter.items.is_empty() {
        return rows.to_vec();
    }
    rows.iter()
        .filter(|row| {
            let results: Vec<bool> = filter
                .items
                .iter()
                .filter_map(|rule| {
                    let col = columns.iter().find(|c| c.field == rule.field)?;
                    let cell = resolve_value(col, row);
                    Some(apply_operator(
                        rule.operator,
                        &cell,
                        &rule.value,
                        col.col_type,
                    ))
                })
                .collect();
            match filter.logic {
                FilterLogic::And => results.iter().all(|&r| r),
                FilterLogic::Or => results.iter().any(|&r| r),
            }
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn quick_search_all_tokens() {
        let rows = vec![
            DataTableRowModel::from_text_cells(
                "1",
                HashMap::from([("name".into(), "Ada Lovelace".into())]),
            ),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([("name".into(), "Grace Hopper".into())]),
            ),
        ];
        let columns = vec![DataTableColumnDef::new("name", "Name")];
        let filtered = filter_rows(&rows, &columns, "Ada");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id(), "1");

        let filtered = filter_rows(&rows, &columns, "Grace Ada");
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn contains_case_insensitive() {
        assert!(apply_operator(
            FilterOperator::Contains,
            &DataValue::Text("Hello World".into()),
            &DataValue::Text("hello".into()),
            ColumnType::Text,
        ));
    }
}
