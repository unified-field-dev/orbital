use std::collections::HashMap;

use crate::types::{DataTableRowModel, GetRowId};

/// Stable-sort rows by an explicit id order list.
pub fn apply_row_order(
    rows: Vec<DataTableRowModel>,
    order: &[String],
    get_row_id: Option<&GetRowId>,
) -> Vec<DataTableRowModel> {
    if order.is_empty() {
        return rows;
    }

    let index: HashMap<String, usize> = order
        .iter()
        .enumerate()
        .map(|(i, id)| (id.clone(), i))
        .collect();

    let mut keyed: Vec<(usize, usize, DataTableRowModel)> = rows
        .into_iter()
        .enumerate()
        .map(|(orig, row)| {
            let id = row.resolved_id(get_row_id);
            let key = index.get(&id).copied().unwrap_or(usize::MAX);
            (key, orig, row)
        })
        .collect();

    keyed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    keyed.into_iter().map(|(_, _, row)| row).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn reorders_by_id_list() {
        let rows = vec![
            DataTableRowModel::from_text_cells("a", HashMap::from([("n".into(), "A".into())])),
            DataTableRowModel::from_text_cells("b", HashMap::from([("n".into(), "B".into())])),
            DataTableRowModel::from_text_cells("c", HashMap::from([("n".into(), "C".into())])),
        ];
        let ordered = apply_row_order(rows, &["c".into(), "a".into(), "b".into()], None);
        assert_eq!(ordered[0].id(), "c");
        assert_eq!(ordered[1].id(), "a");
        assert_eq!(ordered[2].id(), "b");
    }
}
