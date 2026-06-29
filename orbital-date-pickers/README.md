# orbital-date-pickers

Leptos date and time pickers — calendar, fields, range pickers, digital and analog clocks, timezone-aware values.

## Quick start

```toml
[dependencies]
orbital-date-pickers = { git = "https://github.com/unified-field-dev/orbital", default-features = false }
orbital = { git = "https://github.com/unified-field-dev/orbital", default-features = false, features = ["hydrate"] }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

Use `default-features = false` in production; enable `preview` only for the doc host.

## Key types

- `DateCalendar`, `DateField`, `DateTimeField`, `DateRangePicker`, `DateTimeRangePicker`
- `DatePickerFeatures`, `DatetimeLocale`, `DatetimeLocaleStrings`
- Shared datetime: [`orbital-base-components`](../orbital-base-components/) `OrbitalDateTime`, `DatetimeTimezone`, `DatetimeFormat`

## Preview

[Date picker preview](https://unified-field-dev.github.io/orbital/date-picker) · local `http://127.0.0.1:3010/orbital/date-picker` (with `cargo leptos watch -p orbital-preview`)

## Deferred (not in current charter)

- Alternate calendar systems (Jalali, Hijri).

## Docs

Consumer API: component rustdoc and preview catalog. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
