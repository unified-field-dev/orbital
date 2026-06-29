pub fn compound_button_styles() -> &'static str {
    r#"
.orbital-compound-button__content {
    display: inline-flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    text-align: left;
}

.orbital-compound-button__text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    line-height: 1.2;
}

.orbital-compound-button__primary {
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-compound-button__secondary {
    font-size: var(--orb-type-size-xs);
    color: var(--orb-color-text-secondary);
    font-weight: var(--orb-type-weight-regular);
}

.orbital-button .orbital-compound-button__icon {
    display: inline-flex;
    align-items: center;
}
"#
}
