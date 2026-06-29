use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::projection::ProjectionError;
use crate::{DataRecord, DataSchema, DataValue};

/// The shared shape. Tables render it; charts build series by field key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    pub schema: DataSchema,
    pub records: Vec<DataRecord>,
}

impl Dataset {
    pub fn new(schema: DataSchema, records: Vec<DataRecord>) -> Self {
        Self { schema, records }
    }

    pub fn from_records(schema: DataSchema, records: Vec<DataRecord>) -> Self {
        Self::new(schema, records)
    }

    /// Build a dataset from text field definitions and `(id, cells)` rows.
    pub fn from_text_rows(
        fields: impl IntoIterator<Item = (String, String)>,
        rows: impl IntoIterator<Item = (String, HashMap<String, String>)>,
    ) -> Self {
        let schema = DataSchema::from_text_fields(fields);
        let records = rows
            .into_iter()
            .map(|(id, cells)| DataRecord::from_text_map(id, cells))
            .collect();
        Self::new(schema, records)
    }

    pub fn field_keys(&self) -> impl Iterator<Item = &str> {
        self.schema.fields.iter().map(|f| f.key.as_str())
    }

    /// Whether `field` is declared in the dataset schema.
    pub fn has_field(&self, field: &str) -> bool {
        self.schema.fields.iter().any(|f| f.key == field)
    }

    /// Extract a numeric column for chart value axes.
    ///
    /// Accepts [`DataValue::Number`]. [`DataValue::Null`] maps to `f64::NAN` as a gap.
    pub fn column_as_numbers(&self, field: &str) -> Result<Vec<f64>, ProjectionError> {
        if self.records.is_empty() {
            return Err(ProjectionError::EmptyDataset);
        }
        if !self.has_field(field) {
            return Err(ProjectionError::UnknownField {
                field: field.to_string(),
            });
        }

        self.records
            .iter()
            .map(|record| match record.get(field) {
                None | Some(DataValue::Null) => Ok(f64::NAN),
                Some(DataValue::Number(n)) => Ok(*n),
                Some(other) => Err(ProjectionError::TypeMismatch {
                    field: field.to_string(),
                    expected: "number",
                    got: other.data_type(),
                }),
            })
            .collect()
    }

    /// Extract a categorical column for band/point axes.
    ///
    /// Accepts text, category, date, number, and bool variants (stringified).
    pub fn column_as_categories(&self, field: &str) -> Result<Vec<String>, ProjectionError> {
        if self.records.is_empty() {
            return Err(ProjectionError::EmptyDataset);
        }
        if !self.has_field(field) {
            return Err(ProjectionError::UnknownField {
                field: field.to_string(),
            });
        }

        self.records
            .iter()
            .map(|record| match record.get(field) {
                None | Some(DataValue::Null) => Ok(String::new()),
                Some(value) => Ok(value.display_string()),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn sample_dataset() -> Dataset {
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
    fn column_as_numbers_happy_path() {
        let dataset = sample_dataset();
        let nums = dataset.column_as_numbers("revenue").unwrap();
        assert_eq!(nums, vec![100.0, 120.0]);
    }

    #[test]
    fn column_as_numbers_null_becomes_nan() {
        let mut dataset = sample_dataset();
        dataset.records[0]
            .values
            .insert("revenue".into(), DataValue::Null);
        let nums = dataset.column_as_numbers("revenue").unwrap();
        assert!(nums[0].is_nan());
        assert_eq!(nums[1], 120.0);
    }

    #[test]
    fn column_as_numbers_type_mismatch() {
        let dataset = sample_dataset();
        let err = dataset.column_as_numbers("quarter").unwrap_err();
        assert!(matches!(
            err,
            ProjectionError::TypeMismatch {
                field,
                expected: "number",
                ..
            } if field == "quarter"
        ));
    }

    #[test]
    fn column_as_numbers_unknown_field() {
        let dataset = sample_dataset();
        let err = dataset.column_as_numbers("missing").unwrap_err();
        assert!(matches!(err, ProjectionError::UnknownField { .. }));
    }

    #[test]
    fn column_as_numbers_empty_dataset() {
        let dataset = Dataset::default();
        let err = dataset.column_as_numbers("revenue").unwrap_err();
        assert!(matches!(err, ProjectionError::EmptyDataset));
    }

    #[test]
    fn column_as_categories_happy_path() {
        let dataset = sample_dataset();
        let cats = dataset.column_as_categories("quarter").unwrap();
        assert_eq!(cats, vec!["Q1", "Q2"]);
    }

    #[test]
    fn has_field() {
        let dataset = sample_dataset();
        assert!(dataset.has_field("quarter"));
        assert!(!dataset.has_field("missing"));
    }
}
