/// Errors from datetime boundary conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatetimeError {
    /// Unix timestamp is out of the valid chrono range.
    OutOfRange,
    /// Input string or value variant cannot be parsed as a datetime.
    InvalidInput,
}
