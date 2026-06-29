/// Compiled input stylesheet and stable `orbital-input*` class names (excluded from turf hashing).
///
pub fn input_styles() -> &'static str {
    r#"
.orbital-input {
    display: inline-flex;
    align-items: center;
    flex-wrap: nowrap;
    position: relative;
    box-sizing: border-box;
    min-height: 32px;
    gap: var(--orb-space-inline-2xs);
    vertical-align: middle;
    background-color: var(--orb-color-surface-canvas);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-regular);
    line-height: var(--orb-type-line-md);
    border: 1px solid var(--orb-color-border-default);
    border-bottom-color: var(--orb-color-border-accessible);
    border-radius: var(--orb-radius-md);
}
.orbital-input--small {
    min-height: 24px;
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
}
.orbital-input--large {
    min-height: 40px;
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-lg);
    gap: var(--orb-space-inline-snudge);
}
.orbital-input:hover {
    border-color: var(--orb-color-border-default-hover);
    border-bottom-color: var(--orb-color-border-accessible-hover);
}
.orbital-input:focus-within {
    outline: transparent solid 2px;
}
.orbital-input:active,
.orbital-input:focus-within {
    border-color: var(--orb-color-border-default-pressed);
    border-bottom-color: var(--orb-color-border-accessible-pressed);
}
.orbital-input::after {
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
    clip-path: inset(calc(100% - 2px) 0px 0px);
    transform: scaleX(0);
    transition-property: transform;
    transition-duration: var(--orb-motion-duration-2xs);
    transition-timing-function: var(--orb-motion-ease-accelerate);
}
.orbital-input:focus-within::after {
    transform: scaleX(1);
    transition-property: transform;
    transition-duration: var(--orb-motion-duration-md);
    transition-timing-function: var(--orb-motion-ease-decelerate);
}
.orbital-input:focus-within:active::after {
    border-bottom-color: var(--orb-color-brand-compound-stroke-pressed);
}
.orbital-input__input {
    align-self: stretch;
    box-sizing: border-box;
    flex-grow: 1;
    min-width: 0px;
    border-style: none;
    padding: 0 var(--orb-space-inline-md);
    color: var(--orb-color-text-primary);
    background-color: transparent;
    outline-style: none;
    font-family: inherit;
    font-size: inherit;
    font-weight: inherit;
    line-height: inherit;
}
.orbital-input--small .orbital-input__input {
    padding: 0 var(--orb-space-inline-sm);
}
.orbital-input--large .orbital-input__input {
    padding: 0 calc(var(--orb-space-inline-md) + var(--orb-space-inline-snudge));
}
.orbital-input__input::placeholder {
    color: var(--orb-color-text-quaternary);
    opacity: 1;
}
.orbital-input--prefix {
    padding-left: var(--orb-space-inline-mnudge);
}
.orbital-input--small.orbital-input--prefix {
    padding-left: var(--orb-space-inline-snudge);
}
.orbital-input--large.orbital-input--prefix {
    padding-left: var(--orb-space-inline-md);
}
.orbital-input--prefix > .orbital-input__input {
    padding-left: var(--orb-space-inline-2xs);
}
.orbital-input--large.orbital-input--prefix > .orbital-input__input {
    padding-left: var(--orb-space-inline-snudge);
}
.orbital-input--suffix {
    padding-right: var(--orb-space-inline-mnudge);
}
.orbital-input--small.orbital-input--suffix {
    padding-right: var(--orb-space-inline-snudge);
}
.orbital-input--large.orbital-input--suffix {
    padding-right: var(--orb-space-inline-md);
}
.orbital-input--suffix > .orbital-input__input {
    padding-right: var(--orb-space-inline-2xs);
}
.orbital-input--large.orbital-input--suffix > .orbital-input__input {
    padding-right: var(--orb-space-inline-snudge);
}
.orbital-input__prefix,
.orbital-input__suffix {
    box-sizing: border-box;
    color: var(--orb-color-text-tertiary);
    display: flex;
}
.orbital-input.orbital-input--disabled {
    border-color: var(--orb-color-border-disabled);
    border-bottom-color: var(--orb-color-border-disabled);
    background-color: var(--orb-color-transparent-bg);
    cursor: not-allowed;
}
.orbital-input--disabled > .orbital-input__input {
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}
.orbital-input--disabled > .orbital-input__input::placeholder {
    color: var(--orb-color-text-disabled);
}
"#
}
