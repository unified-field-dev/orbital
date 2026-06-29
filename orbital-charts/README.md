# orbital-charts

Leptos chart library — bar, line, area, pie, scatter, heatmap, gauge, sparkline, zoom/pan, tooltips, and keyboard navigation.

## Key types

- `BarChart`, `LineChart`, `PieChart`, `ScatterChart`, `HeatmapChart`, `GaugeChart`
- `ChartContainer`, `ChartFeatures`, `ChartEmbedMode`, `ChartOverlayLayer`
- Shared data: [`orbital-data`](../orbital-data/) `Dataset` (projected via `engine::project_chart_data`)

## Preview

Local: `http://127.0.0.1:3010/orbital/bar-chart` (with `cargo leptos watch -p orbital-preview`).

## Deferred (not in current charter)

Advanced chart types: `RadarChart`, `FunnelChart`, `SankeyChart`, `CandlestickChart`, `RadialBarChart`, `RadialLineChart`, `TreemapChart`, `GanttChart`, `PyramidChart`, `RangeBarChart`, polar axes.

## Docs

Consumer API: component rustdoc and preview catalog. Use `default-features = false` in production; enable `preview` for the doc host.
