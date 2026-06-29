use crate::types::{DataTableRowModel, GetRowId, PinnedRowsState};

/// Partition processed rows into top-pinned, scrollable middle, and bottom-pinned sections.
pub fn partition_rows(
    rows: &[DataTableRowModel],
    pinned: &PinnedRowsState,
    get_row_id: Option<&GetRowId>,
) -> (
    Vec<DataTableRowModel>,
    Vec<DataTableRowModel>,
    Vec<DataTableRowModel>,
) {
    let top_set: std::collections::HashSet<&str> = pinned.top.iter().map(String::as_str).collect();
    let bottom_set: std::collections::HashSet<&str> =
        pinned.bottom.iter().map(String::as_str).collect();

    let mut top = Vec::new();
    let mut middle = Vec::new();
    let mut bottom = Vec::new();

    for pinned_id in &pinned.top {
        if let Some(row) = rows
            .iter()
            .find(|r| r.resolved_id(get_row_id) == *pinned_id)
        {
            top.push(row.clone());
        }
    }

    for row in rows {
        let id = row.resolved_id(get_row_id);
        if top_set.contains(id.as_str()) || bottom_set.contains(id.as_str()) {
            continue;
        }
        middle.push(row.clone());
    }

    for pinned_id in &pinned.bottom {
        if let Some(row) = rows
            .iter()
            .find(|r| r.resolved_id(get_row_id) == *pinned_id)
        {
            bottom.push(row.clone());
        }
    }

    (top, middle, bottom)
}
