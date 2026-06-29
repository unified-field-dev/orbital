use std::cmp::Ordering;
use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Typed cell value — replaces stringly-typed `HashMap<String, String>`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataValue {
    Text(String),
    Number(f64),
    Bool(bool),
    Date(NaiveDate),
    Category(String),
    Null,
}

/// Column/field data type descriptor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    #[default]
    Text,
    Number,
    Bool,
    Date,
    Category,
}

/// Column type hint for typed comparison (mirrors data table `ColumnType` without coupling).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CompareHint {
    #[default]
    Text,
    Number,
    Date,
    Boolean,
    SingleSelect,
}

impl DataValue {
    /// Human-readable string for display and text-based filtering.
    pub fn display_string(&self) -> String {
        match self {
            Self::Text(s) | Self::Category(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Date(d) => d.format("%Y-%m-%d").to_string(),
            Self::Null => String::new(),
        }
    }

    /// Whether this value is considered empty for filter operators.
    pub fn is_empty_value(&self) -> bool {
        matches!(self, Self::Null) || matches!(self, Self::Text(s) if s.is_empty())
    }

    /// Case-insensitive text for Contains/StartsWith operators.
    pub fn filter_text(&self) -> String {
        self.display_string().to_lowercase()
    }

    /// Typed partial comparison for sorting. Cross-variant or invalid pairs return `None`.
    pub fn partial_cmp_typed(&self, other: &Self, hint: CompareHint) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Null, _) => Some(Ordering::Greater),
            (_, Self::Null) => Some(Ordering::Less),
            (Self::Number(a), Self::Number(b)) if matches!(hint, CompareHint::Number) => {
                a.partial_cmp(b)
            }
            (Self::Date(a), Self::Date(b)) if matches!(hint, CompareHint::Date) => Some(a.cmp(b)),
            (Self::Bool(a), Self::Bool(b)) if matches!(hint, CompareHint::Boolean) => {
                Some(a.cmp(b))
            }
            (Self::Text(a) | Self::Category(a), Self::Text(b) | Self::Category(b))
                if matches!(hint, CompareHint::Text | CompareHint::SingleSelect) =>
            {
                Some(a.to_lowercase().cmp(&b.to_lowercase()))
            }
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            (Self::Date(a), Self::Date(b)) => Some(a.cmp(b)),
            (Self::Bool(a), Self::Bool(b)) => Some(a.cmp(b)),
            (Self::Text(a) | Self::Category(a), Self::Text(b) | Self::Category(b)) => {
                Some(a.to_lowercase().cmp(&b.to_lowercase()))
            }
            _ => None,
        }
    }

    /// Infer a [`DataType`] from this value variant.
    pub fn data_type(&self) -> DataType {
        match self {
            Self::Text(_) => DataType::Text,
            Self::Number(_) => DataType::Number,
            Self::Bool(_) => DataType::Bool,
            Self::Date(_) => DataType::Date,
            Self::Category(_) => DataType::Category,
            Self::Null => DataType::Text,
        }
    }
}

impl From<String> for DataValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for DataValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<f64> for DataValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// Convert a string map into typed text values.
pub fn text_map_from_strings(cells: HashMap<String, String>) -> HashMap<String, DataValue> {
    cells
        .into_iter()
        .map(|(k, v)| (k, DataValue::Text(v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_sort_order() {
        let two = DataValue::Number(2.0);
        let ten = DataValue::Number(10.0);
        assert_eq!(
            two.partial_cmp_typed(&ten, CompareHint::Number),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn null_sorts_last() {
        let text = DataValue::Text("a".into());
        let null = DataValue::Null;
        assert_eq!(
            text.partial_cmp_typed(&null, CompareHint::Text),
            Some(Ordering::Less)
        );
    }
}
