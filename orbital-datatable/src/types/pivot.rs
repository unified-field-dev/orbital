use leptos::prelude::*;

use crate::types::AggregationFn;

/// Pivot configuration: row dimensions, column dimensions, and value measures.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTablePivotModel {
    /// Field keys placed on pivot rows.
    pub row_fields: Vec<String>,
    /// Field keys placed on pivot columns.
    pub column_fields: Vec<String>,
    /// Numeric field keys to aggregate in cells.
    pub value_fields: Vec<String>,
    /// Aggregate function applied to value fields.
    pub value_fn: AggregationFn,
}

impl DataTablePivotModel {
    pub fn is_active(&self) -> bool {
        !self.value_fields.is_empty()
            && (!self.row_fields.is_empty() || !self.column_fields.is_empty())
    }
}

/// Signal-backed pivot model.
pub type PivotSignal = Signal<DataTablePivotModel>;
