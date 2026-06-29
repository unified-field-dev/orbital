//! Reference line styling types.

/// Label placement along a reference line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReferenceLineLabelAlign {
    Start,
    #[default]
    Middle,
    End,
}

/// SVG stroke styling for a reference line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReferenceLineStyle {
    /// Stroke color (CSS color string).
    pub color: Option<String>,
    /// Stroke width in pixels.
    pub stroke_width: Option<f64>,
    /// SVG dash array (e.g. `"6 4"`).
    pub dash_array: Option<String>,
    /// Line opacity (0–1).
    pub opacity: Option<f64>,
}
