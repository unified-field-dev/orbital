/// Stable stylesheet for [`super::DateRangeField`] (not turf-hashed).
pub fn date_range_field_styles() -> &'static str {
    r#"
.orb-date-range-field,
.orb-time-range-field,
.orb-datetime-range-field {
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.orb-picker-range-field__separator {
    color: var(--orb-color-text-secondary);
    user-select: none;
    flex-shrink: 0;
}
"#
}
