/// Clamps a column width between optional min/max bounds.
pub fn clamp_column_width(width: f64, min_width: Option<f64>, max_width: Option<f64>) -> f64 {
    let mut new_width = width;
    if let Some(max_width) = max_width {
        if new_width > max_width {
            new_width = max_width;
        }
    }
    if let Some(min_width) = min_width {
        if new_width < min_width {
            new_width = min_width;
        }
    }
    if new_width < 0.0 {
        new_width = 0.0;
    }
    new_width
}

/// Inline style for a resizable header cell width.
pub fn column_width_style(width: f64, min_width: Option<f64>, max_width: Option<f64>) -> String {
    let mut style = format!("width: {width:.2}px");
    if let Some(max_width) = max_width {
        style.push_str(&format!(";max-width: {max_width:.2}px"));
    }
    if let Some(min_width) = min_width {
        style.push_str(&format!(";min-width: {min_width:.2}px"));
    }
    style
}

#[cfg(test)]
mod tests {
    use super::{clamp_column_width, column_width_style};

    #[test]
    fn clamp_respects_min_and_max() {
        assert_eq!(clamp_column_width(50.0, Some(80.0), Some(240.0)), 80.0);
        assert_eq!(clamp_column_width(300.0, Some(80.0), Some(240.0)), 240.0);
    }

    #[test]
    fn width_style_includes_bounds() {
        let style = column_width_style(120.0, Some(80.0), Some(240.0));
        assert!(style.contains("width: 120.00px"));
        assert!(style.contains("min-width: 80.00px"));
        assert!(style.contains("max-width: 240.00px"));
    }
}
