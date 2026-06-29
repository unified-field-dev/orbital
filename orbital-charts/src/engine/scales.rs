//! Pure scale builders for chart geometry.

use chrono::NaiveDate;

/// Band scale mapping categorical domain to a numeric range.
#[derive(Clone, Debug, PartialEq)]
pub struct BandScale {
    domain: Vec<String>,
    range_start: f64,
    range_end: f64,
    step: f64,
    bandwidth: f64,
    padding_inner: f64,
    padding_outer: f64,
}

impl BandScale {
    /// Create a band scale with inner and outer padding (0.0–1.0).
    ///
    /// Outer padding insets the first and last bands from the range edges so bars
    /// do not sit flush against the y-axis line or the plot boundary.
    pub fn new(domain: Vec<String>, range: (f64, f64), padding_inner: f64) -> Self {
        Self::with_padding(domain, range, padding_inner, padding_inner)
    }

    /// Create a band scale with independent inner and outer padding ratios.
    pub fn with_padding(
        domain: Vec<String>,
        range: (f64, f64),
        padding_inner: f64,
        padding_outer: f64,
    ) -> Self {
        let n = domain.len().max(1) as f64;
        let range_start = range.0.min(range.1);
        let range_end = range.0.max(range.1);
        let padding_inner = padding_inner.clamp(0.0, 1.0);
        let padding_outer = padding_outer.clamp(0.0, 1.0);
        let inner_gaps = padding_inner * (n - 1.0).max(0.0);
        let step = (range_end - range_start) / (n + inner_gaps + padding_outer * 2.0).max(1.0);
        let bandwidth = step * (1.0 - padding_inner);

        Self {
            domain,
            range_start,
            range_end,
            step,
            bandwidth,
            padding_inner,
            padding_outer,
        }
    }

    /// Band width in range units.
    pub fn bandwidth(&self) -> f64 {
        self.bandwidth
    }

    /// Step between band starts.
    pub fn step(&self) -> f64 {
        self.step
    }

    /// Map a category to the center of its band.
    pub fn scale(&self, category: &str) -> Option<f64> {
        let index = self.domain.iter().position(|c| c == category)?;
        self.scale_by_index(index)
    }

    /// Map a domain index to the center of its band.
    pub fn scale_by_index(&self, index: usize) -> Option<f64> {
        if index >= self.domain.len() {
            return None;
        }
        let band_start =
            self.range_start + self.padding_outer * self.step + self.step * index as f64;
        Some(band_start + self.bandwidth / 2.0)
    }

    /// Domain categories in band order.
    pub fn domain(&self) -> &[String] {
        &self.domain
    }

    /// Map a range coordinate to the nearest domain index.
    pub fn index_at(&self, position: f64) -> Option<usize> {
        if self.domain.is_empty() {
            return None;
        }
        let pos = position.clamp(self.range_start, self.range_end);
        let relative = pos - self.range_start - self.padding_outer * self.step;
        if relative < 0.0 {
            return None;
        }
        let idx = (relative / self.step).floor() as usize;
        if idx >= self.domain.len() {
            None
        } else {
            Some(idx)
        }
    }

    /// Fraction (0–1) of full domain for a range position (for zoom pointer anchor).
    pub fn position_to_fraction(&self, position: f64) -> Option<f64> {
        let idx = self.index_at(position)?;
        Some((idx as f64 + 0.5) / self.domain.len() as f64)
    }

    /// Band rectangle `[x, width]` for a domain index in range coordinates.
    pub fn band_rect(&self, index: usize) -> Option<(f64, f64)> {
        if index >= self.domain.len() {
            return None;
        }
        let x = self.range_start + self.padding_outer * self.step + self.step * index as f64;
        Some((x, self.bandwidth))
    }
}

/// Linear scale mapping numeric domain to range.
#[derive(Clone, Debug, PartialEq)]
pub struct LinearScale {
    domain_min: f64,
    domain_max: f64,
    range_start: f64,
    range_end: f64,
}

