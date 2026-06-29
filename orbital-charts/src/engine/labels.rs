//! Series and slice label resolution.

use leptos::prelude::Callable;

use crate::{LabelLocation, PieSliceData, SeriesDef};

/// Resolve display label for a series at a given surface.
pub fn resolve_series_label(series: &SeriesDef, location: LabelLocation) -> String {
    if let Some(formatter) = series.label_formatter.as_ref() {
        return formatter.run((location,));
    }
    series.label.clone().unwrap_or_else(|| series.id.clone())
}

/// Resolve display label for a pie slice at a given surface.
pub fn resolve_slice_label(slice: &PieSliceData, _location: LabelLocation) -> String {
    slice.label.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::callback::Callback;

    #[test]
    fn prefers_formatter_over_fixed_label() {
        leptos::prelude::Owner::new().with(|| {
            let series = SeriesDef {
                id: "a".into(),
                label: Some("Fixed".into()),
                label_formatter: Some(Callback::new(|(loc,): (LabelLocation,)| format!("{loc:?}"))),
                ..Default::default()
            };
            assert_eq!(
                resolve_series_label(&series, LabelLocation::Legend),
                "Legend"
            );
        });
    }

    #[test]
    fn falls_back_to_id() {
        let series = SeriesDef {
            id: "revenue".into(),
            ..Default::default()
        };
        assert_eq!(
            resolve_series_label(&series, LabelLocation::Tooltip),
            "revenue"
        );
    }
}
