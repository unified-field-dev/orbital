use leptos::prelude::Callable;
use orbital_data::DataValue;

use crate::types::{DataTableColumnDef, DataTableRowModel};

pub fn resolve_value(col: &DataTableColumnDef, row: &DataTableRowModel) -> DataValue {
    if let Some(resolver) = &col.resolve_value {
        return resolver.run((row.record.clone(),));
    }
    row.get(&col.field).cloned().unwrap_or(DataValue::Null)
}

pub fn format_display(col: &DataTableColumnDef, value: &DataValue) -> String {
    if let Some(formatter) = &col.format_display {
        return formatter.run((value.clone(),));
    }
    value.display_string()
}
