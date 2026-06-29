# orbital-datatable

Leptos **DataTable** — sortable, filterable, editable grids with paging, grouping, pivot, export, and optional chart binding.

## Key types

- `DataTable`, `DataTableColumnDef`, `DataTableRowModel`, `DataTableFeatures`
- `DataTableSource`, `DataTableHandle`, `ChartBinding`
- Shared data: [`orbital-data`](../orbital-data/) `Dataset`, `DataRecord`, `DataValue`

## Preview

Local: `http://127.0.0.1:3010/orbital/data-table` (with `cargo leptos watch -p orbital-preview`).

## Deferred (not in current charter)

- **Chart Panel UI** — toolbar config drawer for chart type and field mapping (charts integration wiring is shipped).

## Docs

Consumer API: component rustdoc and preview catalog (`#[component_doc]`). Use `default-features = false` in production; enable `preview` for the doc host.
