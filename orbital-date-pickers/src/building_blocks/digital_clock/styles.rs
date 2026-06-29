/// Stable stylesheet for [`super::DigitalClock`] (not turf-hashed).
pub fn digital_clock_styles() -> &'static str {
    r#"
.orb-picker-digital-clock {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    width: 220px;
    font-family: var(--orb-type-family-sans);
}

.orb-picker-digital-clock--density-compact {
    width: 200px;
}

.orb-picker-digital-clock--density-spacious {
    width: 240px;
}

.orb-picker-digital-clock__readout {
    box-sizing: border-box;
    min-height: 40px;
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    font-size: var(--orb-type-size-lg);
    font-weight: var(--orb-type-weight-semibold, 600);
    line-height: var(--orb-type-line-md);
    text-align: center;
}

.orb-picker-digital-clock__readout--placeholder {
    color: var(--orb-color-text-tertiary);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-md);
}

.orb-picker-digital-clock__scroll {
    height: 280px;
    max-height: 280px;
    box-sizing: border-box;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-canvas);
}

.orb-picker-digital-clock--density-compact .orb-picker-digital-clock__scroll {
    height: 220px;
    max-height: 220px;
}

.orb-picker-digital-clock--density-spacious .orb-picker-digital-clock__scroll {
    height: 340px;
    max-height: 340px;
}

.orb-picker-digital-clock__item {
    font-size: var(--orb-type-size-md);
}

.orb-picker-digital-clock--density-compact .orb-picker-digital-clock__item {
    font-size: var(--orb-type-size-sm);
}

.orb-picker-digital-clock--density-spacious .orb-picker-digital-clock__item {
    font-size: var(--orb-type-size-lg);
}
"#
}
