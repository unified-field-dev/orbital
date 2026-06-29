use orbital_data::DataRecord;
use orbital_paging::{
    FilterLogicWire, FilterQuery, FilterRuleParam, Page, PageRequest, SortDirectionWire, SortParam,
};

use crate::engine::{filter_by_rules, filter_rows, sort_rows_multi};
use crate::types::{
    DataTableColumnDef, DataTableFilter, DataTableRowModel, DataTableSort, FilterLogic,
    FilterOperator, FilterRule,
};

/// Encode [`FilterOperator`] as a snake_case wire string.
pub fn operator_to_wire(operator: FilterOperator) -> &'static str {
    match operator {
        FilterOperator::IsEmpty => "is_empty",
        FilterOperator::IsNotEmpty => "is_not_empty",
        FilterOperator::Contains => "contains",
        FilterOperator::NotContains => "not_contains",
        FilterOperator::Equals => "equals",
        FilterOperator::NotEquals => "not_equals",
        FilterOperator::StartsWith => "starts_with",
        FilterOperator::EndsWith => "ends_with",
        FilterOperator::GreaterThan => "greater_than",
        FilterOperator::GreaterThanOrEqual => "greater_than_or_equal",
        FilterOperator::LessThan => "less_than",
        FilterOperator::LessThanOrEqual => "less_than_or_equal",
        FilterOperator::Is => "is",
        FilterOperator::IsNot => "is_not",
    }
}

/// Decode a wire operator string into [`FilterOperator`].
pub fn operator_from_wire(value: &str) -> Option<FilterOperator> {
    match value {
        "is_empty" => Some(FilterOperator::IsEmpty),
        "is_not_empty" => Some(FilterOperator::IsNotEmpty),
        "contains" => Some(FilterOperator::Contains),
        "not_contains" => Some(FilterOperator::NotContains),
        "equals" => Some(FilterOperator::Equals),
        "not_equals" => Some(FilterOperator::NotEquals),
        "starts_with" => Some(FilterOperator::StartsWith),
        "ends_with" => Some(FilterOperator::EndsWith),
        "greater_than" => Some(FilterOperator::GreaterThan),
        "greater_than_or_equal" => Some(FilterOperator::GreaterThanOrEqual),
        "less_than" => Some(FilterOperator::LessThan),
        "less_than_or_equal" => Some(FilterOperator::LessThanOrEqual),
        "is" => Some(FilterOperator::Is),
        "is_not" => Some(FilterOperator::IsNot),
        _ => None,
    }
}

fn sort_to_wire(sort: &DataTableSort) -> Option<Vec<SortParam>> {
    if sort.items.is_empty() {
        return None;
    }
    Some(
        sort.items
            .iter()
            .map(|rule| SortParam {
                field: rule.field.clone(),
                direction: match rule.direction {
                    crate::engine::SortDirection::Asc => SortDirectionWire::Asc,
                    crate::engine::SortDirection::Desc => SortDirectionWire::Desc,
                },
            })
            .collect(),
    )
}

fn filter_to_wire(filter: &DataTableFilter) -> Option<FilterQuery> {
    if filter.items.is_empty() {
        return None;
    }
    Some(FilterQuery {
        items: filter
            .items
            .iter()
            .map(|rule| FilterRuleParam {
                field: rule.field.clone(),
                operator: operator_to_wire(rule.operator).to_string(),
                value: rule.value.clone(),
            })
            .collect(),
        logic: match filter.logic {
            FilterLogic::And => FilterLogicWire::And,
            FilterLogic::Or => FilterLogicWire::Or,
        },
    })
}

fn sort_from_wire(params: &[SortParam]) -> DataTableSort {
    DataTableSort {
        items: params
            .iter()
            .map(|param| crate::types::SortRule {
                field: param.field.clone(),
                direction: match param.direction {
                    SortDirectionWire::Asc => crate::engine::SortDirection::Asc,
                    SortDirectionWire::Desc => crate::engine::SortDirection::Desc,
                },
            })
            .collect(),
    }
}

