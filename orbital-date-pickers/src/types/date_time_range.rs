use orbital_base_components::OrbitalDateTime;

/// Inclusive datetime range for range pickers and shortcuts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTimeRange {
    pub start: OrbitalDateTime,
    pub end: OrbitalDateTime,
}

impl DateTimeRange {
    pub fn new(start: OrbitalDateTime, end: OrbitalDateTime) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, dt: OrbitalDateTime) -> bool {
        dt >= self.start && dt <= self.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Returns a range with `start <= end`, swapping endpoints when inverted.
    pub fn normalized(self) -> Self {
        if self.start <= self.end {
            self
        } else {
            Self {
                start: self.end,
                end: self.start,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DateTimeRange;
    use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};

    #[test]
    fn contains_and_overlaps() {
        let tz = DatetimeTimezone::Utc;
        let start = OrbitalDateTime::try_from_unix_seconds(100, tz).expect("valid");
        let mid = OrbitalDateTime::try_from_unix_seconds(200, tz).expect("valid");
        let end = OrbitalDateTime::try_from_unix_seconds(300, tz).expect("valid");
        let range = DateTimeRange::new(start, end);
        assert!(range.contains(mid));
        assert!(!range.contains(OrbitalDateTime::try_from_unix_seconds(400, tz).expect("valid")));

        let other = DateTimeRange::new(
            OrbitalDateTime::try_from_unix_seconds(250, tz).expect("valid"),
            OrbitalDateTime::try_from_unix_seconds(350, tz).expect("valid"),
        );
        assert!(range.overlaps(&other));
    }

    #[test]
    fn normalized_swaps_inverted_endpoints() {
        let tz = DatetimeTimezone::Utc;
        let early = OrbitalDateTime::try_from_unix_seconds(100, tz).expect("valid");
        let late = OrbitalDateTime::try_from_unix_seconds(300, tz).expect("valid");
        let inverted = DateTimeRange::new(late, early).normalized();
        assert_eq!(inverted.start, early);
        assert_eq!(inverted.end, late);
    }
}
