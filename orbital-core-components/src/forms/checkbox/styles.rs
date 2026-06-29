/// Compiled checkbox stylesheet and stable `orbital-checkbox*` class names (excluded from turf hashing).
///
pub fn checkbox_styles() -> &'static str {
    r#"
.orbital-checkbox {
    position: relative;
    display: inline-flex;
    vertical-align: middle;
    color: var(--orb-color-text-tertiary);
    cursor: pointer;
}

.orbital-checkbox:hover {
    color: var(--orb-color-text-secondary);
    --orbital-checkbox__indicator--border-color: var(--orb-color-border-accessible-hover);
}

.orbital-checkbox:active {
    color: var(--orb-color-text-primary);
    --orbital-checkbox__indicator--border-color: var(--orb-color-border-accessible-pressed);
}

.orbital-checkbox--checked {
    color: var(--orb-color-text-primary);
    --orbital-checkbox__indicator--background-color: var(--orb-color-brand-compound-bg);
    --orbital-checkbox__indicator--color: var(--orb-color-text-on-brand);
    --orbital-checkbox__indicator--border-color: var(--orb-color-brand-compound-bg);
}

.orbital-checkbox--checked:hover {
    --orbital-checkbox__indicator--border-color: var(--orb-color-brand-compound-bg-hover);
    --orbital-checkbox__indicator--background-color: var(--orb-color-brand-compound-bg-hover);
}

.orbital-checkbox--checked:active {
    --orbital-checkbox__indicator--border-color: var(--orb-color-brand-compound-bg-pressed);
    --orbital-checkbox__indicator--background-color: var(--orb-color-brand-compound-bg-pressed);
}

.orbital-checkbox:focus,
.orbital-checkbox:focus-visible {
    outline-style: none;
}

.orbital-checkbox__input {
    position: absolute;
    top: 0px;
    left: 0px;
    width: calc(16px + 2 * var(--orb-space-inline-sm));
    height: 100%;
    margin: 0px;
    opacity: 0;
    box-sizing: border-box;
    cursor: inherit;
}

.orbital-checkbox--large > .orbital-checkbox__input {
    width: calc(20px + 2 * var(--orb-space-inline-sm));
}

.orbital-checkbox__input:disabled {
    cursor: not-allowed;
}

.orbital-checkbox:has(.orbital-checkbox__input:disabled) {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}

.orbital-checkbox__input:disabled ~ .orbital-checkbox__indicator {
    background-color: var(--orb-color-surface-disabled);
    border-color: var(--orb-color-border-disabled);
    color: var(--orb-color-text-disabled);
}

.orbital-checkbox__indicator {
    align-self: flex-start;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: var(--orb-space-block-sm) var(--orb-space-inline-sm);
    height: 16px;
    width: 16px;
    background-color: var(--orbital-checkbox__indicator--background-color);
    font-size: 12px;
    color: var(--orbital-checkbox__indicator--color);
    border-color: var(
        --orbital-checkbox__indicator--border-color,
        var(--orb-color-border-accessible)
    );
    border-style: solid;
    border-width: var(--orb-stroke-thin);
    border-radius: var(--orb-radius-sm);
    fill: currentcolor;
    box-sizing: border-box;
    pointer-events: none;
    overflow: hidden;
}

.orbital-checkbox--large > .orbital-checkbox__indicator {
    font-size: 16px;
    height: 20px;
    width: 20px;
}

.orbital-checkbox__check-icon {
    display: block;
    line-height: 0;
}

.orbital-checkbox__label {
    align-self: center;
    margin-bottom: calc((16px - var(--orb-type-line-md)) / 2);
    margin-top: calc((16px - var(--orb-type-line-md)) / 2);
    padding-bottom: var(--orb-space-block-sm);
    padding-top: var(--orb-space-block-sm);
    padding-left: var(--orb-space-inline-xs);
    padding-right: var(--orb-space-inline-sm);
    line-height: var(--orb-type-line-md);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    color: inherit;
    cursor: inherit;
}

.orbital-checkbox--large > .orbital-checkbox__label {
    margin-top: calc((20px - var(--orb-type-line-md)) / 2);
    margin-bottom: calc((20px - var(--orb-type-line-md)) / 2);
}

.orbital-checkbox__input:disabled ~ .orbital-checkbox__label {
    cursor: not-allowed;
}
"#
}
