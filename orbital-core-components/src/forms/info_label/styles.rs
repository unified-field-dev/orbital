/// Compiled info-label stylesheet and stable `orbital-info-label*` class names.
///
pub fn info_label_styles() -> &'static str {
    r#"
.orbital-info-label {
    display: inline-flex;
    align-items: center;
    gap: 2px;
}

.orbital-info-label__label {
    color: inherit;
    cursor: inherit;
    vertical-align: top;
}

.orbital-info-label__info-button {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    text-decoration-line: none;
    box-sizing: border-box;
    margin: calc(0px - var(--orb-space-block-2xs)) 0;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-xs);
    color: var(--orb-color-text-secondary);
    background-color: var(--orb-color-transparent-bg);
    border-style: none;
    border-radius: var(--orb-radius-md);
    line-height: 1;
}

.orbital-info-label__info-button .orbital-icon {
    font-size: 14px;
}

.orbital-popover-trigger--open > .orbital-info-label__info-button,
.orbital-info-label__info-button:hover {
    background: var(--orb-color-transparent-bg-hover);
    color: var(--orb-color-text-secondary-brand-hover);
    cursor: pointer;
}
"#
}
