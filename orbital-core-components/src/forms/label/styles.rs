use turf::inline_style_sheet_values;

/// Compiled label stylesheet and stable `orbital-label*` class names (excluded from turf hashing).
pub fn label_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-label {
            line-height: var(--orb-type-line-md);
            font-size: var(--orb-type-size-sm);
            font-family: var(--orb-type-family-sans);
            color: var(--orb-color-text-primary);
        }

        .orbital-label__required {
            padding-left: var(--orb-space-inline-xs);
            color: var(--orb-color-palette-red-fg-strong);
        }

        .orbital-label--disabled {
            color: var(--orb-color-text-disabled);
        }

        .orbital-label--disabled .orbital-label__required {
            color: var(--orb-color-text-disabled);
        }

        .orbital-label--small {
            font-size: var(--orb-type-size-xs);
            line-height: var(--orb-type-line-sm);
        }

        .orbital-label--large {
            font-weight: var(--orb-type-weight-semibold);
            font-size: var(--orb-type-size-md);
            line-height: var(--orb-type-line-lg);
        }

        .orbital-label--semibold {
            font-weight: var(--orb-type-weight-semibold);
        }
    };
    style_sheet
}
