//! Tick generation and default label formatting.

/// Approximate number of ticks for a linear value axis.
pub const DEFAULT_TICK_COUNT: usize = 6;

/// Generate evenly spaced tick values for a linear domain.
pub fn compute_linear_ticks(min: f64, max: f64, count: usize) -> Vec<f64> {
    let count = count.max(2);
    if !min.is_finite() || !max.is_finite() {
        return vec![0.0, 1.0];
    }
    if (max - min).abs() < f64::EPSILON {
        return vec![min];
    }
    let step = (max - min) / (count - 1) as f64;
    (0..count).map(|i| min + step * i as f64).collect()
}

/// Compute "nice" linear ticks using a simple step rounding heuristic.
pub fn compute_nice_linear_ticks(min: f64, max: f64, target_count: usize) -> Vec<f64> {
    if !min.is_finite() || !max.is_finite() {
        return vec![0.0, 1.0];
    }
    if (max - min).abs() < f64::EPSILON {
        return vec![min];
    }

    let range = nice_num(max - min, false);
    let step = nice_num(range / (target_count - 1).max(1) as f64, true);
    let nice_min = (min / step).floor() * step;
    let nice_max = (max / step).ceil() * step;

    let mut ticks = Vec::new();
    let mut value = nice_min;
    while value <= nice_max + step * 0.5 {
        if value >= min - step * 0.5 && value <= max + step * 0.5 {
            ticks.push(value);
        }
        value += step;
    }
    if ticks.is_empty() {
        return compute_linear_ticks(min, max, target_count);
    }
    ticks
}

fn nice_num(value: f64, round: bool) -> f64 {
    if value <= 0.0 || !value.is_finite() {
        return 1.0;
    }
    let exponent = value.log10().floor();
    let fraction = value / 10_f64.powf(exponent);
    let nice_fraction = if round {
        if fraction < 1.5 {
            1.0
        } else if fraction < 3.0 {
            2.0
        } else if fraction < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * 10_f64.powf(exponent)
}

/// Default compact numeric tick label (e.g. 1200 → "1k").
pub fn default_tick_format(value: f64) -> String {
    let abs = value.abs();
    if abs >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.0}k", value / 1_000.0)
    } else if (value - value.round()).abs() < f64::EPSILON {
        format!("{:.0}", value)
    } else {
        format!("{:.1}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_linear_ticks_spans_domain() {
        let ticks = compute_linear_ticks(0.0, 100.0, 5);
        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0], 0.0);
        assert_eq!(ticks[4], 100.0);
    }

    #[test]
    fn compute_nice_linear_ticks_non_empty() {
        let ticks = compute_nice_linear_ticks(23.0, 87.0, 5);
        assert!(!ticks.is_empty());
        assert!(ticks.first().unwrap() <= &30.0);
        assert!(ticks.last().unwrap() >= &80.0);
    }

    #[test]
    fn default_tick_format_compacts_thousands() {
        assert_eq!(default_tick_format(420_000.0), "420k");
        assert_eq!(default_tick_format(1_200_000.0), "1.2M");
    }
}
