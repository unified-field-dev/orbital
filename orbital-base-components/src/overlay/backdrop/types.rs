use leptos::prelude::*;

/// How a backdrop scrim covers the viewport.
#[derive(Clone, Copy, Default)]
pub enum BackdropMode {
    /// Uniform dim across the full viewport.
    #[default]
    Full,
    /// Dim everything except a padded cutout around `anchor_id`.
    Spotlight {
        anchor_id: Signal<Option<String>>,
        padding: u32,
    },
}

/// Resolved cutout rectangle for spotlight backdrops.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotlightRect {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
}
