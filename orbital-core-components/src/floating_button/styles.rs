pub fn floating_button_styles() -> &'static str {
    r#".orbital-floating-button {
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    box-sizing: border-box;
    font-family: var(--orb-type-family-sans);
    font-weight: var(--orb-type-weight-semibold);
    transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
        box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1),
        background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: var(--orb-elev-raised-md);
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
    padding: 0;
}

.orbital-floating-button:hover,
.orbital-floating-button:active {
    box-shadow: var(--orb-elev-floating);
}

.orbital-floating-button--primary {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-floating-button--primary:hover,
.orbital-floating-button--primary:active {
    background-color: var(--orb-color-brand-bg-hover);
}

.orbital-floating-button--secondary {
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
}

.orbital-floating-button--secondary:hover,
.orbital-floating-button--secondary:active {
    background-color: var(--orb-color-surface-canvas-hover);
}

.orbital-floating-button--rounded {
    border-radius: var(--orb-radius-floating);
}

.orbital-floating-button--circular {
    border-radius: var(--orb-radius-circular);
}

.orbital-floating-button--extended {
    border-radius: var(--orb-radius-floating);
    padding: 0 var(--orb-space-inline-lg);
    gap: var(--orb-space-inline-snudge);
}

.orbital-floating-button--small:not(.orbital-floating-button--extended) {
    width: 40px;
    height: 40px;
    min-width: 40px;
}

.orbital-floating-button--medium:not(.orbital-floating-button--extended) {
    width: 48px;
    height: 48px;
    min-width: 48px;
}

.orbital-floating-button--large:not(.orbital-floating-button--extended) {
    width: 56px;
    height: 56px;
    min-width: 56px;
}

.orbital-floating-button--small.orbital-floating-button--extended {
    min-height: 40px;
    font-size: var(--orb-type-size-xs);
}

.orbital-floating-button--medium.orbital-floating-button--extended {
    min-height: 48px;
    font-size: var(--orb-type-size-sm);
}

.orbital-floating-button--large.orbital-floating-button--extended {
    min-height: 56px;
    font-size: var(--orb-type-size-sm);
}

.orbital-floating-button--fixed {
    position: fixed;
    z-index: 1000;
}

.orbital-floating-button--disabled,
.orbital-floating-button--disabled:hover,
.orbital-floating-button--disabled:active {
    cursor: not-allowed;
    color: var(--orb-color-text-disabled);
    background-color: var(--orb-color-surface-disabled);
    box-shadow: none;
}

.orbital-floating-button__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    line-height: 1;
    pointer-events: none;
}

.orbital-floating-button__icon svg {
    pointer-events: none;
}

.orbital-floating-button svg {
    pointer-events: none;
}

.orbital-floating-button--small .orbital-floating-button__icon {
    font-size: 20px;
}

.orbital-floating-button--large .orbital-floating-button__icon {
    font-size: 28px;
}

.orbital-floating-button--extended .orbital-floating-button__icon {
    margin-inline-end: var(--orb-space-inline-xs);
}

.orbital-back-to-top.fade-in-scale-up-transition-leave-active {
    transform-origin: inherit;
    transition: opacity 0.2s cubic-bezier(0.4, 0, 1, 1),
        transform 0.2s cubic-bezier(0.4, 0, 1, 1);
}

.orbital-back-to-top.fade-in-scale-up-transition-enter-active {
    transform-origin: inherit;
    transition: opacity 0.2s cubic-bezier(0, 0, 0.2, 1),
        transform 0.2s cubic-bezier(0, 0, 0.2, 1);
}

.orbital-back-to-top.fade-in-scale-up-transition-enter-from,
.orbital-back-to-top.fade-in-scale-up-transition-leave-to {
    opacity: 0;
    transform: scale(0.9);
}

.orbital-back-to-top.fade-in-scale-up-transition-leave-from,
.orbital-back-to-top.fade-in-scale-up-transition-enter-to {
    opacity: 1;
    transform: scale(1);
}
"#
}
