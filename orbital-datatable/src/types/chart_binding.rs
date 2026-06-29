use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};

/// Live chart-data context for [`DataTableFeatures::CHARTS_INTEGRATION`].
///
/// Provided by [`crate::DataTable`] when the integration flag is set. Downstream
/// chart renderers subscribe to [`Self::dataset`] and use schema hints to build
/// a [`ChartFieldBinding`] without importing `orbital-charts`.
#[derive(Clone, Copy)]
pub struct ChartBinding {
    /// Live processed dataset; updated after every pipeline run when integration is enabled.
    pub dataset: RwSignal<Dataset>,
    /// Suggested category / x-axis field (first visible Text/SingleSelect/Date column).
    pub x_field: Signal<Option<String>>,
    /// Suggested value / y-axis fields (visible Number columns).
    pub y_fields: Signal<Vec<String>>,
}

impl ChartBinding {
    /// Build a [`ChartFieldBinding`] from current schema hints when both axes are available.
    pub fn suggested_field_binding(&self) -> Option<ChartFieldBinding> {
        let x = self.x_field.get()?;
        let ys = self.y_fields.get();
        if ys.is_empty() {
            return None;
        }
        Some(ChartFieldBinding::new(x, ys))
    }
}
