use turf::inline_style_sheet_values;

/// Compiled material surface stylesheet and stable BEM class names.
pub fn material_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-material {
            position: relative;
            box-sizing: border-box;
            overflow: hidden;
            width: var(--orbital-material-width, 100%);
            max-width: var(--orbital-material-max-width, 100%);
            margin: var(--orbital-material-margin, 0);
            border-radius: var(--orb-radius-md);
            color: var(--orb-color-text-primary);
        }

        .orbital-material--solid {
            background-color: var(--orb-color-surface-canvas);
        }

        .orbital-material--frost {
            background: color-mix(in oklab, var(--orb-color-surface-canvas) 78%, transparent);
            backdrop-filter: saturate(108%) blur(12px);
        }

        .orbital-material--shell {
            background-color: var(--orb-color-surface-shell);
            backdrop-filter: saturate(128%) blur(16px);
        }

        .orbital-material--scrim {
            background: color-mix(in oklab, black 38%, transparent);
        }

        .orbital-material--outlined {
            background-color: var(--orb-color-surface-canvas);
            border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
        }

        .orbital-material--square {
            border-radius: 0;
        }

        .orbital-material--elev-flat {
            box-shadow: none;
        }

        // Flat elevation with outlined stroke separators for co-planar shell chrome.
        // Shell chrome composes edge-specific outline modifiers at Flat elevation.
        .orbital-material--elev-flat.orbital-material--outline-bottom {
            border-bottom: var(--orb-stroke-thin) solid var(--orb-color-border-default);
        }

        .orbital-material--elev-flat.orbital-material--outline-end {
            border-inline-end: var(--orb-stroke-thin) solid var(--orb-color-border-default);
        }

        .orbital-material--elev-resting {
            box-shadow: var(--orb-elev-raised-sm);
        }

        .orbital-material--elev-raised {
            box-shadow: var(--orb-elev-raised-md);
        }

        .orbital-material--elev-floating {
            box-shadow: var(--orb-elev-floating);
        }

        .orbital-material--elev-modal {
            box-shadow: var(--orb-elev-modal);
        }
    };

    style_sheet
}

pub fn material_modifier_classes(
    variant: orbital_base_components::MaterialVariant,
    elevation: orbital_base_components::MaterialElevation,
    corners: orbital_base_components::MaterialCorners,
) -> String {
    format!(
        "{} {} {}",
        variant.modifier_class(),
        elevation.modifier_class(),
        corners.modifier_class()
    )
}

/// Shell edge outline applied at [`MaterialElevation::Flat`] for co-planar regions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialOutlineEdge {
    Bottom,
    End,
}

impl MaterialOutlineEdge {
    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Bottom => "orbital-material--outline-bottom",
            Self::End => "orbital-material--outline-end",
        }
    }
}

/// Outline edge modifier for Flat shell chrome, or empty when elevated / already outlined.
pub fn material_flat_outline_modifier(
    variant: orbital_base_components::MaterialVariant,
    elevation: orbital_base_components::MaterialElevation,
    edge: MaterialOutlineEdge,
) -> Option<&'static str> {
    if elevation == orbital_base_components::MaterialElevation::Flat
        && variant != orbital_base_components::MaterialVariant::Outlined
    {
        Some(edge.modifier_class())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{material_flat_outline_modifier, material_styles, MaterialOutlineEdge};
    use orbital_base_components::{MaterialElevation, MaterialVariant};

    #[test]
    fn flat_outline_modifiers_use_outlined_stroke() {
        let css = material_styles();
        assert!(css.contains(".orbital-material--elev-flat.orbital-material--outline-bottom"));
        assert!(css.contains(".orbital-material--elev-flat.orbital-material--outline-end"));
        assert!(css.contains("var(--orb-color-border-default)"));
    }

    #[test]
    fn flat_outline_modifier_only_for_flat_non_outlined() {
        assert_eq!(
            material_flat_outline_modifier(
                MaterialVariant::Solid,
                MaterialElevation::Flat,
                MaterialOutlineEdge::Bottom
            ),
            Some("orbital-material--outline-bottom")
        );
        assert_eq!(
            material_flat_outline_modifier(
                MaterialVariant::Outlined,
                MaterialElevation::Flat,
                MaterialOutlineEdge::Bottom
            ),
            None
        );
        assert_eq!(
            material_flat_outline_modifier(
                MaterialVariant::Solid,
                MaterialElevation::Resting,
                MaterialOutlineEdge::End
            ),
            None
        );
    }

    #[test]
    fn shell_variant_uses_neutral_background2() {
        let css = material_styles();
        assert!(css.contains(".orbital-material--shell"));
        assert!(css.contains("var(--orb-color-surface-shell)"));
    }
}
