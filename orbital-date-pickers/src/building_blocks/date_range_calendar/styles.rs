/// Stable stylesheet for [`super::DateRangeCalendar`] (not turf-hashed).
pub fn date_range_calendar_styles() -> &'static str {
    r#"
.orb-picker-range-calendar {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
    align-items: flex-start;
}

.orb-picker-range-calendar__nav {
    flex: 0 0 100%;
    margin-bottom: 4px;
}

.orb-picker-range-calendar__panel {
    display: flex;
    flex-direction: column;
    width: fit-content;
    min-width: 0;
}

.orb-picker-layout-density-compact .orb-picker-range-calendar .orbital-calendar {
    height: 560px;
}

.orb-picker-layout-density-spacious .orb-picker-range-calendar .orbital-calendar {
    height: 840px;
}

.orb-picker-layout-density-compact .orb-picker-range-calendar .orbital-calendar-item {
    padding: 4px 6px;
}

.orb-picker-layout-density-spacious .orb-picker-range-calendar .orbital-calendar-item {
    padding: 12px 16px;
}

.orbital-calendar-item--in-range,
.orbital-calendar-item--preview-range {
    background-color: var(--orb-color-brand-subtle);
}

.orbital-calendar-item--preview-range {
    opacity: 0.85;
}

.orbital-calendar-item--in-range .orbital-calendar-item__bar,
.orbital-calendar-item--preview-range .orbital-calendar-item__bar {
    background-color: var(--orb-color-brand-bg);
}

.orbital-calendar-item--range-start,
.orbital-calendar-item--range-end {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-calendar-item--range-start .orbital-calendar-item__header-day,
.orbital-calendar-item--range-end .orbital-calendar-item__header-day {
    color: var(--orb-color-text-on-brand);
}

.orbital-calendar-item--range-start.orbital-calendar-item--today .orbital-calendar-item__header-day,
.orbital-calendar-item--range-end.orbital-calendar-item--today .orbital-calendar-item__header-day {
    background-color: var(--orb-color-text-on-brand);
    color: var(--orb-color-brand-bg);
}

.orbital-calendar-item--range-start .orbital-calendar-item__bar,
.orbital-calendar-item--range-end .orbital-calendar-item__bar {
    background-color: var(--orb-color-text-on-brand);
}

.orbital-calendar-item--in-range:hover,
.orbital-calendar-item--preview-range:hover {
    background-color: var(--orb-color-brand-subtle);
}

.orbital-calendar-item--range-start:hover,
.orbital-calendar-item--range-end:hover {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}
"#
}
