//! Dataset projection for scatter charts.

use orbital_data::{ChartFieldBinding, Dataset, ProjectionError};

use crate::ProjectedScatterData;
use crate::{ScatterPoint, ScatterSeriesData, SeriesDef};

/// Project a [`Dataset`] into scatter series using a field binding.
pub fn project_scatter_data(
    dataset: &Dataset,
    binding: &ChartFieldBinding,
) -> Result<ProjectedScatterData, ProjectionError> {
    let x_field =
        binding
            .x_field
            .as_deref()
            .ok_or_else(|| ProjectionError::UnsupportedBinding {
                reason: "x_field is required for scatter charts".into(),
            })?;
    let y_field = binding
        .y_fields
        .first()
        .ok_or_else(|| ProjectionError::UnsupportedBinding {
            reason: "at least one y_field is required for scatter charts".into(),
        })?;

    let x_values = dataset.column_as_numbers(x_field)?;
    let y_values = dataset.column_as_numbers(y_field)?;
    let row_count = x_values.len();

    if y_values.len() != row_count {
        return Err(ProjectionError::LengthMismatch {
            field: y_field.clone(),
            expected: row_count,
            got: y_values.len(),
        });
    }

    let z_values: Option<Vec<f64>> = binding
        .size_field
        .as_deref()
        .map(|f| dataset.column_as_numbers(f))
        .transpose()?;

    if let Some(ref z) = z_values {
        if z.len() != row_count {
            return Err(ProjectionError::LengthMismatch {
                field: binding.size_field.clone().unwrap_or_default(),
                expected: row_count,
                got: z.len(),
            });
        }
    }

    let points: Vec<ScatterPoint> = (0..row_count)
        .map(|i| {
            let id = resolve_point_id(dataset, binding, i);
            ScatterPoint {
                x: x_values[i],
                y: y_values[i],
                id,
                z: z_values.as_ref().map(|z| z[i]),
            }
        })
        .collect();

    let label = dataset
        .schema
        .fields
        .iter()
        .find(|f| f.key == *y_field)
        .map(|f| f.label.clone())
        .unwrap_or_else(|| y_field.clone());

    Ok(ProjectedScatterData {
        series: vec![ScatterSeriesData {
            series_id: y_field.clone(),
            label,
            points,
            x_axis_id: "x".into(),
            y_axis_id: "y".into(),
            color: None,
            marker_size: 4.0,
        }],
    })
}

/// Project scatter from explicit series definitions.
pub fn project_scatter_series(
    series: &[SeriesDef],
) -> Result<ProjectedScatterData, ProjectionError> {
    let mut result = Vec::new();
    for s in series {
        if let Some(points) = &s.scatter_data {
            if points.is_empty() {
                continue;
            }
            result.push(ScatterSeriesData {
                series_id: s.id.clone(),
                label: s.label.clone().unwrap_or_else(|| s.id.clone()),
                points: points.clone(),
                x_axis_id: s.x_axis_id.clone().unwrap_or_else(|| "x".into()),
                y_axis_id: s.y_axis_id.clone().unwrap_or_else(|| "y".into()),
                color: s.color.clone(),
                marker_size: s.marker_size.unwrap_or(4.0),
            });
        }
    }

    if result.is_empty() {
        return Err(ProjectionError::UnsupportedBinding {
            reason: "no scatter_data found in series".into(),
        });
    }

    Ok(ProjectedScatterData { series: result })
}

fn resolve_point_id(dataset: &Dataset, binding: &ChartFieldBinding, index: usize) -> String {
    if let Some(id_field) = &binding.id_field {
        if let Some(record) = dataset.records.get(index) {
            if let Some(value) = record.values.get(id_field) {
                return value.display_string();
            }
        }
    }
    dataset
        .records
        .get(index)
        .map(|r| r.id.clone())
        .unwrap_or_else(|| index.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::{DataRecord, DataSchema, DataValue};
    use std::collections::HashMap;

    fn scatter_dataset() -> Dataset {
        let schema = DataSchema::from_text_fields([
            ("height".into(), "Height".into()),
            ("weight".into(), "Weight".into()),
            ("subject".into(), "Subject".into()),
        ]);
        let records = vec![
            DataRecord::new(
                "r1",
                HashMap::from([
                    ("height".into(), DataValue::Number(170.0)),
                    ("weight".into(), DataValue::Number(65.0)),
                    ("subject".into(), DataValue::Text("A".into())),
                ]),
            ),
            DataRecord::new(
                "r2",
                HashMap::from([
                    ("height".into(), DataValue::Number(180.0)),
                    ("weight".into(), DataValue::Number(80.0)),
                    ("subject".into(), DataValue::Text("B".into())),
                ]),
            ),
        ];
        Dataset::from_records(schema, records)
    }

    #[test]
    fn project_scatter_data_extracts_points() {
        let dataset = scatter_dataset();
        let binding = ChartFieldBinding {
            x_field: Some("height".into()),
            y_fields: vec!["weight".into()],
            id_field: Some("subject".into()),
            ..Default::default()
        };
        let projected = project_scatter_data(&dataset, &binding).unwrap();
        assert_eq!(projected.series.len(), 1);
        assert_eq!(projected.series[0].points.len(), 2);
        assert_eq!(projected.series[0].points[0].id, "A");
        assert!((projected.series[0].points[0].x - 170.0).abs() < f64::EPSILON);
    }

    #[test]
    fn project_scatter_series_inline() {
        let series = vec![SeriesDef {
            id: "s1".into(),
            label: Some("Series".into()),
            scatter_data: Some(vec![ScatterPoint {
                x: 1.0,
                y: 2.0,
                id: "p1".into(),
                z: None,
            }]),
            ..Default::default()
        }];
        let projected = project_scatter_series(&series).unwrap();
        assert_eq!(projected.series[0].points.len(), 1);
    }
}
