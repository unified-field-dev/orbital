use serde::{Deserialize, Serialize};

use crate::DataType;

/// Shared field descriptor — binding key for table columns and chart series.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDef {
    pub key: String,
    pub label: String,
    pub data_type: DataType,
}

impl FieldDef {
    pub fn new(key: impl Into<String>, label: impl Into<String>, data_type: DataType) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            data_type,
        }
    }

    pub fn text(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(key, label, DataType::Text)
    }
}

/// Schema describing the fields in a [`crate::Dataset`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataSchema {
    pub fields: Vec<FieldDef>,
}

impl DataSchema {
    pub fn new(fields: Vec<FieldDef>) -> Self {
        Self { fields }
    }

    pub fn from_text_fields(fields: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            fields: fields
                .into_iter()
                .map(|(key, label)| FieldDef::text(key, label))
                .collect(),
        }
    }
}
