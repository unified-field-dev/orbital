//! Preview render mode for component gallery pages.

/// Controls how `OrbitalComponentView` renders a registered preview.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PreviewRenderMode {
    /// Full gallery page: doc tabs, preview cards, and extra examples.
    #[default]
    Full,
    /// Default example only — no doc panel, tabs, or preview cards.
    BareDefault,
}
