# component-preview-e2e

Integration tests for `#[component_doc]` preview macro expansion.

Runs against fixture components in `src/fixtures.rs`.

- **L1 (this crate):** `cargo test -p component-preview-e2e`
- **L3 (browser):** `cargo leptos end-to-end --project orbital-preview` — see [repository README](../README.md#development)
