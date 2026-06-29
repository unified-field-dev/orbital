//! Optional DataTable capability flags (replaces former license-tier product stubs).

use std::fmt;

bitflags::bitflags! {
    /// Optional capabilities enabled on [`crate::DataTable`].
    ///
    /// Combine with `|` when multiple features are needed. Subcomponents document
    /// which flag they require via `Requires DataTableFeatures::…` in rustdoc.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct DataTableFeatures: u32 {
        /// Pin columns left or right.
        const COLUMN_PINNING = 1 << 0;
        /// Drag-and-drop column reorder.
        const COLUMN_REORDER = 1 << 1;
        /// Pin rows top or bottom.
        const ROW_PINNING = 1 << 2;
        /// Drag-and-drop row reorder.
        const ROW_REORDER = 1 << 3;
        /// Expandable detail panel per row.
        const ROW_DETAIL = 1 << 4;
        /// Hierarchical rows with grouping column.
        const TREE_DATA = 1 << 5;
        /// Row/column virtualization for large datasets.
        const VIRTUALIZATION = 1 << 6;
        /// Group rows by column values.
        const ROW_GROUPING = 1 << 7;
        /// Footer/group aggregation functions.
        const AGGREGATION = 1 << 8;
        /// Pivot table mode.
        const PIVOTING = 1 << 9;
        /// List/card layout variant.
        const LIST_VIEW = 1 << 10;
        /// Range cell selection like a spreadsheet.
        const CELL_SELECTION = 1 << 11;
        /// Copy/paste cells and ranges.
        const CLIPBOARD = 1 << 12;
        /// Excel export (CSV/print always available).
        const EXCEL_EXPORT = 1 << 13;
        /// Undo/redo edit history.
        const UNDO_REDO = 1 << 14;
        /// Inline header row filters.
        const HEADER_FILTERS = 1 << 15;
        /// Multiple filters per column.
        const MULTI_FILTER = 1 << 16;
        /// Multi-column sort.
        const MULTI_COLUMN_SORT = 1 << 17;
        /// Embed charts in grid cells/panels.
        const CHARTS_INTEGRATION = 1 << 19;
    }
}

impl fmt::Display for DataTableFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "none");
        }
        let mut names = Vec::new();
        macro_rules! push {
            ($flag:ident, $name:literal) => {
                if self.contains(Self::$flag) {
                    names.push($name);
                }
            };
        }
        push!(COLUMN_PINNING, "column_pinning");
        push!(COLUMN_REORDER, "column_reorder");
        push!(ROW_PINNING, "row_pinning");
        push!(ROW_REORDER, "row_reorder");
        push!(ROW_DETAIL, "row_detail");
        push!(TREE_DATA, "tree_data");
        push!(VIRTUALIZATION, "virtualization");
        push!(ROW_GROUPING, "row_grouping");
        push!(AGGREGATION, "aggregation");
        push!(PIVOTING, "pivoting");
        push!(LIST_VIEW, "list_view");
        push!(CELL_SELECTION, "cell_selection");
        push!(CLIPBOARD, "clipboard");
        push!(EXCEL_EXPORT, "excel_export");
        push!(UNDO_REDO, "undo_redo");
        push!(HEADER_FILTERS, "header_filters");
        push!(MULTI_FILTER, "multi_filter");
        push!(MULTI_COLUMN_SORT, "multi_column_sort");
        push!(CHARTS_INTEGRATION, "charts_integration");
        write!(f, "{}", names.join(", "))
    }
}
