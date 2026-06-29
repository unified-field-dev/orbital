/// Compiled calendar stylesheet and stable `orbital-calendar*` class names.
///
pub fn calendar_styles() -> &'static str {
    r#"
.orbital-calendar {
    --orb-calendar-cell-min: 2.75rem;
    --orb-calendar-header-title-min: 11.5rem;
    display: flex;
    flex-direction: column;
    width: 100%;
    min-width: calc(7 * var(--orb-calendar-cell-min));
    height: 720px;
}

.orbital-calendar__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-inline-md, 16px);
    padding-bottom: 16px;
}

.orbital-calendar__header-title {
    box-sizing: border-box;
    min-width: var(--orb-calendar-header-title-min);
    flex-shrink: 0;
    font-size: 22px;
    font-weight: 500;
    line-height: var(--orb-type-line-md, 1.375);
    white-space: nowrap;
}

.orbital-calendar__weekdays {
    display: grid;
    grid-template-columns: repeat(7, minmax(var(--orb-calendar-cell-min), 1fr));
    border-top: 1px solid;
    border-left: 1px solid;
    border-color: var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md) var(--orb-radius-md) 0 0;
}

.orbital-calendar__weekday {
    padding: 6px 8px;
    border-right: 1px solid;
    border-bottom: 1px solid;
    border-color: var(--orb-color-border-subtle);
    font-size: var(--orb-type-size-sm, 0.875rem);
    font-weight: 500;
    color: var(--orb-color-text-secondary);
    text-align: center;
}

.orbital-calendar__dates {
    flex: 1;
    display: grid;
    grid-template-columns: repeat(7, minmax(var(--orb-calendar-cell-min), 1fr));
    grid-auto-rows: 1fr;
    border-left: 1px solid;
    border-color: var(--orb-color-border-subtle);
    border-radius: 0 0 var(--orb-radius-md) var(--orb-radius-md);
}

.orbital-calendar-item {
    position: relative;
    box-sizing: border-box;
    min-width: 0;
    padding: 8px 12px;
    border-right: 1px solid;
    border-bottom: 1px solid;
    border-color: var(--orb-color-border-subtle);
    cursor: pointer;
}

.orbital-calendar-item:hover {
    background-color: var(--orb-color-surface-canvas-hover);
}

.orbital-calendar-item--other-month {
    color: var(--orb-color-text-disabled);
}

.orbital-calendar-item__header {
    display: flex;
    align-items: flex-start;
}

.orbital-calendar-item--today .orbital-calendar-item__header-day {
    display: flex;
    justify-content: center;
    align-items: center;
    color: white;
    background-color: var(--orb-color-brand-bg);
    border-radius: 50%;
    margin-left: -0.4em;
    margin-top: -0.3em;
    width: 1.8em;
    height: 1.8em;
}

.orbital-calendar-item__bar {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: transparent;
    height: 3px;
}

.orbital-calendar-item--selected .orbital-calendar-item__bar {
    background-color: var(--orb-color-brand-bg);
}

.orbital-calendar-item--disabled {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
    pointer-events: none;
}

.orbital-calendar-item--disabled:hover {
    background-color: transparent;
}
"#
}
