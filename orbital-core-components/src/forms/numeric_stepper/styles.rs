/// Compiled numeric stepper stylesheet and stable `orbital-numeric-stepper*` class names.
///
pub fn numeric_stepper_styles() -> &'static str {
    r#"
.orbital-numeric-stepper {
    display: inline-grid;
    grid-template-columns: 1fr 24px;
    grid-template-rows: 1fr 1fr;
    column-gap: var(--orb-space-inline-xs);
    row-gap: 0;
    position: relative;
    isolation: isolate;
    background-color: var(--orb-color-surface-canvas);
    min-height: 32px;
    padding: 0 0 0 var(--orb-space-inline-mnudge);
    border: 1px solid var(--orb-color-border-default);
    border-bottom-color: var(--orb-color-border-accessible);
    border-radius: var(--orb-radius-md);
}
.orbital-numeric-stepper--small {
    min-height: 24px;
    padding-left: var(--orb-space-inline-sm);
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
    font-weight: var(--orb-type-weight-regular);
    font-family: var(--orb-type-family-sans);
}
.orbital-numeric-stepper:hover {
    border-color: var(--orb-color-border-default-hover);
    border-bottom-color: var(--orb-color-border-accessible-hover);
}
.orbital-numeric-stepper:focus-within {
    outline: transparent solid 2px;
}
.orbital-numeric-stepper:active,
.orbital-numeric-stepper:focus-within {
    border-color: var(--orb-color-border-default-pressed);
    border-bottom-color: var(--orb-color-border-accessible-pressed);
}
.orbital-numeric-stepper::after {
    box-sizing: border-box;
    content: "";
    position: absolute;
    left: -1px;
    bottom: -1px;
    right: -1px;
    height: max(2px, var(--orb-radius-md));
    border-bottom-left-radius: var(--orb-radius-md);
    border-bottom-right-radius: var(--orb-radius-md);
    border-bottom: 2px solid var(--orb-color-brand-compound-stroke);
    clip-path: inset(calc(100% - 2px) 0 0);
    transform: scaleX(0);
    transition-property: transform;
    transition-duration: var(--orb-motion-duration-2xs);
    transition-timing-function: var(--orb-motion-ease-accelerate);
}
.orbital-numeric-stepper:focus-within::after {
    transform: scaleX(1);
    transition-property: transform;
    transition-duration: var(--orb-motion-duration-md);
    transition-timing-function: var(--orb-motion-ease-decelerate);
}
.orbital-numeric-stepper:focus-within:active::after {
    border-bottom-color: var(--orb-color-brand-compound-stroke-pressed);
}
.orbital-numeric-stepper__input {
    grid-area: 1 / 1 / 3 / 2;
    outline-style: none;
    border: 0;
    padding: 0;
    color: var(--orb-color-text-primary);
    background-color: transparent;
    font-family: inherit;
    font-size: inherit;
    font-weight: inherit;
    line-height: inherit;
    width: 100%;
}
.orbital-numeric-stepper__input:disabled {
    color: var(--orb-color-text-disabled);
    background-color: var(--orb-color-transparent-bg);
    cursor: not-allowed;
}
.orbital-numeric-stepper__increment-button,
.orbital-numeric-stepper__decrement-button {
    display: inline-flex;
    width: 24px;
    align-items: center;
    justify-content: center;
    border: 0;
    position: absolute;
    outline-style: none;
    height: 16px;
    background-color: transparent;
    color: var(--orb-color-text-tertiary);
    grid-column-start: 2;
    border-radius: 0;
    padding: 0 5px;
}
.orbital-numeric-stepper__increment-button:enabled:hover,
.orbital-numeric-stepper__decrement-button:enabled:hover {
    cursor: pointer;
    color: var(--orb-color-text-tertiary-hover);
    background-color: var(--orb-color-subtle-bg-hover);
}
.orbital-numeric-stepper__increment-button:enabled:active,
.orbital-numeric-stepper__decrement-button:enabled:active {
    color: var(--orb-color-text-tertiary-pressed);
    background-color: var(--orb-color-subtle-bg-pressed);
}
.orbital-numeric-stepper__increment-button:active,
.orbital-numeric-stepper__decrement-button:active {
    outline-style: none;
}
.orbital-numeric-stepper__increment-button {
    grid-row-start: 1;
    padding-top: 4px;
    padding-bottom: 1px;
    border-top-right-radius: var(--orb-radius-md);
}
.orbital-numeric-stepper--small .orbital-numeric-stepper__increment-button {
    padding: 3px 6px 0 4px;
    height: 12px;
}
.orbital-numeric-stepper__decrement-button {
    padding-bottom: 4px;
    padding-top: 1px;
    grid-row-start: 2;
    border-bottom-right-radius: var(--orb-radius-md);
}
.orbital-numeric-stepper--small .orbital-numeric-stepper__decrement-button {
    padding: 0 6px 3px 4px;
    height: 12px;
}
.orbital-numeric-stepper__increment-button:disabled,
.orbital-numeric-stepper__decrement-button:disabled {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}
.orbital-numeric-stepper__increment-button--disabled:enabled:hover,
.orbital-numeric-stepper__decrement-button--disabled:enabled:hover,
.orbital-numeric-stepper__increment-button--disabled:enabled:active,
.orbital-numeric-stepper__decrement-button--disabled:enabled:active {
    background-color: transparent;
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}
.orbital-numeric-stepper--disabled,
.orbital-numeric-stepper--disabled:hover {
    background-color: var(--orb-color-transparent-bg);
    border-color: var(--orb-color-border-disabled);
    border-bottom-color: var(--orb-color-border-disabled);
    cursor: not-allowed;
}
"#
}
