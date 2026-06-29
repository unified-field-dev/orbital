/// Overlay state for loading, empty, and no-results views.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayState {
    None,
    Loading,
    Empty,
    NoResults,
}
