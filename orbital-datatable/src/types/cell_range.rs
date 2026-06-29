/// A single cell address in the visible grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellCoord {
    /// Stable row id (from [`crate::GetRowId`] when set).
    pub row_id: String,
    /// Column field key.
    pub field: String,
}

impl CellCoord {
    pub fn new(row_id: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            row_id: row_id.into(),
            field: field.into(),
        }
    }
}

/// Spreadsheet-style cell range selection (anchor + focus).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CellSelection {
    /// Range start cell (fixed on first click).
    pub anchor: Option<CellCoord>,
    /// Range end cell (follows pointer/keyboard focus).
    pub focus: Option<CellCoord>,
}

/// Normalized row/column index bounds into the visible grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalizedCellRange {
    /// First selected row index (inclusive).
    pub row_start: usize,
    /// Last selected row index (inclusive).
    pub row_end: usize,
    /// First selected column index (inclusive).
    pub col_start: usize,
    /// Last selected column index (inclusive).
    pub col_end: usize,
}

impl CellSelection {
    pub fn is_active(&self) -> bool {
        self.anchor.is_some() && self.focus.is_some()
    }

    pub fn clear(&mut self) {
        self.anchor = None;
        self.focus = None;
    }

    pub fn normalized(&self, row_ids: &[String], fields: &[String]) -> Option<NormalizedCellRange> {
        let anchor = self.anchor.as_ref()?;
        let focus = self.focus.as_ref()?;
        let row_start_idx = row_ids.iter().position(|id| id == &anchor.row_id)?;
        let row_end_idx = row_ids.iter().position(|id| id == &focus.row_id)?;
        let col_start_idx = fields.iter().position(|f| f == &anchor.field)?;
        let col_end_idx = fields.iter().position(|f| f == &focus.field)?;
        Some(NormalizedCellRange {
            row_start: row_start_idx.min(row_end_idx),
            row_end: row_start_idx.max(row_end_idx),
            col_start: col_start_idx.min(col_end_idx),
            col_end: col_start_idx.max(col_end_idx),
        })
    }
}

/// Whether a cell lies inside a normalized range.
pub fn cell_in_range(
    row_id: &str,
    field: &str,
    row_ids: &[String],
    fields: &[String],
    range: NormalizedCellRange,
) -> bool {
    let Some(row_idx) = row_ids.iter().position(|id| id == row_id) else {
        return false;
    };
    let Some(col_idx) = fields.iter().position(|f| f == field) else {
        return false;
    };
    row_idx >= range.row_start
        && row_idx <= range.row_end
        && col_idx >= range.col_start
        && col_idx <= range.col_end
}

/// CSS edge classes for range border rendering.
pub fn range_edge_classes(
    row_id: &str,
    field: &str,
    row_ids: &[String],
    fields: &[String],
    range: NormalizedCellRange,
) -> Vec<&'static str> {
    if !cell_in_range(row_id, field, row_ids, fields, range) {
        return Vec::new();
    }
    let Some(row_idx) = row_ids.iter().position(|id| id == row_id) else {
        return Vec::new();
    };
    let Some(col_idx) = fields.iter().position(|f| f == field) else {
        return Vec::new();
    };
    let mut classes = vec!["orbital-data-table__cell--range-selected"];
    if row_idx == range.row_start {
        classes.push("orbital-data-table__cell--range-top");
    }
    if row_idx == range.row_end {
        classes.push("orbital-data-table__cell--range-bottom");
    }
    if col_idx == range.col_start {
        classes.push("orbital-data-table__cell--range-left");
    }
    if col_idx == range.col_end {
        classes.push("orbital-data-table__cell--range-right");
    }
    classes
}
