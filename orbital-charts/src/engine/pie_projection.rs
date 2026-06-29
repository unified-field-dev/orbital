//! Dataset projection for pie charts.

use orbital_data::{ChartFieldBinding, Dataset, ProjectionError};

use crate::PieSliceData;
use crate::ProjectedPieData;

/// Project a [`Dataset`] into pie slice data.
pub fn project_pie_data(
    dataset: &Dataset,
    binding: &ChartFieldBinding,
) -> Result<ProjectedPieData, ProjectionError> {
    let value_field =
        binding
            .y_fields
            .first()
            .ok_or_else(|| ProjectionError::UnsupportedBinding {
                reason: "at least one y_field (value field) is required for pie charts".into(),
            })?;

    let values = dataset.column_as_numbers(value_field)?;
    let row_count = values.len();

    let labels: Vec<String> = if let Some(label_field) = &binding.label_field {
        dataset.column_as_categories(label_field)?
    } else if let Some(x_field) = &binding.x_field {
        dataset.column_as_categories(x_field)?
    } else {
        (0..row_count)
            .map(|i| {
                dataset
                    .records
                    .get(i)
                    .map(|r| r.id.clone())
                    .unwrap_or_else(|| i.to_string())
            })
            .collect()
    };

    if labels.len() != row_count {
        return Err(ProjectionError::LengthMismatch {
            field: "label".into(),
            expected: row_count,
            got: labels.len(),
        });
    }

    let mut slices = Vec::with_capacity(row_count);
    for (i, (&value, label)) in values.iter().zip(labels.iter()).enumerate() {
        if value < 0.0 {
            return Err(ProjectionError::UnsupportedBinding {
                reason: format!("pie values must be non-negative, got {value}"),
            });
        }
        let id = dataset
            .records
            .get(i)
            .map(|r| r.id.clone())
            .unwrap_or_else(|| i.to_string());
        slices.push(PieSliceData {
            id,
            label: label.clone(),
            value,
            color: None,
        });
    }

    Ok(ProjectedPieData {
        series_id: value_field.clone(),
        slices,
    })
}

/// Build pie slices from inline numeric data and category labels.
pub fn project_pie_inline(
    categories: &[String],
    values: &[f64],
    series_id: &str,
) -> Result<ProjectedPieData, ProjectionError> {
    if categories.len() != values.len() {
        return Err(ProjectionError::LengthMismatch {
            field: "inline pie data".into(),
            expected: categories.len(),
            got: values.len(),
        });
    }

    let slices: Vec<PieSliceData> = categories
        .iter()
        .zip(values.iter())
        .enumerate()
        .map(|(i, (label, &value))| {
            if value < 0.0 {
                return Err(ProjectionError::UnsupportedBinding {
                    reason: format!("pie values must be non-negative, got {value}"),
                });
            }
            Ok(PieSliceData {
                id: i.to_string(),
                label: label.clone(),
                value,
                color: None,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ProjectedPieData {
        series_id: series_id.to_string(),
        slices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::{DataRecord, DataSchema, DataValue};
    use std::collections::HashMap;

    fn pie_dataset() -> Dataset {
        let schema = DataSchema::from_text_fields([
            ("segment".into(), "Segment".into()),
            ("share".into(), "Share".into()),
        ]);
        let records = vec![
            DataRecord::new(
                "1",
                HashMap::from([
                    ("segment".into(), DataValue::Category("Alpha".into())),
                    ("share".into(), DataValue::Number(40.0)),
                ]),
            ),
            DataRecord::new(
                "2",
                HashMap::from([
                    ("segment".into(), DataValue::Category("Beta".into())),
                    ("share".into(), DataValue::Number(30.0)),
                ]),
            ),
            DataRecord::new(
                "3",
                HashMap::from([
                    ("segment".into(), DataValue::Category("Gamma".into())),
                    ("share".into(), DataValue::Number(20.0)),
                ]),
            ),
            DataRecord::new(
                "4",
                HashMap::from([
                    ("segment".into(), DataValue::Category("Delta".into())),
                    ("share".into(), DataValue::Number(10.0)),
                ]),
            ),
        ];
        Dataset::from_records(schema, records)
    }

    #[test]
    fn project_pie_data_four_slices() {
        let dataset = pie_dataset();
        let binding = ChartFieldBinding {
            label_field: Some("segment".into()),
            y_fields: vec!["share".into()],
            ..Default::default()
        };
        let projected = project_pie_data(&dataset, &binding).unwrap();
        assert_eq!(projected.slices.len(), 4);
        assert!((projected.slices[0].value - 40.0).abs() < f64::EPSILON);
    }

    #[test]
    fn project_pie_inline_matches_categories() {
        let cats = vec!["A".into(), "B".into()];
        let vals = vec![60.0, 40.0];
        let projected = project_pie_inline(&cats, &vals, "pie").unwrap();
        assert_eq!(projected.slices.len(), 2);
    }

    #[test]
    fn project_pie_rejects_negative_values() {
        let cats = vec!["A".into()];
        let vals = vec![-1.0];
        assert!(project_pie_inline(&cats, &vals, "pie").is_err());
    }
}
