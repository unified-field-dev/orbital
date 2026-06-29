pub fn auto_complete_styles() -> &'static str {
    r#"
.orbital-auto-complete {
    display: inline-flex;
}

.orbital-auto-complete > .orbital-input {
    min-width: 250px;
}

.orbital-auto-complete__listbox.orbital-listbox {
    width: 100%;
    --orbital-auto-complete-option-block-size: calc(
        var(--orb-type-line-md) + 2 * var(--orb-space-block-snudge)
    );
    --orbital-auto-complete-listbox-chrome: calc(2 * var(--orb-space-inline-xs));
    --orbital-auto-complete-listbox-row-gap: var(--orb-space-inline-2xs);
    --orbital-auto-complete-listbox-max-rows: 10;
    max-height: 320px;
    max-height: calc(
        var(--orbital-auto-complete-listbox-chrome)
        + var(--orbital-auto-complete-listbox-max-rows) * var(--orbital-auto-complete-option-block-size)
        + (var(--orbital-auto-complete-listbox-max-rows) - 1) * var(--orbital-auto-complete-listbox-row-gap)
    );
    overflow-x: hidden;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--orb-color-text-tertiary) transparent;
}

.orbital-auto-complete__listbox.orbital-listbox::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

.orbital-auto-complete__listbox.orbital-listbox::-webkit-scrollbar-thumb {
    background-color: var(--orb-color-text-tertiary);
    border-radius: var(--orb-radius-circular);
}

.orbital-auto-complete__listbox.orbital-listbox::-webkit-scrollbar-thumb:hover {
    background-color: var(--orb-color-text-secondary);
}

.orbital-auto-complete__listbox.orbital-listbox::-webkit-scrollbar-track {
    background: transparent;
}

.orbital-auto-complete__listbox > div {
    display: flex;
    flex-direction: column;
    row-gap: var(--orb-space-inline-2xs);
}

.orbital-listbox {
    row-gap: var(--orb-space-inline-2xs);
    display: flex;
    flex-direction: column;
    min-width: 160px;
    background-color: var(--orb-color-surface-canvas);
    padding: var(--orb-space-inline-xs);
    outline: 1px solid var(--orb-color-border-transparent);
    border-radius: var(--orb-radius-md);
    box-shadow: var(--orb-elev-floating);
    box-sizing: border-box;
    overflow-y: auto;
}

.orbital-auto-complete-option {
    column-gap: var(--orb-space-inline-xs);
    position: relative;
    display: flex;
    align-items: center;
    padding: var(--orb-space-block-snudge) var(--orb-space-inline-sm);
    line-height: var(--orb-type-line-md);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
    cursor: pointer;
    flex-shrink: 0;
}

.orbital-auto-complete-option[data-activedescendant-focusvisible]::after {
    content: "";
    position: absolute;
    right: -2px;
    left: -2px;
    bottom: -2px;
    top: -2px;
    z-index: 1;
    pointer-events: none;
    border-radius: var(--orb-radius-md);
    border: 2px solid var(--orb-color-border-focus);
}

.orbital-auto-complete-option:hover {
    color: var(--orb-color-text-primary-hover);
    background-color: var(--orb-color-surface-canvas-hover);
}

.orbital-auto-complete-option:active {
    color: var(--orb-color-text-primary-pressed);
    background-color: var(--orb-color-surface-canvas-pressed);
}

.orbital-auto-complete-option--hidden {
    display: none;
}
"#
}
