pub fn breadcrumb_styles() -> &'static str {
    r#"
.orbital-breadcrumb__list {
    list-style-type: none;
    display: flex;
    align-items: center;
    margin: 0px;
    padding: 0px;
}

.orbital-breadcrumb-item {
    display: flex;
    align-items: center;
    color: var(--orb-color-text-secondary);
    box-sizing: border-box;
    flex-wrap: nowrap;
}

.orbital-breadcrumb-button {
    align-items: center;
    box-sizing: border-box;
    display: inline-flex;
    justify-content: center;
    text-decoration-line: none;
    vertical-align: middle;
    margin: 0px;
    overflow: hidden;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    font-family: var(--orb-type-family-sans);
    outline-style: none;
    border-radius: var(--orb-radius-md);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    transition-duration: var(--orb-motion-duration-xs);
    transition-property: background, border, color;
    transition-timing-function: var(--orb-motion-ease-standard);

    flex-wrap: nowrap;
    min-width: unset;
    height: 32px;
    color: var(--orb-color-text-secondary);
    background-color: var(--orb-color-subtle-bg);
    border-color: transparent;
    font-weight: var(--orb-type-weight-regular);
    padding: var(--orb-space-inline-snudge);
}

.orbital-breadcrumb-button:hover {
    color: var(--orb-color-text-secondary-hover);
    background-color: var(--orb-color-subtle-bg-hover);
    cursor: pointer;
}

.orbital-breadcrumb-button:hover:active {
    color: var(--orb-color-text-secondary-pressed);
    background-color: var(--orb-color-subtle-bg-pressed);
    outline-style: none;
}

.orbital-breadcrumb-button--current {
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-breadcrumb-button--current:hover {
    color: var(--orb-color-text-secondary);
    background-color: var(--orb-color-transparent-bg);
    cursor: auto;
}

.orbital-breadcrumb-button--current:hover:active {
    color: var(--orb-color-text-secondary);
    background-color: var(--orb-color-transparent-bg);
    outline-style: none;
}

.orbital-breadcrumb-divider {
    font-size: 16px;
    display: flex;
}

.orbital-breadcrumb-divider > svg {
    display: inline;
    line-height: 0;
}
"#
}
