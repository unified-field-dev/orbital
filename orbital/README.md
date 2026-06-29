# Orbital

Leptos UI component library.

## Design language

Orbital defines spacing tokens, typography presets, elevation ramps, and surface
materials authors compose through shared components. Run the preview server to
explore live component docs at `/orbital/{slug}`.

## Quick start

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

## Composition

- **Shell** — [`OrbitalTemplate`] wraps apps with theme and layout primitives.
- **Auth** — [`AuthContext`] + [`init_auth_resource`] for session state; the preview catalog uses a stub, while integrators wire a real session server function.
- **Preview catalog** — `#[component_doc]` emits [`PreviewRegistration`] entries; run `cargo leptos watch -p orbital-preview` and browse `/orbital/{slug}` on `:3010`.

Capability flags on feature crates (for example `DataTableFeatures`, `ChartFeatures`) express optional behavior — not license-tier suffixes on type names.

See the [repository README](../README.md) for workspace layout and testing.
