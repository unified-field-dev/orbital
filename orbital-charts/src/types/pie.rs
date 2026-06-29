//! Pie chart types.

/// One slice in a pie series.
#[derive(Clone, Debug, PartialEq)]
pub struct PieSliceData {
    /// Stable slice identifier.
    pub id: String,
    /// Display label for legend and arc labels.
    pub label: String,
    /// Slice magnitude (must be non-negative).
    pub value: f64,
    /// Optional per-slice color override.
    pub color: Option<String>,
}

/// Pie geometry configuration (radii and center may be px or `%` strings).
#[derive(Clone, Debug, PartialEq)]
pub struct PieGeometry {
    /// Inner radius for donut charts (0 = full pie).
    pub inner_radius: PieRadius,
    /// Outer radius.
    pub outer_radius: PieRadius,
    /// Gap between slices in degrees.
    pub padding_angle: f64,
    /// Start angle in degrees.
    pub start_angle: f64,
    /// End angle in degrees.
    pub end_angle: f64,
    /// Center X (px number or `%` of plot width).
    pub cx: PieRadius,
    /// Center Y (px number or `%` of plot height).
    pub cy: PieRadius,
    /// Arc corner radius in pixels.
    pub corner_radius: f64,
}

impl Default for PieGeometry {
    fn default() -> Self {
        Self {
            inner_radius: PieRadius::Px(0.0),
            outer_radius: PieRadius::Percent(90.0),
            padding_angle: 0.0,
            start_angle: 0.0,
            end_angle: 360.0,
            cx: PieRadius::Percent(50.0),
            cy: PieRadius::Percent(50.0),
            corner_radius: 0.0,
        }
    }
}

/// Radius or center offset expressed as pixels or percentage of plot dimension.
#[derive(Clone, Debug, PartialEq)]
pub enum PieRadius {
    /// Absolute pixels.
    Px(f64),
    /// Percentage of plot width (cx/outer) or height (cy) or min dimension (radii).
    Percent(f64),
}

impl PieRadius {
    /// Parse a numeric or percentage string (e.g. `"50%"`, `"40"`).
    pub fn parse(s: &str) -> Self {
        let trimmed = s.trim();
        if let Some(pct) = trimmed.strip_suffix('%') {
            if let Ok(v) = pct.trim().parse::<f64>() {
                return Self::Percent(v);
            }
        }
        if let Ok(v) = trimmed.parse::<f64>() {
            return Self::Px(v);
        }
        Self::Percent(50.0)
    }

    /// Resolve to pixels given plot width, height, and whether this is a radius.
    pub fn resolve(&self, plot_width: f64, plot_height: f64, is_radius: bool) -> f64 {
        match self {
            Self::Px(v) => *v,
            Self::Percent(pct) => {
                let factor = pct / 100.0;
                if is_radius {
                    plot_width.min(plot_height) / 2.0 * factor
                } else {
                    // cx uses width, cy uses height — caller passes appropriate dim via is_radius=false + custom resolve
                    plot_width.min(plot_height) * factor
                }
            }
        }
    }

    /// Resolve center coordinate: `is_x` true uses width, false uses height.
    pub fn resolve_center(&self, plot_width: f64, plot_height: f64, is_x: bool) -> f64 {
        match self {
            Self::Px(v) => *v,
            Self::Percent(pct) => {
                let dim = if is_x { plot_width } else { plot_height };
                dim * pct / 100.0
            }
        }
    }
}

/// Arc label content mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArcLabelMode {
    /// Raw numeric value.
    Value,
    /// Value formatted via series formatter.
    #[default]
    FormattedValue,
    /// Slice label text.
    Label,
}

/// Configuration for pie arc labels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PieArcLabelConfig {
    /// Label content mode.
    pub mode: Option<ArcLabelMode>,
    /// Minimum slice angle (degrees) to show a label.
    pub min_angle: Option<f64>,
    /// Label placement radius (px or `%`).
    pub radius: Option<PieRadius>,
}

/// Projected pie data ready for layout.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedPieData {
    /// Series identifier.
    pub series_id: String,
    /// Slice data.
    pub slices: Vec<PieSliceData>,
}
