/// Compiled radio/radio-group stylesheet and stable `orbital-radio*` class names.
///
pub fn radio_group_styles() -> &'static str {
    r#"
.orbital-radio-group {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}

.orbital-radio-group--horizontal {
    flex-direction: row;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-md);
}

.orbital-radio {
    display: inline-flex;
    position: relative;
}

.orbital-radio__input {
    position: absolute;
    left: 0px;
    top: 0px;
    width: calc(16px + 2 * var(--orb-space-inline-sm));
    height: 100%;
    box-sizing: border-box;
    margin: 0px;
    opacity: 0;
}

.orbital-radio__input:enabled {
    cursor: pointer;
}

.orbital-radio__indicator {
    position: relative;
    width: 16px;
    height: 16px;
    font-size: 12px;
    box-sizing: border-box;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    border: var(--orb-stroke-thin) solid;
    border-radius: var(--orb-radius-circular);
    margin: var(--orb-space-block-sm) var(--orb-space-inline-sm);
    fill: currentcolor;
    pointer-events: none;
}

.orbital-radio__input:enabled:not(:checked) ~ .orbital-radio__indicator {
    border-color: var(--orb-color-border-accessible);
}

.orbital-radio__input:checked ~ .orbital-radio__indicator {
    border-color: var(--orb-color-brand-compound-stroke);
    color: var(--orb-color-brand-compound-fg);
}

.orbital-radio__input:hover:checked ~ .orbital-radio__indicator {
    color: var(--orb-color-brand-compound-fg-hover);
}

.orbital-radio__input:hover:active ~ .orbital-radio__indicator {
    color: var(--orb-color-brand-compound-fg-pressed);
}

.orbital-radio__indicator::after {
    position: absolute;
    width: 16px;
    height: 16px;
    border-radius: var(--orb-radius-circular);
    transform: scale(0.625);
    background-color: currentcolor;
}

.orbital-radio__input:checked ~ .orbital-radio__indicator::after {
    content: "";
}

.orbital-radio__label {
    margin-bottom: calc((16px - var(--orb-type-line-md)) / 2);
    margin-top: calc((16px - var(--orb-type-line-md)) / 2);
    align-self: center;
    padding-bottom: var(--orb-space-block-sm);
    padding-top: var(--orb-space-block-sm);
    padding-left: var(--orb-space-inline-xs);
    padding-right: var(--orb-space-inline-sm);
    line-height: var(--orb-type-line-md);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: var(--orb-color-text-tertiary);
}

.orbital-radio__input:enabled:hover ~ .orbital-radio__label {
    color: var(--orb-color-text-secondary);
}

.orbital-radio__input:enabled:active ~ .orbital-radio__label {
    color: var(--orb-color-text-primary);
}

.orbital-radio__input:enabled:checked ~ .orbital-radio__label {
    color: var(--orb-color-text-primary);
}

.orbital-radio__input:enabled ~ .orbital-radio__label {
    cursor: pointer;
}
"#
}
