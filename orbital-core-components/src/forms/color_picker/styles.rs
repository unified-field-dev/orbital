pub fn color_picker_styles() -> &'static str {
    r#"
.orbital-color-picker {
    display: inline-flex;
}

.orbital-color-picker__trigger {
    display: inline-flex;
    align-items: center;
    gap: var(--orb-space-inline-snudge);
    min-height: 32px;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    padding: 0 var(--orb-space-inline-mnudge);
    cursor: pointer;
}

.orbital-color-picker__trigger:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: 1px;
}

.orbital-color-picker--disabled .orbital-color-picker__trigger {
    color: var(--orb-color-text-disabled);
    border-color: var(--orb-color-border-disabled);
    cursor: not-allowed;
}

.orbital-color-picker__swatch {
    width: 14px;
    height: 14px;
    border-radius: var(--orb-radius-sm);
    border: 1px solid var(--orb-color-border-subtle);
}

.orbital-color-picker__label {
    font-family: var(--orb-type-family-mono);
    font-size: var(--orb-type-size-xs);
}

.orbital-color-picker-panel {
    min-width: 248px;
    padding: var(--orb-space-block-snudge);
    border-radius: var(--orb-radius-md);
    border: 1px solid var(--orb-color-border-default);
    background: var(--orb-color-surface-canvas);
    box-shadow: var(--orb-elev-floating);
}

.orbital-color-picker-panel__sv {
    position: relative;
    width: 220px;
    height: 140px;
    border-radius: var(--orb-radius-sm);
    overflow: hidden;
    cursor: crosshair;
}

.orbital-color-picker-panel__sv-white,
.orbital-color-picker-panel__sv-black {
    position: absolute;
    inset: 0;
}

.orbital-color-picker-panel__sv-white {
    background: linear-gradient(90deg, #fff 0%, rgba(255, 255, 255, 0) 100%);
}

.orbital-color-picker-panel__sv-black {
    background: linear-gradient(0deg, #000 0%, rgba(0, 0, 0, 0) 100%);
}

.orbital-color-picker-panel__sv-cursor {
    position: absolute;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    border: 2px solid #fff;
    box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.4);
    transform: translate(-50%, -50%);
}

.orbital-color-picker-panel__hue {
    width: 100%;
    margin-top: var(--orb-space-block-snudge);
}

.orbital-color-picker-panel__actions {
    display: flex;
    justify-content: flex-end;
    margin-top: var(--orb-space-block-snudge);
}

.orbital-color-picker-panel__action {
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    padding: 4px 10px;
    cursor: pointer;
}
"#
}
