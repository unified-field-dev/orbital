//! Nearest-neighbor hit testing for scatter charts.

use crate::PlacedScatterPoint;

/// Hit target returned from pointer lookup.
#[derive(Clone, Debug, PartialEq)]
pub struct HitTarget {
    /// Series identifier.
    pub series_id: String,
    /// Data index within the series.
    pub data_index: usize,
    /// Distance from pointer to point in pixels.
    pub distance: f64,
}

/// Find the nearest scatter point within an optional max radius.
pub fn find_nearest_point(
    px: f64,
    py: f64,
    points: &[PlacedScatterPoint],
    max_radius: Option<f64>,
) -> Option<HitTarget> {
    let mut best: Option<HitTarget> = None;

    for point in points {
        let dx = px - point.px;
        let dy = py - point.py;
        let distance = (dx * dx + dy * dy).sqrt();

        if let Some(max) = max_radius {
            if distance > max {
                continue;
            }
        }

        if best.as_ref().is_none_or(|b| distance < b.distance) {
            best = Some(HitTarget {
                series_id: point.series_id.clone(),
                data_index: point.data_index,
                distance,
            });
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(px: f64, py: f64, series_id: &str, data_index: usize) -> PlacedScatterPoint {
        PlacedScatterPoint {
            px,
            py,
            series_id: series_id.into(),
            data_index,
        }
    }

    #[test]
    fn find_nearest_point_picks_closest() {
        let points = vec![point(10.0, 10.0, "a", 0), point(50.0, 50.0, "a", 1)];
        let hit = find_nearest_point(12.0, 11.0, &points, None).unwrap();
        assert_eq!(hit.data_index, 0);
    }

    #[test]
    fn find_nearest_point_respects_max_radius() {
        let points = vec![point(100.0, 100.0, "a", 0)];
        assert!(find_nearest_point(0.0, 0.0, &points, Some(10.0)).is_none());
        assert!(find_nearest_point(95.0, 100.0, &points, Some(10.0)).is_some());
    }
}
