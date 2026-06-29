//! Analog clock face geometry for time picker dial layouts.

pub const DEFAULT_FACE_SIZE: f64 = 260.0;
pub const DEFAULT_MARKER_SIZE: f64 = 36.0;

/// Angle in radians for marker `index` of `count` evenly spaced labels (12 o'clock = index 0).
pub fn marker_angle(index: usize, count: usize) -> f64 {
    (index as f64 / count as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2
}

/// Marker center offset from face center as a fraction of face size (for `calc(50% + …)`).
pub fn marker_position_fraction(index: usize, count: usize, inner: bool) -> (f64, f64) {
    let angle = marker_angle(index, count);
    let radius_fraction = marker_radius_fraction(inner);
    (angle.cos() * radius_fraction, angle.sin() * radius_fraction)
}

/// CSS `left` / `top` values positioning a marker centered on the dial ring.
pub fn marker_position_style(index: usize, count: usize, inner: bool) -> (String, String) {
    let (xf, yf) = marker_position_fraction(index, count, inner);
    (
        format!("calc(50% + {xf:.6} * var(--orb-clock-face-size))"),
        format!("calc(50% + {yf:.6} * var(--orb-clock-face-size))"),
    )
}

fn marker_radius_fraction(inner: bool) -> f64 {
    let outer = (DEFAULT_FACE_SIZE - DEFAULT_MARKER_SIZE - 2.0) / 2.0 / DEFAULT_FACE_SIZE;
    if inner {
        outer * 0.65
    } else {
        outer
    }
}

fn get_angle_value(step: f64, offset_x: f64, offset_y: f64, face_size: f64) -> (u32, f64) {
    let center = face_size / 2.0;
    let cx = 0.0;
    let cy = -center;

    let x = offset_x - center;
    let y = offset_y - center;

    let mut deg = (f64::atan2(cx, cy) - f64::atan2(x, y)).to_degrees();
    deg = (deg / step).round() * step;
    deg %= 360.0;
    if deg < 0.0 {
        deg += 360.0;
    }

    let value = (deg / step).floor() as u32;
    let distance = (x * x + y * y).sqrt();
    (value, distance)
}

/// Resolve hour from pointer coordinates on the clock face.
pub fn get_hours_from_point(
    offset_x: f64,
    offset_y: f64,
    face_size: f64,
    _marker_size: f64,
    ampm: bool,
) -> u32 {
    if ampm {
        let (value, _) = get_angle_value(30.0, offset_x, offset_y, face_size);
        if value == 0 {
            12
        } else {
            value
        }
    } else {
        let (value, _) = get_angle_value(15.0, offset_x, offset_y, face_size);
        value % 24
    }
}

/// Resolve minute from pointer coordinates, snapped to `minute_step`.
pub fn get_minutes_from_point(
    offset_x: f64,
    offset_y: f64,
    face_size: f64,
    minute_step: u32,
) -> u32 {
    let step = minute_step.max(1) as f64;
    let angle_step = step * 6.0;
    let (value, _) = get_angle_value(angle_step, offset_x, offset_y, face_size);
    (value * minute_step.max(1)) % 60
}

/// Always-visible minute dial labels (00, 05, …, 55).
pub fn minute_display_markers() -> [u32; 12] {
    [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
}

/// Hand rotation angle in degrees for a minute value (0–59).
pub fn minute_hand_degrees(minute: u32) -> f64 {
    (360.0 / 60.0) * minute as f64
}

/// Hand rotation angle in degrees for the current hour selection.
pub fn hour_hand_degrees(hour_24: u32, ampm: bool) -> f64 {
    if ampm {
        let display = super::to_twelve_hour(hour_24).0;
        let value = if display == 12 { 0 } else { display };
        (360.0 / 12.0) * value as f64
    } else {
        (360.0 / 24.0) * hour_24 as f64
    }
}

/// Whether a minute value is disabled for the given step.
pub fn is_minute_step_disabled(minute: u32, minute_step: u32) -> bool {
    let step = minute_step.max(1);
    step > 1 && !minute.is_multiple_of(step)
}

/// Index of a minute label in the fixed 5-minute display ring.
pub fn minute_display_index(minute: u32) -> usize {
    let snapped = (minute / 5) * 5;
    minute_display_markers()
        .iter()
        .position(|&m| m == snapped)
        .unwrap_or(0)
}

/// SVG tick endpoints for the 12 hour marks on the dial border.
pub fn tick_endpoints(index: usize, face_size: f64) -> (f64, f64, f64, f64) {
    let center = face_size / 2.0;
    let scale = face_size / DEFAULT_FACE_SIZE;
    let angle = marker_angle(index, 12);
    // Radii in viewBox coordinates (dial border is r=125 on the 260 viewBox).
    let tick_inner = 104.0 * scale;
    let tick_outer = 125.0 * scale;
    let x1 = center + tick_inner * angle.cos();
    let y1 = center + tick_inner * angle.sin();
    let x2 = center + tick_outer * angle.cos();
    let y2 = center + tick_outer * angle.sin();
    (x1, y1, x2, y2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn center(face: f64) -> f64 {
        face / 2.0
    }

    #[test]
    fn top_of_clock_is_twelve_ampm() {
        let c = center(DEFAULT_FACE_SIZE);
        assert_eq!(
            get_hours_from_point(c, 0.0, DEFAULT_FACE_SIZE, DEFAULT_MARKER_SIZE, true),
            12
        );
    }

    #[test]
    fn three_oclock_is_three_ampm() {
        let c = center(DEFAULT_FACE_SIZE);
        assert_eq!(
            get_hours_from_point(
                DEFAULT_FACE_SIZE,
                c,
                DEFAULT_FACE_SIZE,
                DEFAULT_MARKER_SIZE,
                true
            ),
            3
        );
    }

    #[test]
    fn top_of_clock_is_zero_24h_outer() {
        let c = center(DEFAULT_FACE_SIZE);
        assert_eq!(
            get_hours_from_point(c, 0.0, DEFAULT_FACE_SIZE, DEFAULT_MARKER_SIZE, false),
            0
        );
    }

    #[test]
    fn three_oclock_is_fifteen_minutes() {
        let c = center(DEFAULT_FACE_SIZE);
        assert_eq!(
            get_minutes_from_point(DEFAULT_FACE_SIZE, c, DEFAULT_FACE_SIZE, 1),
            15
        );
    }

    #[test]
    fn minute_step_snaps_on_drag() {
        let c = center(DEFAULT_FACE_SIZE);
        let minute = get_minutes_from_point(DEFAULT_FACE_SIZE, c, DEFAULT_FACE_SIZE, 5);
        assert_eq!(minute % 5, 0);
    }

    #[test]
    fn minute_display_markers_are_five_min_apart() {
        let markers = minute_display_markers();
        assert_eq!(markers.len(), 12);
        assert_eq!(markers[0], 0);
        assert_eq!(markers[11], 55);
    }

    #[test]
    fn tick_marks_span_to_dial_border() {
        let (_, _, x2, y2) = tick_endpoints(0, DEFAULT_FACE_SIZE);
        let center = DEFAULT_FACE_SIZE / 2.0;
        // Top tick outer point should reach the dial border (r=125).
        let outer_radius = ((x2 - center).powi(2) + (y2 - center).powi(2)).sqrt();
        assert!((outer_radius - 125.0).abs() < 0.5);
    }
}
