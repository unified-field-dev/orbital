//! Heatmap cell layout from band scales.

use crate::engine::color_map::{heatmap_value_domain, resolve_color};
use crate::engine::scales::BandScale;
use crate::{ColorScale, HeatmapCell, HeatmapCellLayout};

/// Default gap ratio between heatmap cells (visual 2px gap via inset).
pub const HEATMAP_CELL_GAP_RATIO: f64 = 0.08;

/// Threshold above which canvas rendering is preferred (internal optimization).
pub const HEATMAP_CANVAS_THRESHOLD: usize = 2500;

/// Compute cell layouts from band scales and color scale.
pub fn compute_heatmap_cell_layouts(
    cells: &[HeatmapCell],
    x_scale: &BandScale,
    y_scale: &BandScale,
    color_scale: &ColorScale,
    value_min: Option<f64>,
    value_max: Option<f64>,
    gap_ratio: f64,
) -> Vec<HeatmapCellLayout> {
    let domain = heatmap_value_domain(cells, value_min, value_max);
    let x_bw = x_scale.bandwidth() * (1.0 - gap_ratio);
    let y_bw = y_scale.bandwidth() * (1.0 - gap_ratio);

    cells
        .iter()
        .enumerate()
        .filter_map(|(index, cell)| {
            let x_center = x_scale.scale_by_index(cell.x)?;
            let y_center = y_scale.scale_by_index(cell.y)?;
            Some(HeatmapCellLayout {
                index,
                x_index: cell.x,
                y_index: cell.y,
                x: x_center - x_bw / 2.0,
                y: y_center - y_bw / 2.0,
                width: x_bw,
                height: y_bw,
                fill: resolve_color(cell.value, color_scale, domain),
                value: cell.value,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ColorScaleKind;

    #[test]
    fn compute_cell_layouts_positions() {
        let x = BandScale::new(vec!["A".into(), "B".into()], (0.0, 200.0), 0.1);
        let y = BandScale::new(vec!["1".into(), "2".into()], (0.0, 100.0), 0.1);
        let scale = ColorScale {
            kind: ColorScaleKind::Continuous,
            colors: vec!["#000".into(), "#fff".into()],
            thresholds: None,
        };
        let cells = vec![HeatmapCell {
            x: 0,
            y: 0,
            value: 50.0,
        }];
        let layouts = compute_heatmap_cell_layouts(
            &cells,
            &x,
            &y,
            &scale,
            None,
            None,
            HEATMAP_CELL_GAP_RATIO,
        );
        assert_eq!(layouts.len(), 1);
        assert!(layouts[0].width > 0.0);
        assert!(layouts[0].height > 0.0);
    }
}
