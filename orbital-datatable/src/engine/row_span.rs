use crate::engine::{compare_for_sort, resolve_value};
use crate::types::{DataTableColumnDef, DataTableRowModel};

/// Per-row slot for row-span rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowSpanSlot {
    pub rowspan: u32,
    pub skip: bool,
}

impl RowSpanSlot {
    pub fn render(rowspan: u32) -> Self {
        Self {
            rowspan,
            skip: false,
        }
    }

    pub fn skip() -> Self {
        Self {
            rowspan: 1,
            skip: true,
        }
    }
}

/// Compute row-span slots for a column with `row_span_merge` enabled.
pub fn compute_row_spans(
    rows: &[DataTableRowModel],
    column: &DataTableColumnDef,
) -> Vec<RowSpanSlot> {
    if rows.is_empty() {
        return Vec::new();
    }

    let mut slots = Vec::with_capacity(rows.len());
    let mut i = 0usize;
    while i < rows.len() {
        let value = resolve_value(column, &rows[i]);
        let mut span = 1u32;
        let mut j = i + 1;
        while j < rows.len() {
            let next = resolve_value(column, &rows[j]);
            if compare_for_sort(column, &value, &next).is_eq() {
                span += 1;
                j += 1;
            } else {
                break;
            }
        }
        slots.push(RowSpanSlot::render(span));
        for _ in 1..span as usize {
            slots.push(RowSpanSlot::skip());
        }
        i = j;
    }
    slots
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::{DataRecord, DataValue};
    use std::collections::HashMap;

    fn row(id: &str, qty: f64) -> DataTableRowModel {
        DataTableRowModel::new(DataRecord::new(
            id,
            HashMap::from([("qty".into(), DataValue::Number(qty))]),
        ))
    }

    #[test]
    fn merges_consecutive_equal_values() {
        let rows = vec![
            row("1", 2.0),
            row("2", 2.0),
            row("3", 5.0),
            row("4", 5.0),
            row("5", 5.0),
        ];
        let col = DataTableColumnDef::new("qty", "Qty")
            .with_col_type(crate::types::ColumnType::Number)
            .with_row_span_merge(true);
        let slots = compute_row_spans(&rows, &col);
        assert_eq!(slots.len(), 5);
        assert_eq!(slots[0].rowspan, 2);
        assert!(!slots[0].skip);
        assert!(slots[1].skip);
        assert_eq!(slots[2].rowspan, 3);
        assert!(slots[3].skip);
        assert!(slots[4].skip);
    }
}
