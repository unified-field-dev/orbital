use orbital_base_components::OverlayAppearance;

/// Build BEM modifier classes for a floating overlay Material root.
pub fn overlay_surface_class(
    prefix: &str,
    appearance: OverlayAppearance,
    size: Option<&str>,
) -> String {
    let mut parts = vec![prefix.to_string()];
    if let Some(modifier) = appearance.modifier_class() {
        parts.push(format!("{prefix}--{modifier}"));
    } else if prefix.contains("tooltip") {
        parts.push(format!("{prefix}--normal"));
    }
    if let Some(size) = size {
        parts.push(format!("{prefix}--{size}"));
    }
    parts.join(" ")
}
