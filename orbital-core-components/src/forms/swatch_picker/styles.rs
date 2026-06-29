pub fn swatch_picker_styles() -> &'static str {
    r#"
.orbital-swatch-picker {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-xs);
}

.orbital-swatch-picker--grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(32px, 1fr));
    align-items: stretch;
}

.orbital-swatch-picker--row {
    flex-direction: row;
}

.orbital-swatch-picker__item {
    box-sizing: border-box;
    padding: 0;
    margin: 0;
    border: var(--orb-stroke-thick) solid transparent;
    background: var(--orbital-swatch-picker__color, var(--orb-color-surface-raised));
    cursor: pointer;
    flex-shrink: 0;
}

.orbital-swatch-picker__item:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: 2px;
}

.orbital-swatch-picker__item--rounded {
    border-radius: var(--orb-radius-circular);
}

.orbital-swatch-picker__item--square {
    border-radius: var(--orb-radius-sm);
}

.orbital-swatch-picker__item--small {
    width: 24px;
    height: 24px;
}

.orbital-swatch-picker__item--medium {
    width: 32px;
    height: 32px;
}

.orbital-swatch-picker__item--selected {
    border-color: var(--orb-color-brand-compound-stroke);
    box-shadow: 0 0 0 1px var(--orb-color-brand-compound-stroke);
}

.orbital-swatch-picker__item--disabled,
.orbital-swatch-picker__item:disabled {
    opacity: 0.4;
    cursor: not-allowed;
}
"#
}
