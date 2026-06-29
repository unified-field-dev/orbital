pub fn accordion_styles() -> &'static str {
    r#"
.orbital-accordion-header {
    margin: 0;
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
}

.orbital-accordion-header__button {
    display: flex;
    align-items: center;
    position: relative;
    padding-left: var(--orb-space-inline-mnudge);
    padding-right: var(--orb-space-inline-md);
    padding-top: 0px;
    padding-bottom: 0px;
    width: 100%;
    min-height: 44px;
    text-align: unset;
    line-height: var(--orb-type-line-md);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-regular);
    background-color: inherit;
    color: inherit;
    border-width: 0px;
    appearance: button;
    overflow: visible;
    box-sizing: border-box;
    cursor: pointer;
}

.orbital-accordion-header__expand-icon {
    display: flex;
    align-items: center;
    height: 100%;
    padding-right: var(--orb-space-inline-sm);
    font-size: var(--orb-type-size-lg);
    line-height: var(--orb-type-line-xl);
}

.orbital-accordion-header__expand-icon > svg {
    display: inline;
    line-height: 0;
}

.orbital-accordion-panel {
    margin: 0 var(--orb-space-inline-md);
}

.orbital-accordion-panel-enter-from,
.orbital-accordion-panel-enter-to {
    opacity: 1;
}

.orbital-accordion-panel-leave-to,
.orbital-accordion-panel-enter-from {
    opacity: 0;
    max-height: 0;
}

.orbital-accordion-panel-leave-active {
    overflow: hidden;
    transition: max-height 0.15s cubic-bezier(0.4, 0, 0.2, 1) 0s,
        opacity 0.15s cubic-bezier(0, 0, 0.2, 1) 0s,
        padding-top 0.15s cubic-bezier(0.4, 0, 0.2, 1) 0s;
}

.orbital-accordion-panel-enter-active {
    overflow: hidden;
    transition: max-height 0.15s cubic-bezier(0.4, 0, 0.2, 1),
        opacity 0.15s cubic-bezier(0.4, 0, 1, 1),
        padding-top 0.15s cubic-bezier(0.4, 0, 0.2, 1);
}
"#
}
