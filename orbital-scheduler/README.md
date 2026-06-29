# orbital-scheduler

Leptos scheduling views — calendar and timeline products with drag-resize, resource lanes, and event editing.

## Key types

- `SchedulerCalendar`, `SchedulerTimeline`
- `PlannedEvent`, `ResourceRow`, `TimelinePreset`
- Shared datetime: [`orbital-base-components`](../orbital-base-components/) `OrbitalDateTime`

## Preview

Local: `http://127.0.0.1:3010/orbital/scheduler-calendar` (with `cargo leptos watch -p orbital-preview`).

## Deferred (not in current charter)

- Calendar and timeline filtering
- ICS import/export

## Docs

Consumer API: component rustdoc and preview catalog. Use `default-features = false` in production; enable `preview` for the doc host.
