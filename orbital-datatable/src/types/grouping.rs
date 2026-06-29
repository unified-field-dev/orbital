use leptos::prelude::*;

/// Row grouping configuration (group-by column field keys, in order).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTableRowGrouping {
    /// Field keys to group by, outermost first.
    pub model: Vec<String>,
    /// Optional field used for the grouping column (defaults to first model field).
    pub grouping_column: Option<String>,
}

impl DataTableRowGrouping {
    pub fn new(model: Vec<String>) -> Self {
        Self {
            model,
            grouping_column: None,
        }
    }

    pub fn with_grouping_column(mut self, field: impl Into<String>) -> Self {
        self.grouping_column = Some(field.into());
        self
    }

    pub fn is_active(&self) -> bool {
        !self.model.is_empty()
    }

    pub fn grouping_field(&self) -> Option<&str> {
        self.grouping_column
            .as_deref()
            .or_else(|| self.model.first().map(String::as_str))
    }
}

/// Signal-backed grouping model for reactive tables.
pub type RowGroupingSignal = Signal<DataTableRowGrouping>;
