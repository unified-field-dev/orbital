use crate::engine::column::resolve_value;
use crate::engine::compare::compare_for_sort;
use crate::types::{DataTableColumnDef, DataTableRowModel, DataTableSort, SortRule};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

/// Legacy single-column sort state (prefer [`DataTableSort`]).
#[derive(Clone, Debug, Default)]
pub struct SortState {
    pub field: Option<String>,
    pub direction: SortDirection,
}

impl SortState {
    pub fn from_data_table_sort(sort: &DataTableSort) -> Self {
        sort.items.first().map_or(Self::default(), |rule| Self {
            field: Some(rule.field.clone()),
            direction: rule.direction,
        })
    }
}

impl From<&SortState> for DataTableSort {
    fn from(state: &SortState) -> Self {
        DataTableSort::from_sort_state(state)
    }
}

pub fn sort_rows_multi(
    mut rows: Vec<DataTableRowModel>,
    columns: &[DataTableColumnDef],
    sort: &DataTableSort,
) -> Vec<DataTableRowModel> {
    if sort.items.is_empty() {
        return rows;
    }

    rows.sort_by(|a, b| {
        for rule in &sort.items {
            let Some(col) = columns.iter().find(|c| c.field == rule.field && c.sortable) else {
                continue;
            };
            let av = resolve_value(col, a);
            let bv = resolve_value(col, b);
            let ord = compare_for_sort(col, &av, &bv);
            let cmp = match rule.direction {
                SortDirection::Asc => ord,
                SortDirection::Desc => ord.reverse(),
            };
            if cmp != std::cmp::Ordering::Equal {
                return cmp;
            }
        }
        std::cmp::Ordering::Equal
    });
    rows
}

pub fn sort_rows(
    rows: Vec<DataTableRowModel>,
    columns: &[DataTableColumnDef],
    sort: &SortState,
) -> Vec<DataTableRowModel> {
    sort_rows_multi(rows, columns, &DataTableSort::from_sort_state(sort))
}

/// Apply header click sort: single-column or multi-column (Ctrl/Cmd additive).
pub fn apply_header_sort(
    sort: &mut DataTableSort,
    field: &str,
    multi_column_enabled: bool,
    additive: bool,
) {
    if multi_column_enabled && additive {
        if let Some(existing) = sort.items.iter_mut().find(|r| r.field == field) {
            existing.direction = match existing.direction {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::Asc,
            };
        } else {
            sort.items.push(SortRule {
                field: field.to_string(),
                direction: SortDirection::Asc,
            });
        }
        return;
    }

    if let Some(existing) = sort.items.first() {
        if existing.field == field {
            sort.items = vec![SortRule {
                field: field.to_string(),
                direction: match existing.direction {
                    SortDirection::Asc => SortDirection::Desc,
                    SortDirection::Desc => SortDirection::Asc,
                },
            }];
            return;
        }
    }

    sort.items = vec![SortRule {
        field: field.to_string(),
        direction: SortDirection::Asc,
    }];
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use orbital_data::DataValue;

    #[test]
    fn sort_desc_reorders_rows() {
        let rows = vec![
            DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into())])),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([("name".into(), "Grace".into())]),
            ),
        ];
        let columns = vec![DataTableColumnDef::new("name", "Name")];
        let sort = SortState {
            field: Some("name".into()),
            direction: SortDirection::Desc,
        };
        let sorted = sort_rows(rows, &columns, &sort);
        assert_eq!(sorted[0].id(), "2");
        assert_eq!(sorted[1].id(), "1");
    }

    #[test]
    fn typed_number_sort() {
        let rows = vec![
            DataTableRowModel::new(orbital_data::DataRecord::new(
                "10",
                HashMap::from([("n".into(), DataValue::Number(10.0))]),
            )),
            DataTableRowModel::new(orbital_data::DataRecord::new(
                "2",
                HashMap::from([("n".into(), DataValue::Number(2.0))]),
            )),
        ];
        let columns =
            vec![DataTableColumnDef::new("n", "N").with_col_type(crate::types::ColumnType::Number)];
        let sort = SortState {
            field: Some("n".into()),
            direction: SortDirection::Asc,
        };
        let sorted = sort_rows(rows, &columns, &sort);
        assert_eq!(sorted[0].id(), "2");
        assert_eq!(sorted[1].id(), "10");
    }

    #[test]
    fn multi_sort_tie_breaks() {
        let rows = vec![
            DataTableRowModel::from_text_cells(
                "1",
                HashMap::from([("role".into(), "A".into()), ("name".into(), "Bob".into())]),
            ),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([("role".into(), "A".into()), ("name".into(), "Ada".into())]),
            ),
            DataTableRowModel::from_text_cells(
                "3",
                HashMap::from([("role".into(), "B".into()), ("name".into(), "Zed".into())]),
            ),
        ];
        let columns = vec![
            DataTableColumnDef::new("role", "Role"),
            DataTableColumnDef::new("name", "Name"),
        ];
        let sort = DataTableSort {
            items: vec![
                SortRule {
                    field: "role".into(),
                    direction: SortDirection::Asc,
                },
                SortRule {
                    field: "name".into(),
                    direction: SortDirection::Asc,
                },
            ],
        };
        let sorted = sort_rows_multi(rows, &columns, &sort);
        assert_eq!(sorted[0].id(), "2");
        assert_eq!(sorted[1].id(), "1");
        assert_eq!(sorted[2].id(), "3");
    }
}
