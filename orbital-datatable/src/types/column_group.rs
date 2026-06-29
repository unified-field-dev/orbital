use std::sync::Arc;

use leptos::prelude::*;

/// Nested column group for multi-row headers.
#[derive(Clone)]
pub struct DataTableColumnGroupDef {
    /// Stable group identifier used in layout resolution.
    pub group_id: String,
    /// Display label for the group header row.
    pub header_name: String,
    /// Optional custom header renderer for the group cell.
    pub header_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// Nested groups or leaf column field references.
    pub children: Vec<DataTableColumnGroupChild>,
}

impl std::fmt::Debug for DataTableColumnGroupDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataTableColumnGroupDef")
            .field("group_id", &self.group_id)
            .field("header_name", &self.header_name)
            .field("children", &self.children.len())
            .finish_non_exhaustive()
    }
}

impl DataTableColumnGroupDef {
    pub fn new(group_id: impl Into<String>, header_name: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            header_name: header_name.into(),
            header_view: None,
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: Vec<DataTableColumnGroupChild>) -> Self {
        self.children = children;
        self
    }

    pub fn with_header_view(mut self, view: Arc<dyn Fn() -> AnyView + Send + Sync>) -> Self {
        self.header_view = Some(view);
        self
    }
}

/// Child of a column group — nested group or leaf column reference.
#[derive(Clone, Debug)]
pub enum DataTableColumnGroupChild {
    Group(DataTableColumnGroupDef),
    Column { field: String },
}

impl DataTableColumnGroupChild {
    pub fn column(field: impl Into<String>) -> Self {
        Self::Column {
            field: field.into(),
        }
    }

    pub fn group(group: DataTableColumnGroupDef) -> Self {
        Self::Group(group)
    }
}
