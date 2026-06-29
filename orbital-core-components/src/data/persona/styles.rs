pub fn persona_styles() -> &'static str {
    r#".orbital-persona {
    display: inline;
}

.orbital-persona__text {
    min-width: 0;
}

.orbital-persona__primary-text {
    display: block;
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-md);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-lg);
}

.orbital-persona__secondary-text {
    margin-top: -2px;
}

.orbital-persona__secondary-text,
.orbital-persona__tertiary-text,
.orbital-persona__quaternary-text {
    display: block;
    color: var(--orb-color-text-secondary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-regular);
    line-height: var(--orb-type-line-sm);
}
"#
}
