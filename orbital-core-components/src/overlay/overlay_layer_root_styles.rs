/// Stacking-context shell for scoped overlay portal mounts (chart embeds, scroll hosts).
pub fn overlay_layer_root_styles() -> &'static str {
    r#".orbital-overlay-layer-root {
    position: relative;
    isolation: isolate;
    z-index: 1;
    pointer-events: none;
}
"#
}

#[cfg(test)]
mod tests {
    use super::overlay_layer_root_styles;

    #[test]
    fn overlay_layer_root_establishes_stacking_context() {
        let css = overlay_layer_root_styles();
        assert!(css.contains("position: relative"));
        assert!(css.contains("isolation: isolate"));
        assert!(css.contains("z-index: 1"));
    }
}
