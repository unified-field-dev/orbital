//! Pie arc layout and SVG path generation.

use std::f64::consts::PI;

use crate::{PieGeometry, PieRadius, PieSliceData};

/// Resolved pie center and radii in plot pixel space.
#[derive(Clone, Debug, PartialEq)]
pub struct PieLayout {
    /// Center x in plot coordinates.
    pub cx: f64,
    /// Center y in plot coordinates.
    pub cy: f64,
    /// Inner radius in pixels.
    pub inner_radius: f64,
    /// Outer radius in pixels.
    pub outer_radius: f64,
    /// Padding angle in degrees.
    pub padding_angle: f64,
    /// Start angle in degrees.
    pub start_angle: f64,
    /// End angle in degrees.
    pub end_angle: f64,
    /// Corner radius for arc corners.
    pub corner_radius: f64,
}

/// Layout for one pie slice including SVG path.
#[derive(Clone, Debug, PartialEq)]
pub struct PieSliceLayout {
    /// Slice index.
    pub index: usize,
    /// Slice data reference fields.
    pub id: String,
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
    /// Start angle in radians.
    pub start_rad: f64,
    /// End angle in radians.
    pub end_rad: f64,
    /// Slice span in degrees.
    pub angle_deg: f64,
    /// SVG path `d` attribute.
    pub path_d: String,
    /// Mid-angle in radians for label placement.
    pub mid_rad: f64,
}

/// Resolve pie geometry from config and plot dimensions.
pub fn resolve_pie_geometry(
    plot_width: f64,
    plot_height: f64,
    geometry: &PieGeometry,
) -> PieLayout {
    PieLayout {
        cx: geometry.cx.resolve_center(plot_width, plot_height, true),
        cy: geometry.cy.resolve_center(plot_width, plot_height, false),
        inner_radius: geometry.inner_radius.resolve(plot_width, plot_height, true),
        outer_radius: geometry.outer_radius.resolve(plot_width, plot_height, true),
        padding_angle: geometry.padding_angle,
        start_angle: geometry.start_angle,
        end_angle: geometry.end_angle,
        corner_radius: geometry.corner_radius,
    }
}

/// Compute slice layouts from slice data and geometry.
pub fn compute_pie_slice_layouts(
    slices: &[PieSliceData],
    layout: &PieLayout,
) -> Vec<PieSliceLayout> {
    let values: Vec<f64> = slices.iter().map(|s| s.value).collect();
    let angles = compute_slice_angles(
        &values,
        layout.padding_angle,
        layout.start_angle,
        layout.end_angle,
    );

    angles
        .into_iter()
        .enumerate()
        .map(|(index, (start_deg, end_deg))| {
            let slice = &slices[index];
            let start_rad = deg_to_rad(start_deg);
            let end_rad = deg_to_rad(end_deg);
            let mid_rad = (start_rad + end_rad) / 2.0;
            let path_d = arc_path_d(
                layout.cx,
                layout.cy,
                layout.inner_radius,
                layout.outer_radius,
                start_rad,
                end_rad,
            );
            PieSliceLayout {
                index,
                id: slice.id.clone(),
                label: slice.label.clone(),
                value: slice.value,
                color: slice.color.clone(),
                start_rad,
                end_rad,
                angle_deg: end_deg - start_deg,
                path_d,
                mid_rad,
            }
        })
        .collect()
}

/// Compute start/end angles in degrees for each value slice.
pub fn compute_slice_angles(
    values: &[f64],
    padding_angle: f64,
    start_angle: f64,
    end_angle: f64,
) -> Vec<(f64, f64)> {
    if values.is_empty() {
        return Vec::new();
    }

    let total: f64 = values.iter().sum();
    if total <= 0.0 {
        return values
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let span = (end_angle - start_angle) / values.len() as f64;
                let s = start_angle + span * i as f64;
                (s, s + span)
            })
            .collect();
    }

    let sweep = end_angle - start_angle;
    let total_padding = padding_angle * values.len() as f64;
    let available = (sweep - total_padding).max(0.0);
    let mut current = start_angle;
    let mut result = Vec::with_capacity(values.len());

    for value in values {
        let slice_angle = (value / total) * available;
        let start = current;
        let end = current + slice_angle;
        result.push((start, end));
        current = end + padding_angle;
    }

    result
}

