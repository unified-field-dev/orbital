use std::cmp::Ordering;

use leptos::prelude::Callable;
use orbital_data::{CompareHint, DataValue};

use crate::types::{ColumnType, DataTableColumnDef};

pub fn compare_hint(col_type: ColumnType) -> CompareHint {
    match col_type {
        ColumnType::Text => CompareHint::Text,
        ColumnType::Number => CompareHint::Number,
        ColumnType::Date => CompareHint::Date,
        ColumnType::Boolean => CompareHint::Boolean,
        ColumnType::SingleSelect => CompareHint::SingleSelect,
        ColumnType::Actions => CompareHint::Text,
    }
}

pub fn default_compare(col_type: ColumnType, a: &DataValue, b: &DataValue) -> Ordering {
    let hint = compare_hint(col_type);
    a.partial_cmp_typed(b, hint)
        .unwrap_or_else(|| a.display_string().cmp(&b.display_string()))
}

pub fn compare_for_sort(col: &DataTableColumnDef, a: &DataValue, b: &DataValue) -> Ordering {
    if let Some(cmp) = &col.compare_values {
        return cmp.run((a.clone(), b.clone()));
    }
    default_compare(col.col_type, a, b)
}
