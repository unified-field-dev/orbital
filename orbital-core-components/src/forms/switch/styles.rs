/// Compiled switch stylesheet and stable `orbital-switch*` class names (excluded from turf hashing).
///
pub fn switch_styles() -> &'static str {
    r#"
.orbital-switch {
    align-items: flex-start;
    box-sizing: border-box;
    display: inline-flex;
    position: relative;
}

.orbital-switch__input {
    position: absolute;
    top: 0px;
    left: 0px;
    width: calc(36px + 2 * var(--orb-space-inline-sm));
    height: 100%;
    margin: 0px;
    opacity: 0;
    box-sizing: border-box;
    cursor: pointer;
}

.orbital-switch__input:disabled {
    cursor: not-allowed;
}

.orbital-switch__indicator {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    width: 36px;
    height: 18px;
    padding: 2px;
    margin: var(--orb-space-block-sm) var(--orb-space-inline-sm);
    border-radius: var(--orb-radius-circular);
    border: var(--orb-stroke-thin) solid;
    box-sizing: border-box;
    pointer-events: none;
    transition-duration: var(--orb-motion-duration-md);
    transition-timing-function: var(--orb-motion-ease-standard);
    transition-property: background, border, color;
}

.orbital-switch__thumb {
    width: 14px;
    height: 14px;
    border-radius: var(--orb-radius-circular);
    background-color: currentcolor;
    box-shadow: var(--orb-elev-raised-xs);
    transition-duration: var(--orb-motion-duration-md);
    transition-timing-function: var(--orb-motion-ease-standard);
    transition-property: transform;
}

.orbital-switch__input:enabled:not(:checked) ~ .orbital-switch__indicator {
    background-color: var(--orb-color-surface-subtle);
    border-color: var(--orb-color-border-default);
    color: var(--orb-color-surface-canvas);
}

.orbital-switch__input:enabled:checked ~ .orbital-switch__indicator {
    background-color: var(--orb-color-brand-compound-bg);
    border-color: var(--orb-color-border-transparent);
    color: var(--orb-color-text-on-brand);
}

.orbital-switch__input:enabled:checked:hover ~ .orbital-switch__indicator {
    background-color: var(--orb-color-brand-compound-bg-hover);
}

.orbital-switch__input:enabled:checked:active ~ .orbital-switch__indicator {
    background-color: var(--orb-color-brand-compound-bg-pressed);
}

.orbital-switch__input:enabled:checked ~ .orbital-switch__indicator .orbital-switch__thumb {
    transform: translateX(18px);
}

.orbital-switch__input:disabled:not(:checked) ~ .orbital-switch__indicator {
    background-color: var(--orb-color-surface-disabled);
    border-color: var(--orb-color-border-disabled);
    color: var(--orb-color-text-disabled);
}

.orbital-switch__input:disabled:checked ~ .orbital-switch__indicator {
    background-color: var(--orb-color-surface-disabled);
    border-color: var(--orb-color-border-disabled);
    color: var(--orb-color-text-disabled);
}

.orbital-switch__label {
    margin-top: calc((18px - var(--orb-type-line-md)) / 2);
    margin-bottom: calc((18px - var(--orb-type-line-md)) / 2);
    padding: var(--orb-space-block-sm) var(--orb-space-inline-sm);
    padding-left: var(--orb-space-inline-xs);
    line-height: var(--orb-type-line-md);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: var(--orb-color-text-primary);
    cursor: pointer;
}

.orbital-switch__input:enabled:not(:checked) ~ .orbital-switch__label {
    color: var(--orb-color-text-primary);
}

.orbital-switch__input:disabled ~ .orbital-switch__label {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}
"#
}
