//! Shared typed dataset primitives for Orbital tables and charts.
//!
//! This crate provides [`Dataset`], [`DataRecord`], and [`DataValue`] — the
//! canonical data model shared by `orbital-datatable` and `orbital-charts`.
//! Because it has no UI or framework dependencies, any crate in the workspace
//! can depend on it without introducing cycles.
//!
//! # Example
//!
//! ```rust
//! use std::collections::HashMap;
//! use orbital_data::{DataRecord, DataSchema, DataValue, Dataset, FieldDef};
//!
//! let schema = DataSchema::new(vec![
//!     FieldDef::text("name", "Name"),
//!     FieldDef::text("role", "Role"),
//! ]);
//! let record = DataRecord::new(
//!     "1",
//!     HashMap::from([
//!         ("name".into(), DataValue::Text("Ada".into())),
//!         ("role".into(), DataValue::Text("Admin".into())),
//!     ]),
//! );
//! let dataset = Dataset::from_records(schema, vec![record]);
//! assert_eq!(dataset.records.len(), 1);
//! ```

mod binding;
mod dataset;
mod projection;
mod record;
mod schema;
mod value;

pub use binding::ChartFieldBinding;
pub use dataset::Dataset;
pub use projection::ProjectionError;
pub use record::DataRecord;
pub use schema::{DataSchema, FieldDef};
pub use value::{text_map_from_strings, CompareHint, DataType, DataValue};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn display_string_covers_variants() {
        assert_eq!(DataValue::Text("hello".into()).display_string(), "hello");
        assert_eq!(DataValue::Number(42.5).display_string(), "42.5");
        assert_eq!(DataValue::Bool(true).display_string(), "true");
        assert_eq!(DataValue::Null.display_string(), "");
    }

    #[test]
    fn from_text_map_and_rows() {
        let record =
            DataRecord::from_text_map("a", HashMap::from([("name".into(), "Alpha".into())]));
        assert_eq!(record.get("name").unwrap().display_string(), "Alpha");

        let dataset = Dataset::from_text_rows(
            [("name".into(), "Name".into())],
            [("1".into(), HashMap::from([("name".into(), "Ada".into())]))],
        );
        assert_eq!(dataset.records.len(), 1);
        assert_eq!(dataset.schema.fields.len(), 1);
    }
}
