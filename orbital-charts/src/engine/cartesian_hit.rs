//! Cartesian pointer hit testing.

use crate::engine::{find_nearest_point, BandScale, BarGeometry, HitTarget};

/// Find the bar containing a plot-space pointer, if any.
pub fn find_bar_at_pointer(bars: &[BarGeometry], px: f64, py: f64) -> Option<HitTarget> {
    let mut best: Option<HitTarget> = None;

    for bar in bars {
        if px >= bar.x && px <= bar.x + bar.width && py >= bar.y && py <= bar.y + bar.height {
            let cx = bar.x + bar.width / 2.0;
            let cy = bar.y + bar.height / 2.0;
            let distance = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
            if best.as_ref().is_none_or(|b| distance < b.distance) {
                best = Some(HitTarget {
                    series_id: bar.series_id.clone(),
                    data_index: bar.data_index,
                    distance,
                });
            }
        }
    }

    best
}

/// Map a plot x coordinate to a band-scale category index.
pub fn category_index_at_x(scale: &BandScale, px: f64) -> Option<usize> {
    scale.index_at(px)
}

/// Find nearest line marker point within radius.
pub fn find_nearest_marker(
    px: f64,
    py: f64,
    markers: &[(f64, f64, String, usize)],
    max_radius: f64,
) -> Option<HitTarget> {
    let points: Vec<crate::PlacedScatterPoint> = markers
        .iter()
        .map(|(x, y, series_id, data_index)| crate::PlacedScatterPoint {
            px: *x,
            py: *y,
            series_id: series_id.clone(),
            data_index: *data_index,
        })
        .collect();
    find_nearest_point(px, py, &points, Some(max_radius))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bar() -> BarGeometry {
        BarGeometry {
            x: 10.0,
            y: 20.0,
            width: 20.0,
            height: 30.0,
            series_id: "s1".into(),
            data_index: 1,
            value: 5.0,
        }
    }

    #[test]
    fn finds_bar_under_pointer() {
        let bars = vec![sample_bar()];
        let hit = find_bar_at_pointer(&bars, 15.0, 25.0).unwrap();
        assert_eq!(hit.series_id, "s1");
        assert_eq!(hit.data_index, 1);
    }

    #[test]
    fn misses_outside_bar() {
        let bars = vec![sample_bar()];
        assert!(find_bar_at_pointer(&bars, 0.0, 0.0).is_none());
    }

    #[test]
    fn category_index_from_band_scale() {
        let scale = BandScale::new(vec!["A".into(), "B".into(), "C".into()], (0.0, 300.0), 0.1);
        let idx = category_index_at_x(&scale, scale.scale("B").unwrap()).unwrap();
        assert_eq!(idx, 1);
    }

    #[allow(unused_imports)]
    use crate::ChartOrientation as _;
}
