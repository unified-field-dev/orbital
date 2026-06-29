use leptos::prelude::*;

use crate::types::{ChartBinding, ColumnType, DataTableColumnDef};

/// Derive suggested chart field keys from visible table columns.
pub fn derive_schema_hints(columns: &[DataTableColumnDef]) -> (Option<String>, Vec<String>) {
    let x_field = columns
        .iter()
        .find(|c| {
            matches!(
                c.col_type,
                ColumnType::Text | ColumnType::SingleSelect | ColumnType::Date
            )
        })
        .map(|c| c.field.clone());
    let y_fields = columns
        .iter()
        .filter(|c| c.col_type == ColumnType::Number)
        .map(|c| c.field.clone())
        .collect();
    (x_field, y_fields)
}

/// Provide [`ChartBinding`] to descendants (chart slots, dashboard panels).
pub fn provide_chart_binding(binding: ChartBinding) {
    provide_context(binding);
}

/// Read the nearest [`ChartBinding`] when [`DataTableFeatures::CHARTS_INTEGRATION`] is enabled.
pub fn use_chart_binding() -> Option<ChartBinding> {
    use_context::<ChartBinding>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DataTableColumnDef;

    #[test]
    fn derive_schema_hints_picks_category_and_numeric_columns() {
        let columns = vec![
            DataTableColumnDef::new("name", "Name"),
            DataTableColumnDef::new("score", "Score").with_col_type(ColumnType::Number),
            DataTableColumnDef::new("notes", "Notes"),
        ];
        let (x, ys) = derive_schema_hints(&columns);
        assert_eq!(x.as_deref(), Some("name"));
        assert_eq!(ys, vec!["score".to_string()]);
    }
}
