use std::fmt;

use crate::DataType;

/// Errors raised when extracting or projecting chart columns from a [`crate::Dataset`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionError {
    /// Referenced field is not present in the dataset schema.
    UnknownField { field: String },
    /// Cell value variant does not match the expected column type.
    TypeMismatch {
        field: String,
        expected: &'static str,
        got: DataType,
    },
    /// Dataset has no records.
    EmptyDataset,
    /// Extracted column length does not match the expected row count.
    LengthMismatch {
        field: String,
        expected: usize,
        got: usize,
    },
    /// Binding feature not yet supported by the projection engine.
    UnsupportedBinding { reason: String },
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownField { field } => write!(f, "unknown field '{field}'"),
            Self::TypeMismatch {
                field,
                expected,
                got,
            } => write!(f, "field '{field}' expected {expected}, got {got:?}"),
            Self::EmptyDataset => write!(f, "dataset has no records"),
            Self::LengthMismatch {
                field,
                expected,
                got,
            } => write!(
                f,
                "field '{field}' length mismatch: expected {expected}, got {got}"
            ),
            Self::UnsupportedBinding { reason } => write!(f, "unsupported binding: {reason}"),
        }
    }
}

impl std::error::Error for ProjectionError {}