impl LinearScale {
    /// Create a linear scale.
    pub fn new(domain: (f64, f64), range: (f64, f64)) -> Self {
        Self {
            domain_min: domain.0,
            domain_max: domain.1,
            range_start: range.0,
            range_end: range.1,
        }
    }

    /// Map a domain value to range coordinates.
    pub fn scale(&self, value: f64) -> f64 {
        let domain_span = self.domain_max - self.domain_min;
        if domain_span == 0.0 {
            return (self.range_start + self.range_end) / 2.0;
        }
        let t = (value - self.domain_min) / domain_span;
        self.range_start + t * (self.range_end - self.range_start)
    }

    /// Map a range coordinate back to a domain value.
    pub fn invert(&self, pixel: f64) -> f64 {
        let range_span = self.range_end - self.range_start;
        if range_span == 0.0 {
            return (self.domain_min + self.domain_max) / 2.0;
        }
        let t = (pixel - self.range_start) / range_span;
        self.domain_min + t * (self.domain_max - self.domain_min)
    }
}

/// Logarithmic (base-10) scale.
#[derive(Clone, Debug, PartialEq)]
pub struct LogScale {
    inner: LinearScale,
}

impl LogScale {
    /// Create a log10 scale. Domain values must be positive.
    pub fn new(domain: (f64, f64), range: (f64, f64)) -> Option<Self> {
        if domain.0 <= 0.0 || domain.1 <= 0.0 {
            return None;
        }
        Some(Self {
            inner: LinearScale::new((domain.0.log10(), domain.1.log10()), range),
        })
    }

    /// Map a positive domain value to range coordinates.
    pub fn scale(&self, value: f64) -> Option<f64> {
        if value <= 0.0 {
            return None;
        }
        Some(self.inner.scale(value.log10()))
    }
}

/// Ordinal time scale mapping dates to range positions by index.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeScale {
    domain: Vec<NaiveDate>,
    inner: LinearScale,
}

impl TimeScale {
    /// Create a time scale using ordinal date positions.
    pub fn new(dates: Vec<NaiveDate>, range: (f64, f64)) -> Self {
        let max_index = dates.len().saturating_sub(1).max(1) as f64;
        Self {
            domain: dates,
            inner: LinearScale::new((0.0, max_index), range),
        }
    }

    /// Map a date to range coordinates by ordinal index.
    pub fn scale(&self, date: NaiveDate) -> Option<f64> {
        let index = self.domain.iter().position(|d| *d == date)?;
        Some(self.inner.scale(index as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn band_scale_maps_categories() {
        let scale = BandScale::new(vec!["A".into(), "B".into(), "C".into()], (0.0, 300.0), 0.1);
        assert!(scale.bandwidth() > 0.0);
        let b = scale.scale("B").unwrap();
        assert!((b - 136.76470588235292).abs() < 1e-9);
        assert!(scale.scale("missing").is_none());
    }

    #[test]
    fn band_scale_offsets_first_band_from_range_start() {
        let scale = BandScale::new(vec!["A".into(), "B".into()], (0.0, 200.0), 0.1);
        let center = scale.scale("A").unwrap();
        let band_start = center - scale.bandwidth() / 2.0;
        assert!(band_start > 0.0);
    }

    #[test]
    fn linear_scale_interpolates() {
        let scale = LinearScale::new((0.0, 100.0), (0.0, 200.0));
        assert_eq!(scale.scale(50.0), 100.0);
        assert_eq!(scale.scale(0.0), 0.0);
    }

    #[test]
    fn log_scale_requires_positive_domain() {
        assert!(LogScale::new((0.0, 100.0), (0.0, 100.0)).is_none());
        let scale = LogScale::new((1.0, 1000.0), (0.0, 100.0)).unwrap();
        assert!((scale.scale(10.0).unwrap() - 33.33333333333333).abs() < f64::EPSILON);
        assert!(scale.scale(0.0).is_none());
    }

    #[test]
    fn time_scale_maps_by_index() {
        let dates = vec![
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        ];
        let scale = TimeScale::new(dates.clone(), (0.0, 100.0));
        assert_eq!(scale.scale(dates[1]).unwrap(), 50.0);
    }
}
