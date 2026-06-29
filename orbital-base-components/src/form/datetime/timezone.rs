#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatetimeTimezone {
    Local,
    Utc,
    /// Fixed offset in seconds from UTC.
    FixedOffset(i32),
}
