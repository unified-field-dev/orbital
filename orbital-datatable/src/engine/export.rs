use std::collections::HashMap;

use orbital_data::{DataRecord, DataSchema, DataType, DataValue, Dataset, FieldDef};

use crate::engine::{format_display, resolve_value};
use crate::types::{ColumnType, DataTableColumnDef, DataTableRowModel, NormalizedCellRange};

fn column_data_type(col_type: ColumnType) -> DataType {
    match col_type {
        ColumnType::Text | ColumnType::SingleSelect | ColumnType::Actions => DataType::Text,
        ColumnType::Number => DataType::Number,
        ColumnType::Date => DataType::Date,
        ColumnType::Boolean => DataType::Bool,
    }
}

/// Which rows to include in an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportRowScope {
    /// All rows matching current filter/sort/visibility (ignores pagination).
    AllMatching,
    /// Currently rendered processed rows (respects pagination).
    CurrentPage,
    /// Row checkbox selection ids.
    SelectedRows,
    /// Active cell range.
    CellRange,
}

/// Build a typed dataset from rows and visible column definitions.
pub fn build_export_dataset(rows: &[DataTableRowModel], columns: &[DataTableColumnDef]) -> Dataset {
    let fields: Vec<FieldDef> = columns
        .iter()
        .map(|col| FieldDef {
            key: col.field.clone(),
            label: col.header_name.clone(),
            data_type: column_data_type(col.col_type),
        })
        .collect();
    let schema = DataSchema::new(fields);
    let records: Vec<DataRecord> = rows
        .iter()
        .map(|row| {
            let mut values = HashMap::new();
            for col in columns {
                let value = resolve_value(col, row);
                values.insert(col.field.clone(), value);
            }
            DataRecord {
                id: row.id().to_string(),
                values,
            }
        })
        .collect();
    Dataset::from_records(schema, records)
}

/// Format a cell value for export (matches on-screen display).
pub fn export_cell_text(column: &DataTableColumnDef, row: &DataTableRowModel) -> String {
    let value = resolve_value(column, row);
    format_display(column, &value)
}

/// Serialize a dataset to CSV with RFC-style escaping.
pub fn serialize_csv(dataset: &Dataset, columns: &[DataTableColumnDef]) -> String {
    let mut lines = Vec::new();
    let headers: Vec<String> = columns
        .iter()
        .map(|c| escape_csv_field(&c.header_name))
        .collect();
    lines.push(headers.join(","));

    for record in &dataset.records {
        let cells: Vec<String> = columns
            .iter()
            .map(|col| {
                let value = record
                    .values
                    .get(&col.field)
                    .cloned()
                    .unwrap_or(DataValue::Null);
                escape_csv_field(&format_display(col, &value))
            })
            .collect();
        lines.push(cells.join(","));
    }
    lines.join("\r\n")
}

fn escape_csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Build an HTML table string for print export.
pub fn serialize_print_html(dataset: &Dataset, columns: &[DataTableColumnDef]) -> String {
    let mut html = String::from(
        "<html><head><title>Export</title><style>table{border-collapse:collapse;width:100%}th,td{border:1px solid #ccc;padding:4px 8px;text-align:left}</style></head><body><table>",
    );
    html.push_str("<thead><tr>");
    for col in columns {
        html.push_str("<th>");
        html.push_str(&html_escape(&col.header_name));
        html.push_str("</th>");
    }
    html.push_str("</tr></thead><tbody>");
    for record in &dataset.records {
        html.push_str("<tr>");
        for col in columns {
            html.push_str("<td>");
            let value = record
                .values
                .get(&col.field)
                .cloned()
                .unwrap_or(DataValue::Null);
            html.push_str(&html_escape(&format_display(col, &value)));
            html.push_str("</td>");
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></body></html>");
    html
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Filter rows by index bounds in the visible grid.
pub fn rows_in_cell_range(
    rows: &[DataTableRowModel],
    range: NormalizedCellRange,
) -> Vec<&DataTableRowModel> {
    rows.iter()
        .enumerate()
        .filter(|(idx, _)| *idx >= range.row_start && *idx <= range.row_end)
        .map(|(_, row)| row)
        .collect()
}

/// Filter columns to those in the cell range.
pub fn columns_in_cell_range(
    columns: &[DataTableColumnDef],
    fields: &[String],
    range: NormalizedCellRange,
) -> Vec<DataTableColumnDef> {
    fields
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx >= range.col_start && *idx <= range.col_end)
        .filter_map(|(_, field)| columns.iter().find(|c| &c.field == field).cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn csv_escapes_commas_and_quotes() {
        assert_eq!(escape_csv_field("hello"), "hello");
        assert_eq!(escape_csv_field("a,b"), "\"a,b\"");
        assert_eq!(escape_csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn serialize_csv_includes_headers() {
        let cols = vec![DataTableColumnDef::new("name", "Name")];
        let rows = vec![DataTableRowModel::from_text_cells(
            "1",
            HashMap::from([("name".into(), "Ada".into())]),
        )];
        let dataset = build_export_dataset(&rows, &cols);
        let csv = serialize_csv(&dataset, &cols);
        assert!(csv.starts_with("Name\r\n"));
        assert!(csv.contains("Ada"));
    }
}