/// Generate SVG arc path for a donut/pie wedge.
pub fn arc_path_d(
    cx: f64,
    cy: f64,
    inner_r: f64,
    outer_r: f64,
    start_rad: f64,
    end_rad: f64,
) -> String {
    if (end_rad - start_rad).abs() < 1e-9 {
        return String::new();
    }

    let (ox0, oy0) = polar_to_cartesian(cx, cy, outer_r, end_rad);
    let (ox1, oy1) = polar_to_cartesian(cx, cy, outer_r, start_rad);
    let large_arc = if (end_rad - start_rad).abs() > PI {
        1
    } else {
        0
    };

    if inner_r > 0.0 {
        let (ix0, iy0) = polar_to_cartesian(cx, cy, inner_r, start_rad);
        let (ix1, iy1) = polar_to_cartesian(cx, cy, inner_r, end_rad);
        format!(
            "M {ox0} {oy0} A {outer_r} {outer_r} 0 {large_arc} 0 {ox1} {oy1} L {ix0} {iy0} A {inner_r} {inner_r} 0 {large_arc} 1 {ix1} {iy1} Z"
        )
    } else {
        let (ix, iy) = (cx, cy);
        format!("M {ix} {iy} L {ox1} {oy1} A {outer_r} {outer_r} 0 {large_arc} 1 {ox0} {oy0} Z")
    }
}

/// Partial arc path for sweep animation (fraction 0..1 of slice span).
pub fn arc_path_d_sweep(
    cx: f64,
    cy: f64,
    inner_r: f64,
    outer_r: f64,
    start_rad: f64,
    end_rad: f64,
    fraction: f64,
) -> String {
    let fraction = fraction.clamp(0.0, 1.0);
    if fraction <= 0.0 {
        return String::new();
    }
    let partial_end = start_rad + (end_rad - start_rad) * fraction;
    arc_path_d(cx, cy, inner_r, outer_r, start_rad, partial_end)
}

/// Arc label position at mid-angle on the given radius.
pub fn arc_label_position(cx: f64, cy: f64, mid_rad: f64, radius: f64) -> (f64, f64) {
    polar_to_cartesian(cx, cy, radius, mid_rad)
}

/// Resolve arc label radius from config or default midpoint.
pub fn resolve_arc_label_radius(
    plot_width: f64,
    plot_height: f64,
    inner_r: f64,
    outer_r: f64,
    radius: Option<&PieRadius>,
) -> f64 {
    radius
        .map(|r| r.resolve(plot_width, plot_height, true))
        .unwrap_or((inner_r + outer_r) / 2.0)
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

fn polar_to_cartesian(cx: f64, cy: f64, r: f64, angle_rad: f64) -> (f64, f64) {
    (cx + r * angle_rad.cos(), cy + r * angle_rad.sin())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PieSliceData;

    #[test]
    fn compute_slice_angles_four_equal_slices() {
        let angles = compute_slice_angles(&[25.0, 25.0, 25.0, 25.0], 0.0, 0.0, 360.0);
        assert_eq!(angles.len(), 4);
        assert!((angles[0].1 - angles[0].0 - 90.0).abs() < 0.01);
        assert!((angles[3].1 - 360.0).abs() < 0.01);
    }

    #[test]
    fn arc_path_d_produces_non_empty_wedge() {
        let d = arc_path_d(100.0, 100.0, 0.0, 50.0, 0.0, PI / 2.0);
        assert!(d.contains('A'));
        assert!(d.ends_with('Z'));
    }

    #[test]
    fn donut_path_has_inner_arc() {
        let d = arc_path_d(100.0, 100.0, 30.0, 50.0, 0.0, PI);
        assert!(d.matches('A').count() >= 2);
    }

    #[test]
    fn compute_pie_slice_layouts_count_matches_slices() {
        let slices = vec![
            PieSliceData {
                id: "a".into(),
                label: "A".into(),
                value: 40.0,
                color: None,
            },
            PieSliceData {
                id: "b".into(),
                label: "B".into(),
                value: 60.0,
                color: None,
            },
        ];
        let layout = resolve_pie_geometry(200.0, 200.0, &PieGeometry::default());
        let result = compute_pie_slice_layouts(&slices, &layout);
        assert_eq!(result.len(), 2);
        assert!(!result[0].path_d.is_empty());
    }
}
