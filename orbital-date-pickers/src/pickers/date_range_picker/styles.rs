/// Stable stylesheet for [`super::DateRangePicker`] (not turf-hashed).
pub fn date_range_picker_styles() -> &'static str {
    r#"
.orb-picker-range-picker__trigger {
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.orb-picker-range-picker__open-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    cursor: pointer;
}

.orb-picker-range-picker__open-btn:hover:not(:disabled) {
    border-color: var(--orb-color-border-default-hover);
    background: var(--orb-color-surface-canvas-hover);
}

.orb-picker-range-picker__open-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.orb-picker-range-picker__panel {
    box-sizing: border-box;
    max-width: min(calc(100vw - 24px), 640px);
}

/* Inline range calendars use fixed heights for density; popover panels should shrink to content. */
.orb-picker-range-picker__panel .orb-picker-layout-density-compact .orb-picker-range-calendar .orbital-calendar,
.orb-picker-range-picker__panel .orb-picker-layout-density-spacious .orb-picker-range-calendar .orbital-calendar,
.orb-picker-range-picker__panel .orb-picker-range-calendar .orbital-calendar {
    height: auto;
}

.orb-picker-range-picker__panel .orbital-calendar__dates {
    flex: 0 0 auto;
    grid-auto-rows: minmax(2.25rem, auto);
}

.orb-picker-range-picker__panel .orb-picker-range-calendar {
    gap: 12px;
}

.orbital-positioning-content:has(.orb-picker-range-picker__panel) {
    max-height: calc(100dvh - 16px);
    overflow-y: auto;
    overscroll-behavior: contain;
    -webkit-overflow-scrolling: touch;
}

@media (max-width: 640px) {
    .orb-picker-range-picker__panel .orb-picker-range-calendar {
        flex-direction: column;
    }
}
"#
}
