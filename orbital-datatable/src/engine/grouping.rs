use std::collections::HashSet;

use orbital_data::{DataRecord, DataValue};

use crate::engine::aggregation::{compute_field_aggregate, values_for_field};
use crate::engine::column::resolve_value;
use crate::types::{
    AggregationModel, AggregationPosition, DataTableColumnDef, DataTableRowGrouping,
    DataTableRowKind, DataTableRowModel, GetRowId, GroupAggregates,
};

/// Build composite group key for a row at a given depth.
pub fn group_key_at_depth(
    row: &DataTableRowModel,
    grouping: &DataTableRowGrouping,
    depth: usize,
    columns: &[DataTableColumnDef],
) -> String {
    grouping
        .model
        .iter()
        .take(depth + 1)
        .map(|field| {
            let col = columns.iter().find(|c| c.field == *field);
            let value = col
                .map(|c| resolve_value(c, row))
                .unwrap_or(DataValue::Null);
            format!("{field}:{}", value.display_string())
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn group_label(row: &DataTableRowModel, field: &str, columns: &[DataTableColumnDef]) -> String {
    columns
        .iter()
        .find(|c| c.field == field)
        .map(|c| {
            let value = resolve_value(c, row);
            value.display_string()
        })
        .unwrap_or_default()
}

fn make_group_header(
    group_key: String,
    field: String,
    depth: usize,
    child_count: usize,
    label: String,
    aggregates: GroupAggregates,
) -> DataTableRowModel {
    let mut values = aggregates;
    values.insert(
        field.clone(),
        DataValue::Text(format!("{label} ({child_count})")),
    );
    DataTableRowModel::with_kind(
        DataTableRowKind::GroupHeader {
            group_key: group_key.clone(),
            field,
            depth,
            child_count,
            aggregates: values.clone(),
        },
        DataRecord::new(format!("group:{group_key}"), values),
    )
}

fn build_grouped_rows_recursive(
    rows: &[DataTableRowModel],
    grouping: &DataTableRowGrouping,
    depth: usize,
    expanded_groups: &HashSet<String>,
    columns: &[DataTableColumnDef],
    get_row_id: Option<&GetRowId>,
    aggregation: Option<&AggregationModel>,
    aggregation_position: AggregationPosition,
) -> Vec<DataTableRowModel> {
    if depth >= grouping.model.len() {
        return rows.to_vec();
    }

    let field = grouping.model[depth].clone();
    let mut groups: Vec<(String, Vec<DataTableRowModel>)> = Vec::new();
    let mut order: Vec<String> = Vec::new();

    for row in rows {
        let key = group_key_at_depth(row, grouping, depth, columns);
        if let Some((_, members)) = groups.iter_mut().find(|(k, _)| k == &key) {
            members.push(row.clone());
        } else {
            order.push(key.clone());
            groups.push((key, vec![row.clone()]));
        }
    }

    let mut output = Vec::new();
    for key in order {
        let (_, members) = groups.iter().find(|(k, _)| k == &key).unwrap();
        let label = group_label(&members[0], &field, columns);
        let child_count = members.len();
        let mut aggregates = GroupAggregates::new();
        if let Some(model) = aggregation {
            if aggregation_position == AggregationPosition::GroupInline {
                for rule in &model.rules {
                    let values = values_for_field(members, &rule.field, columns, get_row_id);
                    if let Some(value) = compute_field_aggregate(&values, rule.func) {
                        aggregates.insert(rule.field.clone(), value);
                    }
                }
            }
        }
        output.push(make_group_header(
            key.clone(),
            field.clone(),
            depth,
            child_count,
            label,
            aggregates,
        ));
        if expanded_groups.contains(&key) {
            if depth + 1 < grouping.model.len() {
                output.extend(build_grouped_rows_recursive(
                    members,
                    grouping,
                    depth + 1,
                    expanded_groups,
                    columns,
                    get_row_id,
                    aggregation,
                    aggregation_position,
                ));
            } else {
                output.extend(members.iter().cloned());
            }
        }
    }
    output
}

/// Inject group header rows into sorted data rows.
pub fn build_group_rows(
    sorted_data_rows: Vec<DataTableRowModel>,
    grouping: &DataTableRowGrouping,
    expanded_groups: &HashSet<String>,
    columns: &[DataTableColumnDef],
    get_row_id: Option<&GetRowId>,
    aggregation: Option<&AggregationModel>,
    aggregation_position: AggregationPosition,
) -> Vec<DataTableRowModel> {
    if !grouping.is_active() {
        return sorted_data_rows;
    }
    let data_only: Vec<_> = sorted_data_rows
        .into_iter()
        .filter(|r| r.is_data_row())
        .collect();
    build_grouped_rows_recursive(
        &data_only,
        grouping,
        0,
        expanded_groups,
        columns,
        get_row_id,
        aggregation,
        aggregation_position,
    )
}

/// Collect leaf data rows under a group key (for aggregate computation).
pub fn data_rows_in_group<'a>(
    rows: &'a [DataTableRowModel],
    group_key: &str,
) -> Vec<&'a DataTableRowModel> {
    let mut in_group = false;
    let mut depth = 0usize;
    let mut output = Vec::new();
    for row in rows {
        match &row.kind {
            DataTableRowKind::GroupHeader {
                group_key: key,
                depth: d,
                ..
            } => {
                if key == group_key {
                    in_group = true;
                    depth = *d;
                } else if in_group && *d <= depth {
                    break;
                }
            }
            DataTableRowKind::Data if in_group => output.push(row),
            _ => {}
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn group_headers_collapse_children() {
        let rows = vec![
            DataTableRowModel::from_text_cells(
                "1",
                HashMap::from([
                    ("company".into(), "Acme".into()),
                    ("name".into(), "Ada".into()),
                ]),
            ),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([
                    ("company".into(), "Acme".into()),
                    ("name".into(), "Bob".into()),
                ]),
            ),
        ];
        let grouping = DataTableRowGrouping::new(vec!["company".into()]);
        let columns = vec![DataTableColumnDef::new("company", "Company")];
        let expanded = HashSet::new();
        let grouped = build_group_rows(
            rows,
            &grouping,
            &expanded,
            &columns,
            None,
            None,
            AggregationPosition::Footer,
        );
        assert_eq!(grouped.len(), 1);
        assert!(matches!(
            grouped[0].kind,
            DataTableRowKind::GroupHeader { .. }
        ));
    }
}
