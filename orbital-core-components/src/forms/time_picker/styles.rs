pub fn time_picker_styles() -> &'static str {
    r#"
.orbital-time-picker {
    display: inline-flex;
    width: 100%;
}

.orbital-time-picker__trigger {
    width: 100%;
    min-height: 32px;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    text-align: left;
    padding: 0 var(--orb-space-inline-mnudge);
    cursor: pointer;
}

.orbital-time-picker__trigger:hover {
    border-color: var(--orb-color-border-default-hover);
}

.orbital-time-picker__trigger:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: 1px;
}

.orbital-time-picker--disabled .orbital-time-picker__trigger {
    background: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
    border-color: var(--orb-color-border-disabled);
    cursor: not-allowed;
}

.orbital-time-picker-panel {
    min-width: 280px;
    padding: var(--orb-space-block-snudge);
    border-radius: var(--orb-radius-md);
    border: 1px solid var(--orb-color-border-default);
    background: var(--orb-color-surface-canvas);
    box-shadow: var(--orb-elev-floating);
}

.orbital-time-picker-panel__columns {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--orb-space-inline-xs);
}

.orbital-time-picker-panel__column {
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-sm);
}

.orbital-time-picker-panel__column--period {
    grid-column: span 3;
}

.orbital-time-picker-panel__scroll {
    max-height: 168px;
    overflow-y: auto;
    padding: 2px;
    scrollbar-width: thin;
    scrollbar-color: var(--orb-color-text-tertiary) transparent;
}

.orbital-time-picker-panel__scroll::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

.orbital-time-picker-panel__scroll::-webkit-scrollbar-thumb {
    background-color: var(--orb-color-text-tertiary);
    border-radius: var(--orb-radius-circular);
}

.orbital-time-picker-panel__scroll::-webkit-scrollbar-thumb:hover {
    background-color: var(--orb-color-text-secondary);
}

.orbital-time-picker-panel__scroll::-webkit-scrollbar-track {
    background: transparent;
}

.orbital-time-picker-panel__item {
    display: block;
    width: 100%;
    border: 0;
    border-radius: var(--orb-radius-sm);
    background: transparent;
    color: var(--orb-color-text-primary);
    padding: 4px 6px;
    text-align: center;
    cursor: pointer;
}

.orbital-time-picker-panel__item:hover {
    background: var(--orb-color-subtle-bg-hover);
}

.orbital-time-picker-panel__item--selected {
    background: var(--orb-color-brand-bg-subtle);
    color: var(--orb-color-text-on-brand);
}

.orbital-time-picker-panel__actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--orb-space-inline-xs);
    margin-top: var(--orb-space-block-snudge);
}

.orbital-time-picker-panel__action {
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    padding: 4px 10px;
    cursor: pointer;
}

.orbital-time-picker-panel__action--primary {
    background: var(--orb-color-brand-bg);
    border-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}
"#
}
