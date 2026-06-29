# Orbital

[![CI](https://github.com/unified-field-dev/orbital/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/orbital/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Preview catalog](https://img.shields.io/badge/docs-preview_catalog-blue)](https://unified-field-dev.github.io/orbital/)

**Orbital** is a Leptos component library and design system for building focused, accessible product interfaces in Rust. It ships themed UI primitives, shared typed data models, and higher-level feature crates (tables, charts, scheduling, and more) that compose on the same tokens, motion vocabulary, and accessibility patterns.

*A themed component stack for focused, accessible product UIs in Rust.*

**Status:** v0.1.0 early release · [MIT](LICENSE) · [GitHub](https://github.com/unified-field-dev/orbital)

**Requires:** nightly Rust ([`rust-toolchain.toml`](rust-toolchain.toml)) · [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) **≥ 0.3.6** · Node 24+ for Playwright

## Why Orbital

| Pain | Orbital answer |
|------|----------------|
| Reinventing tables, charts, and pickers per screen | Feature crates with shared tokens and typed data models |
| Inconsistent spacing, motion, and brand tone | CSS-variable design language across shell, theme, and components |
| Component docs that drift from code | `#[component_doc]` macro + live preview catalog with Playwright coverage (~460 previews) |
| Heavy preview deps in production builds | `default-features = false` on component crates; enable `preview` only for the doc host |

## What you get

- **Foundation components** — buttons, forms, navigation, overlays, tables, and layout primitives built for SSR and hydration.
- **Feature crates** — data tables, charts, date/time pickers, schedulers, trees, and discussion threads for product-heavy screens.
- **Design language** — spacing, typography, elevation, surface materials, brand tone, and motion tokens wired through CSS variables and theme hooks.
- **Shared types** — UI-agnostic crates for datasets, paging, and datetime values so tables, charts, and schedulers agree on the same wire format.
- **Live component preview** — every public component has an interactive doc page with examples and Playwright coverage.

## Design language

Orbital is more than a bag of widgets. Components share a single visual and interaction vocabulary:

| Layer | Crate | What it defines |
|-------|-------|-----------------|
| Tokens | `orbital-shell`, `orbital-style` | Corner radius, elevation, materials, brand tone, stroke, glow |
| Theme | `orbital-theme` | Light/dark mode, density (`Compact` / `Default` / `Spacious`), typography and spacing overrides |
| Motion | `orbital-motion` | Duration curves, presence transitions, reduced-motion respect |
| Components | `orbital-core-components` | Themed controls that consume the tokens above |

Authors wrap apps in `OrbitalThemeProvider` and compose shell chrome via [`OrbitalTemplate`](orbital/README.md). Capability is expressed through feature flags on each crate (for example `DataTableFeatures`, `ChartFeatures`) — not license-tier suffixes on type names.

Feature crate overviews: [`orbital-datatable/README.md`](orbital-datatable/README.md), [`orbital-charts/README.md`](orbital-charts/README.md), [`orbital-date-pickers/README.md`](orbital-date-pickers/README.md), [`orbital-scheduler/README.md`](orbital-scheduler/README.md), [`orbital-discussion/README.md`](orbital-discussion/README.md).

## Shared types

These crates have **no UI dependency** — use them in server functions, adapters, and tests without pulling in Leptos views.

| Crate | Key types | Used by |
|-------|-----------|---------|
| [`orbital-data`](orbital-data/) | `Dataset`, `DataRecord`, `DataValue`, `DataSchema`, `FieldDef` | Data tables and charts share one chart-friendly dataset model |
| [`orbital-paging`](orbital-paging/) | `Page<T>`, `PageRequest`, `SortDirection` | Server fetch, infinite scroll, table paging |
| [`orbital-base-components`](orbital-base-components/) (datetime) | `OrbitalDateTime`, `DatetimeTimezone`, `DatetimeFormat` | Date pickers and scheduler events |
| [`orbital-markdown`](orbital-markdown/) | `OrbitalMarkdownOptions`, citation refs | Discussion and rich text rendering |

## Component surface

Import the facade crate [`orbital`](orbital/) for shell, auth hooks, and `orbital::primitives`, or depend on individual workspace crates with `default-features = false` in production (enable `preview` only for the doc host).

### Foundation

Buttons, inputs, dialogs, drawers, menus, tabs, cards, lists, avatars, pagination, and layout (`Flex`, `Grid`, `Stack`, `ScrollArea`). Browse the live preview catalog at `/orbital/` when the preview server is running.

### Feature crates

| Product | Crate | Highlights |
|---------|-------|------------|
| Data Table | `orbital-datatable` | Sorting, filtering, editing, grouping, pivot, export, chart binding |
| Charts | `orbital-charts` | Bar, line, pie, gauge, scatter, heatmap, zoom/pan, tooltips |
| Date Pickers | `orbital-date-pickers` | Calendar, fields, range pickers, timezone-aware `OrbitalDateTime` |
| Scheduler | `orbital-scheduler` | Calendar and timeline views, drag-resize, resource lanes |
| Tree | `orbital-tree` | Hierarchical navigation and selection |
| Discussion | `orbital-discussion` | Threaded replies, composer, markdown, attachments |

## Live preview

The component preview catalog is published to GitHub Pages on each version tag (`v*`):

**https://unified-field-dev.github.io/orbital/**

| Page | URL |
|------|-----|
| Catalog home | [unified-field-dev.github.io/orbital](https://unified-field-dev.github.io/orbital) |
| Button | [unified-field-dev.github.io/orbital/button](https://unified-field-dev.github.io/orbital/button) |
| Data Table | [unified-field-dev.github.io/orbital/data-table](https://unified-field-dev.github.io/orbital/data-table) |
| Charts | [unified-field-dev.github.io/orbital/bar-chart](https://unified-field-dev.github.io/orbital/bar-chart) |
| Date Pickers | [unified-field-dev.github.io/orbital/date-picker](https://unified-field-dev.github.io/orbital/date-picker) |
| Scheduler | [unified-field-dev.github.io/orbital/scheduler-calendar](https://unified-field-dev.github.io/orbital/scheduler-calendar) |
| Discussion | [unified-field-dev.github.io/orbital/discussion](https://unified-field-dev.github.io/orbital/discussion) |
| Tree | [unified-field-dev.github.io/orbital/tree-view](https://unified-field-dev.github.io/orbital/tree-view) |

Local equivalent: `http://127.0.0.1:3010/{slug}` while `cargo leptos watch -p orbital-preview` is running.

## Quick start

Add the facade crate from GitHub (enable features per target):

```toml
[dependencies]
orbital = { git = "https://github.com/unified-field-dev/orbital", default-features = false }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

For SSR + hydrate, enable `orbital` features such as `ssr` and `hydrate` on the facade and dependent Leptos crates. For production component crates without the preview catalog, use `default-features = false` on each `orbital-*` dependency and enable only the features you need. See [orbital-macros/README.md](orbital-macros/README.md#consumer-feature-flags).

```rust,ignore
use leptos::prelude::*;
use orbital::{orbital_shell, OrbitalTemplate};

#[component]
fn App() -> impl IntoView {
    view! {
        <OrbitalTemplate>
            <main>"Hello Orbital"</main>
        </OrbitalTemplate>
    }
}
```

See [`orbital/README.md`](orbital/README.md) for the preview workflow, auth context, and shell composition.

## Workspace crates

| Crate | Description |
|-------|-------------|
| `orbital` | Facade — stable import path for shell, primitives, previews |
| `orbital-shell` | App shell chrome (layouts, marketing tokens, icons) |
| `orbital-macros` | `#[component_doc]`, route extraction macros |
| `orbital-theme` | Theme provider, density, mode, CSS variable injection |
| `orbital-style` | Style registry and injected component CSS |
| `orbital-motion` | Shared motion tokens and presence transitions |
| `orbital-core-components` | Themed UI components |
| `orbital-base-components` | Headless primitives (structure, semantics, a11y) |
| `orbital-primitives` | Public primitives facade re-exported as `orbital::primitives` |
| `orbital-data` | `Dataset` / `DataRecord` / `DataValue` typed model |
| `orbital-paging` | `Page` / `PageRequest` paging wire types |
| `orbital-markdown` | Markdown → HTML with citations |
| `orbital-datatable`, `orbital-charts`, `orbital-date-pickers`, `orbital-scheduler`, `orbital-tree`, `orbital-discussion` | Feature product crates |
| `orbital-preview-*` | Preview server for component docs and E2E (`:3010`) |
| `component-preview-e2e` | Macro expansion test fixtures |

## Development

**Toolchain:** nightly Rust (see `rust-toolchain.toml`), [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) **≥ 0.3.6**, Node 24+ for Playwright.

Self-hosted LoMT fonts (League Spartan, League Mono, Orbitron) live under [`public/fonts/`](public/fonts/) and are copied into the preview site by cargo-leptos. They are not generated by the build — ensure the `.woff2` files are present locally before running font-related E2E tests.

`wasm-bindgen` is pinned to an exact patch version in the workspace `Cargo.toml` (currently `=0.2.108`) so it matches the `wasm-bindgen-cli` version used during `cargo leptos build`. If you see a schema version mismatch error, align `cargo-leptos`, the workspace pin, and your global `wasm-bindgen-cli` to the same patch version.

Component work uses a three-layer test model — see [docs/component-testing.md](docs/component-testing.md) for conventions, helpers, and `data-testid` rules.

### L1 — Macro / compile

```bash
cargo test -p component-preview-e2e
```

### L2 — Preview host

```bash
cargo leptos watch -p orbital-preview
# http://127.0.0.1:3010/paginator
```

### L3 — Browser E2E

Playwright specs live in `end2end/tests/components/` (~460 component previews + smoke).

**One-shot:**

```bash
cd end2end && npm ci && npx playwright install --with-deps chromium
cargo leptos end-to-end --project orbital-preview
```

**With the server already running:**

```bash
cd end2end && npm test
```

### Release gate (before tagging)

```bash
./scripts/maintainer/preview_release_gate.sh
```

Scans tracked release surface (`ip_release_gate.sh`), then runs full release-mode E2E (~1–2 hours). PR CI runs lean checks only; tag deploy runs the full gate via [`.github/workflows/preview-pages.yml`](.github/workflows/preview-pages.yml).

## Verify

Same commands as [`.github/workflows/ci.yml`](.github/workflows/ci.yml):

```bash
cargo fmt --all -- --check
cargo test -p component-preview-e2e
cargo check --workspace
cargo check -p orbital-core-components --no-default-features
cargo check -p orbital-datatable --no-default-features
cargo check -p orbital-charts --no-default-features
cargo check -p orbital-date-pickers --no-default-features
cargo check -p orbital-scheduler --no-default-features
cargo check -p orbital-tree --no-default-features
cargo clippy --workspace --all-targets -- -D warnings
```

## Documentation

| Doc | Audience |
|-----|----------|
| [`orbital/README.md`](orbital/README.md) | Shell, auth, preview workflow |
| [`orbital-macros/README.md`](orbital-macros/README.md) | `#[component_doc]`, consumer feature flags |
| [`docs/component-testing.md`](docs/component-testing.md) | L1/L2/L3 testing, Playwright helpers |
| Feature crate READMEs | Data table, charts, date pickers, scheduler, discussion |
| `cargo doc -p orbital --open` | API reference (with `preview` features as needed) |

## `#[component_doc]`

See [orbital-macros/README.md](orbital-macros/README.md).

## Publishing preview

One-time setup: **Settings → Pages → Build and deployment** → Source: **GitHub Actions**.

After `preview_release_gate.sh` passes on `main`, push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The preview workflow publishes `target/site-preview/` to **https://unified-field-dev.github.io/orbital/**.

## Contributing

| Doc | Purpose |
|-----|---------|
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Prerequisites, verify commands, PR expectations |
| [`SECURITY.md`](SECURITY.md) | Report vulnerabilities privately |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Community standards |

## License

MIT — see [LICENSE](LICENSE).