fn filter_from_wire(query: &FilterQuery) -> DataTableFilter {
    DataTableFilter {
        items: query
            .items
            .iter()
            .filter_map(|param| {
                operator_from_wire(&param.operator).map(|operator| FilterRule {
                    field: param.field.clone(),
                    operator,
                    value: param.value.clone(),
                })
            })
            .collect(),
        logic: match query.logic {
            FilterLogicWire::And => FilterLogic::And,
            FilterLogicWire::Or => FilterLogic::Or,
        },
    }
}

/// Build a [`PageRequest`] from reactive table state.
pub fn build_page_request(
    sort: &DataTableSort,
    filter: &DataTableFilter,
    quick_search: &str,
    offset: u32,
    limit: u32,
) -> PageRequest {
    let quick = quick_search.trim();
    PageRequest::with_query(
        offset,
        limit,
        sort_to_wire(sort),
        filter_to_wire(filter),
        if quick.is_empty() {
            None
        } else {
            Some(quick.to_string())
        },
    )
}

/// Apply server query parameters to in-memory rows (mock backends + tests).
pub fn process_server_rows(
    rows: Vec<DataTableRowModel>,
    columns: &[DataTableColumnDef],
    request: &PageRequest,
) -> (Vec<DataTableRowModel>, usize) {
    let quick_search = request.quick_search.as_deref().unwrap_or("");
    let quick_filtered = filter_rows(&rows, columns, quick_search);
    let filter = request
        .filter
        .as_ref()
        .map(filter_from_wire)
        .unwrap_or_default();
    let filtered = filter_by_rules(&quick_filtered, columns, &filter);
    let sort = request
        .sort
        .as_ref()
        .map(|params| sort_from_wire(params))
        .unwrap_or_default();
    let sorted = sort_rows_multi(filtered, columns, &sort);
    let total = sorted.len();
    (sorted, total)
}

/// Slice processed rows into a [`Page`] for the given request.
pub fn page_from_processed(
    processed: Vec<DataTableRowModel>,
    total: usize,
    request: &PageRequest,
) -> Page<DataRecord> {
    let start = request.offset as usize;
    let end = ((request.offset + request.limit + 1) as usize).min(processed.len());
    let slice: Vec<DataRecord> = if start < processed.len() {
        processed[start..end]
            .iter()
            .map(|row| row.record.clone())
            .collect()
    } else {
        Vec::new()
    };
    let total_count = if request.is_first_page() {
        Some(total as u64)
    } else {
        None
    };
    Page::from_oversized(slice, request.limit, total_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use orbital_data::DataValue;

    #[test]
    fn round_trip_operator_wire() {
        for op in [
            FilterOperator::Contains,
            FilterOperator::GreaterThan,
            FilterOperator::IsEmpty,
        ] {
            let wire = operator_to_wire(op);
            assert_eq!(operator_from_wire(wire), Some(op));
        }
    }

    #[test]
    fn build_page_request_omits_empty_query() {
        let req = build_page_request(
            &DataTableSort::default(),
            &DataTableFilter::default(),
            "  ",
            0,
            10,
        );
        assert!(req.sort.is_none());
        assert!(req.filter.is_none());
        assert!(req.quick_search.is_none());
    }

    #[test]
    fn process_server_rows_filters_and_sorts() {
        let rows = vec![
            DataTableRowModel::from_text_cells(
                "1",
                HashMap::from([
                    ("name".into(), "User 1".into()),
                    ("role".into(), "Admin".into()),
                ]),
            ),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([
                    ("name".into(), "User 2".into()),
                    ("role".into(), "Member".into()),
                ]),
            ),
        ];
        let columns = vec![
            DataTableColumnDef::new("name", "Name"),
            DataTableColumnDef::new("role", "Role"),
        ];
        let request = PageRequest::with_query(
            0,
            10,
            Some(vec![SortParam {
                field: "name".into(),
                direction: SortDirectionWire::Desc,
            }]),
            Some(FilterQuery {
                items: vec![FilterRuleParam {
                    field: "role".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("Admin".into()),
                }],
                logic: FilterLogicWire::And,
            }),
            None,
        );
        let (processed, total) = process_server_rows(rows, &columns, &request);
        assert_eq!(total, 1);
        assert_eq!(processed[0].id(), "1");
    }
}
