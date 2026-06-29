use leptos::prelude::*;
use orbital_base_components::TransferListItem;

/// Configuration for [`TransferList`] panel data and feature flags.
#[derive(Clone, Copy)]
pub struct TransferListConfig {
    /// Items available to move into the right panel.
    pub left: RwSignal<Vec<TransferListItem>>,
    /// Items already moved to the right panel.
    pub right: RwSignal<Vec<TransferListItem>>,
    /// When true, each panel shows select-all and selected counters.
    pub enhanced: Signal<bool>,
    /// When true, show move-all buttons between panels.
    pub show_move_all: Signal<bool>,
}

impl TransferListConfig {
    pub fn basic(
        left: RwSignal<Vec<TransferListItem>>,
        right: RwSignal<Vec<TransferListItem>>,
    ) -> Self {
        Self {
            left,
            right,
            enhanced: Signal::from(false),
            show_move_all: Signal::from(true),
        }
    }
}

/// Payload emitted when items move between panels.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferListChange {
    pub left_count: usize,
    pub right_count: usize,
}
