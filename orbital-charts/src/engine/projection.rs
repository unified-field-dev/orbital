//! Dataset projection into render-ready series.

use orbital_data::{ChartFieldBinding, Dataset, ProjectionError};

/// One projected data series ready for scale mapping.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedSeries {
    /// Series identifier (field key or derived id).
    pub id: String,
    /// Display label from schema or field key.
    pub label: String,
    /// Numeric values aligned with projected categories.
    pub data: Vec<f64>,
}

/// Result of projecting a dataset through a field binding.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedChartData {
    /// Category axis labels.
    pub categories: Vec<String>,
    /// One or more value series.
    pub series: Vec<ProjectedSeries>,
}

/// Project a [`Dataset`] into chart-ready series using a [`ChartFieldBinding`].
pub fn project_chart_data(
    dataset: &Dataset,
    binding: &ChartFieldBinding,
) -> Result<ProjectedChartData, ProjectionError> {
    if binding.series_by_field.is_some() {
        return Err(ProjectionError::UnsupportedBinding {
            reason: "series_by_field pivot is not yet supported".into(),
        });
    }

    let x_field =
        binding
            .x_field
            .as_deref()
            .ok_or_else(|| ProjectionError::UnsupportedBinding {
                reason: "x_field is required".into(),
            })?;

    if binding.y_fields.is_empty() {
        return Err(ProjectionError::UnsupportedBinding {
            reason: "at least one y_field is required".into(),
        });
    }

    let categories = dataset.column_as_categories(x_field)?;
    let row_count = categories.len();

    let mut series = Vec::with_capacity(binding.y_fields.len());
    for y_field in &binding.y_fields {
        let data = dataset.column_as_numbers(y_field)?;
        if data.len() != row_count {
            return Err(ProjectionError::LengthMismatch {
                field: y_field.clone(),
                expected: row_count,
                got: data.len(),
            });
        }

        let label = dataset
            .schema
            .fields
            .iter()
            .find(|f| f.key == *y_field)
            .map(|f| f.label.clone())
            .unwrap_or_else(|| y_field.clone());

        series.push(ProjectedSeries {
            id: y_field.clone(),
            label,
            data,
        });
    }

    Ok(ProjectedChartData { categories, series })
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::{DataRecord, DataSchema, DataValue};
    use std::collections::HashMap;

    fn fixture_dataset() -> Dataset {
        let schema = DataSchema::from_text_fields([
            ("quarter".into(), "Quarter".into()),
            ("revenue".into(), "Revenue".into()),
            ("cost".into(), "Cost".into()),
        ]);
        let records = vec![
            DataRecord::new(
                "1",
                HashMap::from([
                    ("quarter".into(), DataValue::Category("Q1".into())),
                    ("revenue".into(), DataValue::Number(100.0)),
                    ("cost".into(), DataValue::Number(60.0)),
                ]),
            ),
            DataRecord::new(
                "2",
                HashMap::from([
                    ("quarter".into(), DataValue::Category("Q2".into())),
                    ("revenue".into(), DataValue::Number(120.0)),
                    ("cost".into(), DataValue::Number(70.0)),
                ]),
            ),
        ];
        Dataset::from_records(schema, records)
    }

    #[test]
    fn project_chart_data_basic() {
        let dataset = fixture_dataset();
        let binding = ChartFieldBinding::new("quarter", vec!["revenue".into(), "cost".into()]);
        let projected = project_chart_data(&dataset, &binding).unwrap();

        assert_eq!(projected.categories, vec!["Q1", "Q2"]);
        assert_eq!(projected.series.len(), 2);
        assert_eq!(projected.series[0].id, "revenue");
        assert_eq!(projected.series[0].label, "Revenue");
        assert_eq!(projected.series[0].data, vec![100.0, 120.0]);
        assert_eq!(projected.series[1].data, vec![60.0, 70.0]);
    }

    #[test]
    fn project_chart_data_rejects_series_by_field() {
        let dataset = fixture_dataset();
        let binding = ChartFieldBinding {
            x_field: Some("quarter".into()),
            y_fields: vec!["revenue".into()],
            series_by_field: Some("region".into()),
            ..Default::default()
        };
        let err = project_chart_data(&dataset, &binding).unwrap_err();
        assert!(matches!(err, ProjectionError::UnsupportedBinding { .. }));
    }

    #[test]
    fn project_chart_data_requires_y_fields() {
        let dataset = fixture_dataset();
        let binding = ChartFieldBinding {
            x_field: Some("quarter".into()),
            ..Default::default()
        };
        let err = project_chart_data(&dataset, &binding).unwrap_err();
        assert!(matches!(err, ProjectionError::UnsupportedBinding { .. }));
    }
}
