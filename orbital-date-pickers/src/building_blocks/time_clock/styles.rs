/// Stable stylesheet for [`super::TimeClock`] (not turf-hashed so SVG + dial classes match markup).
pub fn time_clock_styles() -> &'static str {
    r#"
.orb-picker-time-clock {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--orb-space-block-md);
    width: fit-content;
    font-family: var(--orb-type-family-sans);
}

.orb-picker-time-clock__face {
    --orb-clock-face-size: 260px;
    --orb-clock-marker-size: 36px;
    position: relative;
    width: var(--orb-clock-face-size);
    height: var(--orb-clock-face-size);
}

.orb-picker-layout-density-compact .orb-picker-time-clock__face,
.orb-picker-time-clock--density-compact .orb-picker-time-clock__face {
    --orb-clock-face-size: 220px;
}

.orb-picker-layout-density-spacious .orb-picker-time-clock__face,
.orb-picker-time-clock--density-spacious .orb-picker-time-clock__face {
    --orb-clock-face-size: 300px;
}

.orb-picker-time-clock__svg {
    display: block;
    width: 100%;
    height: 100%;
}

.orb-picker-time-clock__circle {
    fill: var(--orb-color-surface-canvas);
    stroke: var(--orb-color-border-default);
    stroke-width: 1;
}

.orb-picker-time-clock__tick {
    stroke: var(--orb-color-border-subtle);
    stroke-width: 1;
    stroke-linecap: round;
}

.orb-picker-time-clock__center {
    fill: var(--orb-color-brand-stroke);
}

.orb-picker-time-clock__number-slot,
.orb-picker-time-clock__marker-slot {
    position: absolute;
    transform: translate(-50%, -50%);
    z-index: 2;
}

.orb-picker-time-clock__number {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--orb-clock-marker-size);
    height: var(--orb-clock-marker-size);
    padding: 0;
    border: none;
    border-radius: var(--orb-radius-circular);
    background: transparent;
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    line-height: 1;
    cursor: pointer;
    position: relative;
    z-index: 1;
}

.orb-picker-time-clock__number:hover:not(:disabled):not(.orb-picker-time-clock__number--disabled) {
    background-color: var(--orb-color-surface-canvas-hover);
}

.orb-picker-time-clock__number--selected {
    color: var(--orb-color-text-on-brand);
    background: transparent;
}

.orb-picker-time-clock__number--selected:hover:not(:disabled) {
    background: transparent;
    color: var(--orb-color-text-on-brand);
}

.orb-picker-time-clock__number--disabled,
.orb-picker-time-clock__number:disabled {
    pointer-events: none;
    color: var(--orb-color-text-disabled);
    cursor: default;
}

.orb-picker-time-clock__overlay {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    z-index: 1;
    touch-action: none;
    user-select: none;
    cursor: pointer;
}

.orb-picker-time-clock__overlay--dragging {
    cursor: grabbing;
}

.orb-picker-time-clock__pointer {
    position: absolute;
    left: 50%;
    bottom: 50%;
    width: 2px;
    height: calc((var(--orb-clock-face-size) - var(--orb-clock-marker-size) - 2px) / 2);
    margin-left: -1px;
    transform-origin: center bottom;
    pointer-events: none;
    z-index: 1;
}

.orb-picker-time-clock__pointer-line {
    width: 100%;
    height: 100%;
    background-color: var(--orb-color-brand-stroke);
    border-radius: 1px;
}

.orb-picker-time-clock__pointer-thumb {
    position: absolute;
    top: calc(-1 * var(--orb-clock-marker-size) / 2);
    left: 50%;
    width: var(--orb-clock-marker-size);
    height: var(--orb-clock-marker-size);
    margin-left: calc(-1 * var(--orb-clock-marker-size) / 2);
    border: none;
    border-radius: var(--orb-radius-circular);
    box-sizing: border-box;
    background-color: var(--orb-color-brand-bg);
}

.orb-picker-time-clock__meridiem {
    display: flex;
    gap: var(--orb-space-inline-sm);
}

.orb-picker-time-clock__meridiem .orbital-button-group {
    gap: var(--orb-space-inline-sm);
}
"#
}
