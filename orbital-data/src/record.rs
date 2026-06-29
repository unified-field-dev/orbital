use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{text_map_from_strings, DataValue};

/// One row of typed values, addressable by field key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: String,
    pub values: HashMap<String, DataValue>,
}

impl DataRecord {
    pub fn new(id: impl Into<String>, values: HashMap<String, DataValue>) -> Self {
        Self {
            id: id.into(),
            values,
        }
    }

    /// Build a record from a string cell map (preview/demo convenience).
    pub fn from_text_map(id: impl Into<String>, cells: HashMap<String, String>) -> Self {
        Self {
            id: id.into(),
            values: text_map_from_strings(cells),
        }
    }

    pub fn get(&self, field: &str) -> Option<&DataValue> {
        self.values.get(field)
    }
}
