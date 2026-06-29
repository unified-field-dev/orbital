/// Compiled date picker stylesheet and stable `orbital-date-picker*` class names.
pub fn date_picker_styles() -> &'static str {
    r#"
.orbital-date-picker {
    display: inline-flex;
    width: 100%;
}

.orbital-date-picker__trigger {
    width: 100%;
}

.orbital-date-picker__suffix-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: inherit;
    cursor: pointer;
    padding: 0;
}

.orbital-date-picker__panel {
    min-width: 280px;
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    box-shadow: var(--orb-elev-floating);
    padding: 12px;
}

.orbital-date-picker__panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 10px;
}

.orbital-date-picker__nav-button,
.orbital-date-picker__title-button {
    border: 1px solid var(--orb-color-border-default);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
    padding: 4px 8px;
    cursor: pointer;
}

.orbital-date-picker__title-button {
    flex: 1;
    font-weight: 600;
}

.orbital-date-picker__weekday-row {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    gap: 4px;
    margin-bottom: 6px;
    font-size: 12px;
    color: var(--orb-color-text-tertiary);
}

.orbital-date-picker__weekday-row span {
    text-align: center;
}

.orbital-date-picker__date-grid,
.orbital-date-picker__month-grid,
.orbital-date-picker__year-grid {
    display: grid;
    gap: 4px;
}

.orbital-date-picker__date-grid {
    grid-template-columns: repeat(7, minmax(0, 1fr));
}

.orbital-date-picker__month-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
}

.orbital-date-picker__year-grid {
    grid-template-columns: repeat(4, minmax(0, 1fr));
}

.orbital-date-picker__day-button,
.orbital-date-picker__month-button,
.orbital-date-picker__year-button {
    border: 1px solid transparent;
    background: transparent;
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
    min-height: 32px;
    cursor: pointer;
    text-align: center;
}

.orbital-date-picker__day-button:hover,
.orbital-date-picker__month-button:hover,
.orbital-date-picker__year-button:hover {
    background: var(--orb-color-surface-canvas-hover);
}

.orbital-date-picker__day-button--outside {
    color: var(--orb-color-text-disabled);
}

.orbital-date-picker__day-button--today {
    border-color: var(--orb-color-brand-stroke);
}

.orbital-date-picker__day-button--disabled,
.orbital-date-picker__day-button:disabled {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
    pointer-events: none;
}

.orb-picker-shortcuts {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 10px;
    padding-top: 10px;
    border-top: 1px solid var(--orb-color-border-subtle);
}

.orb-picker-shortcuts__button {
    border: 1px solid var(--orb-color-border-default);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
    padding: 4px 10px;
    font-size: 12px;
    cursor: pointer;
}

.orb-picker-shortcuts__button:hover:not(:disabled) {
    background: var(--orb-color-surface-canvas-hover);
}

.orb-picker-shortcuts__button:disabled {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
    opacity: 0.6;
}
"#
}
