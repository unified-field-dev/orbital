use leptos::prelude::*;
use orbital_data::Dataset;

use crate::engine::SortDirection;
use crate::types::{DataTableFilter, SerializedState};

/// Imperative handle for programmatic DataTable actions.
#[derive(Clone)]
pub struct DataTableHandle {
    /// Sort a column programmatically (replaces current sort).
    pub sort_column: Callback<(String, SortDirection), ()>,
    /// Replace the filter model.
    pub set_filter: Callback<(DataTableFilter,), ()>,
    /// Replace the quick-search text.
    pub set_quick_search: Callback<(String,), ()>,
    /// Serialize the current table state to JSON.
    pub export_state: Callback<(), SerializedState>,
    /// Restore table state from a previously exported snapshot.
    pub restore_state: Callback<(SerializedState,), ()>,
    /// Download all matching rows as CSV.
    pub export_csv: Callback<(), ()>,
    /// Scroll the table body so the row with the given id is visible.
    pub scroll_to_row: Callback<(String,), ()>,
    /// Scroll the table body so the column with the given field key is visible.
    pub scroll_to_column: Callback<(String,), ()>,
    /// Return the chart-ready processed dataset (grouped/aggregated/pivoted).
    ///
    /// Reads the live [`RwSignal`] updated on every pipeline run when
    /// [`crate::DataTableFeatures::CHARTS_INTEGRATION`] is enabled; otherwise
    /// returns an empty [`Dataset`].
    pub get_processed_dataset: Callback<(), Dataset>,
}
