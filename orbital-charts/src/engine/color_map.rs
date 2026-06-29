//! Color scale resolution for heatmap and axis color maps.

use crate::{ColorScale, ColorScaleKind};

/// Default continuous heatmap color stops (Orbital sequential).
pub fn default_heatmap_colors() -> Vec<String> {
    vec!["#eef2ff".into(), "#2563eb".into()]
}

/// Resolve a numeric value to a CSS color via a color scale.
pub fn resolve_color(value: f64, scale: &ColorScale, domain: (f64, f64)) -> String {
    match scale.kind {
        ColorScaleKind::Continuous => continuous_color(value, &scale.colors, domain),
        ColorScaleKind::Piecewise => piecewise_color(value, scale),
        ColorScaleKind::Ordinal => ordinal_color(value, &scale.colors),
    }
}

/// Default continuous color scale for heatmaps.
pub fn default_continuous_scale() -> ColorScale {
    ColorScale {
        kind: ColorScaleKind::Continuous,
        colors: default_heatmap_colors(),
        thresholds: None,
    }
}

fn continuous_color(value: f64, colors: &[String], domain: (f64, f64)) -> String {
    let colors = if colors.len() >= 2 {
        colors
    } else {
        &default_heatmap_colors()
    };
    let (min, max) = domain;
    if (max - min).abs() < f64::EPSILON {
        return colors[0].clone();
    }
    let t = ((value - min) / (max - min)).clamp(0.0, 1.0);
    if colors.len() == 2 {
        return lerp_css_color(&colors[0], &colors[1], t);
    }
    let segments = colors.len() - 1;
    let scaled = t * segments as f64;
    let idx = scaled.floor() as usize;
    let frac = scaled - idx as f64;
    let i = idx.min(segments - 1);
    lerp_css_color(&colors[i], &colors[i + 1], frac)
}

fn piecewise_color(value: f64, scale: &ColorScale) -> String {
    let thresholds = scale.thresholds.as_deref().unwrap_or(&[]);
    let colors = if scale.colors.is_empty() {
        default_heatmap_colors()
    } else {
        scale.colors.clone()
    };
    if colors.is_empty() {
        return "currentColor".into();
    }
    if thresholds.is_empty() {
        return colors[0].clone();
    }
    let mut bucket = 0usize;
    for (i, t) in thresholds.iter().enumerate() {
        if value >= *t {
            bucket = i + 1;
        }
    }
    colors[bucket.min(colors.len() - 1)].clone()
}

fn ordinal_color(value: f64, colors: &[String]) -> String {
    if colors.is_empty() {
        return "currentColor".into();
    }
    let idx = (value.round() as usize) % colors.len();
    colors[idx].clone()
}

/// Simple hex/CSS lerp; falls back to first color for non-hex tokens.
fn lerp_css_color(a: &str, b: &str, t: f64) -> String {
    let (ra, ga, ba) = parse_hex_or_default(a);
    let (rb, gb, bb) = parse_hex_or_default(b);
    let r = (ra as f64 + (rb as f64 - ra as f64) * t).round() as u8;
    let g = (ga as f64 + (gb as f64 - ga as f64) * t).round() as u8;
    let b = (ba as f64 + (bb as f64 - ba as f64) * t).round() as u8;
    format!("#{r:02x}{g:02x}{b:02x}")
}

fn parse_hex_or_default(s: &str) -> (u8, u8, u8) {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#') {
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return (r, g, b);
            }
        }
    }
    (238, 242, 255)
}

/// Compute z-value domain from cells and optional overrides.
pub fn heatmap_value_domain(
    cells: &[crate::HeatmapCell],
    min_override: Option<f64>,
    max_override: Option<f64>,
) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for cell in cells {
        if cell.value.is_finite() {
            min = min.min(cell.value);
            max = max.max(cell.value);
        }
    }
    if !min.is_finite() {
        min = 0.0;
        max = 100.0;
    }
    (min_override.unwrap_or(min), max_override.unwrap_or(max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuous_color_endpoints() {
        let scale = default_continuous_scale();
        let c0 = resolve_color(0.0, &scale, (0.0, 100.0));
        let c100 = resolve_color(100.0, &scale, (0.0, 100.0));
        assert!(c0.starts_with('#'));
        assert!(c100.starts_with('#'));
        assert_ne!(c0, c100);
    }

    #[test]
    fn piecewise_color_buckets() {
        let scale = ColorScale {
            kind: ColorScaleKind::Piecewise,
            colors: vec!["#000".into(), "#888".into(), "#fff".into()],
            thresholds: Some(vec![10.0, 20.0]),
        };
        assert_eq!(resolve_color(5.0, &scale, (0.0, 100.0)), "#000");
        assert_eq!(resolve_color(15.0, &scale, (0.0, 100.0)), "#888");
        assert_eq!(resolve_color(25.0, &scale, (0.0, 100.0)), "#fff");
    }
}
