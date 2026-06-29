use super::sections::SegmentKind;

/// Inline width/flex overrides for segment inputs (not turf-hashed).
pub fn segmented_field_styles() -> &'static str {
    r#"
.orb-picker-segmented-field-host {
    display: inline-flex;
    width: fit-content;
    min-width: max-content;
    max-width: none;
    flex-shrink: 0;
}

.orb-date-field.orbital-input,
.orb-time-field.orbital-input,
.orb-datetime-field.orbital-input {
    display: inline-flex;
    align-items: center;
    flex-wrap: nowrap;
    flex-shrink: 0;
    gap: 0;
    width: fit-content;
    min-width: max-content;
    max-width: none;
    overflow: visible;
}

.orb-date-field__segment,
.orb-time-field__segment,
.orb-datetime-field__segment {
    flex: 0 0 auto;
    flex-grow: 0;
    flex-shrink: 0;
    box-sizing: content-box;
    width: 3ch;
    min-width: 3ch;
    padding: 0 var(--orb-space-inline-2xs);
    text-align: center;
    color: var(--orb-color-text-primary);
    background-color: transparent;
    cursor: text;
}

.orbital-input .orb-date-field__segment.orbital-input__input,
.orbital-input .orb-time-field__segment.orbital-input__input,
.orbital-input .orb-datetime-field__segment.orbital-input__input {
    flex: 0 0 auto;
    flex-grow: 0;
    flex-shrink: 0;
    min-width: unset;
    align-self: center;
}

.orb-date-field__segment--year,
.orb-datetime-field__segment--year {
    width: 4.5ch;
    min-width: 36px;
}

.orb-date-field__segment--month,
.orb-date-field__segment--day,
.orb-datetime-field__segment--month,
.orb-datetime-field__segment--day {
    width: 3ch;
    min-width: 3ch;
}

.orb-time-field__segment--hour,
.orb-time-field__segment--minute,
.orb-datetime-field__segment--hour,
.orb-datetime-field__segment--minute {
    width: 3ch;
    min-width: 3ch;
}

.orb-time-field__segment--meridiem,
.orb-datetime-field__segment--meridiem {
    width: 3.25ch;
    min-width: 3.25ch;
}

.orbital-input--small.orb-date-field,
.orbital-input--small.orb-time-field,
.orbital-input--small.orb-datetime-field {
    min-width: max-content;
}

.orbital-input--small .orb-date-field__segment,
.orbital-input--small .orb-time-field__segment,
.orbital-input--small .orb-datetime-field__segment {
    padding: 0 1px;
}

.orbital-input--small .orb-date-field__segment--month,
.orbital-input--small .orb-date-field__segment--day,
.orbital-input--small .orb-datetime-field__segment--month,
.orbital-input--small .orb-datetime-field__segment--day,
.orbital-input--small .orb-time-field__segment--hour,
.orbital-input--small .orb-time-field__segment--minute,
.orbital-input--small .orb-datetime-field__segment--hour,
.orbital-input--small .orb-datetime-field__segment--minute {
    width: 3ch;
    min-width: 24px;
}

.orbital-input--small .orb-date-field__segment--year,
.orbital-input--small .orb-datetime-field__segment--year {
    width: 4.5ch;
    min-width: 36px;
}

.orbital-input--small .orb-time-field__segment--meridiem,
.orbital-input--small .orb-datetime-field__segment--meridiem {
    width: 3.25ch;
    min-width: 28px;
}

/* US/ISO date fields: month + day + year + separators */
.orbital-input--small.orb-date-field {
    min-width: calc(24px + 24px + 36px + 2ch);
}

.orbital-input--large .orb-date-field__segment,
.orbital-input--large .orb-time-field__segment,
.orbital-input--large .orb-datetime-field__segment {
    padding: 0 var(--orb-space-inline-xs);
}

.orb-date-field__separator,
.orb-time-field__separator,
.orb-datetime-field__separator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    align-self: center;
    flex: 0 0 auto;
    flex-shrink: 0;
    padding: 0 1px;
    line-height: 1;
    color: var(--orb-color-text-tertiary);
    user-select: none;
}
"#
}

/// Guaranteed inline styles per segment kind (wins over flex input defaults).
pub fn segment_input_style(kind: SegmentKind) -> &'static str {
    match kind {
        SegmentKind::Year => "width:4.5ch;min-width:36px;flex:0 0 auto;box-sizing:content-box;",
        SegmentKind::Meridiem => {
            "width:3.25ch;min-width:3.25ch;flex:0 0 auto;box-sizing:content-box;"
        }
        SegmentKind::Month | SegmentKind::Day | SegmentKind::Hour | SegmentKind::Minute => {
            "width:3ch;min-width:24px;flex:0 0 auto;box-sizing:content-box;"
        }
    }
}
