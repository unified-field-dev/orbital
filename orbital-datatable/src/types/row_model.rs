use std::collections::HashMap;

use leptos::prelude::*;
use orbital_data::DataRecord;

use crate::types::GroupAggregates;

/// Optional custom row id resolver (default: [`DataRecord::id`]).
pub type GetRowId = Callback<(DataRecord,), String>;

/// Resolve a stable row id using an optional custom resolver.
pub fn resolve_row_id(record: &DataRecord, resolver: Option<&GetRowId>) -> String {
    resolver
        .map(|r| r.run((record.clone(),)))
        .unwrap_or_else(|| record.id.clone())
}

/// Display row kind for tree, grouping, and aggregation chrome rows.
#[derive(Clone, Debug, Default)]
pub enum DataTableRowKind {
    /// Normal data row.
    #[default]
    Data,
    /// Tree branch row metadata (grouping column indent + chevron).
    TreeBranch { depth: usize, path_key: String },
    /// Row grouping header row.
    GroupHeader {
        group_key: String,
        field: String,
        depth: usize,
        child_count: usize,
        aggregates: GroupAggregates,
    },
    /// Footer aggregate summary row.
    AggregateFooter,
}

impl DataTableRowKind {
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data)
    }

    pub fn is_selectable(&self) -> bool {
        matches!(self, Self::Data)
    }
}

/// Row model for [`crate::DataTable`].
#[derive(Clone, Debug, Default)]
pub struct DataTableRowModel {
    /// Display kind (data, tree branch, group header, aggregate footer).
    pub kind: DataTableRowKind,
    /// Underlying dataset record for cell values and selection id.
    pub record: DataRecord,
}

impl DataTableRowModel {
    pub fn new(record: DataRecord) -> Self {
        Self {
            kind: DataTableRowKind::Data,
            record,
        }
    }

    pub fn with_kind(kind: DataTableRowKind, record: DataRecord) -> Self {
        Self { kind, record }
    }

    /// Build from a string cell map (preview/demo convenience).
    pub fn from_text_cells(id: impl Into<String>, cells: HashMap<String, String>) -> Self {
        Self {
            kind: DataTableRowKind::Data,
            record: DataRecord::from_text_map(id, cells),
        }
    }

    pub fn id(&self) -> &str {
        &self.record.id
    }

    pub fn resolved_id(&self, resolver: Option<&GetRowId>) -> String {
        resolve_row_id(&self.record, resolver)
    }

    pub fn get(&self, field: &str) -> Option<&orbital_data::DataValue> {
        self.record.get(field)
    }

    pub fn is_data_row(&self) -> bool {
        self.kind.is_data()
    }
}

/// Row selection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTableSelectionMode {
    Single,
    Multiselect,
}
