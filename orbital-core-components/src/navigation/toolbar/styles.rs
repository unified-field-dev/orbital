pub fn toolbar_styles() -> &'static str {
    r#"
.orbital-toolbar {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-xs);
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    background: var(--orb-color-surface-canvas);
    border-radius: var(--orb-radius-md);
    min-height: 40px;
}

.orbital-toolbar--horizontal {
    flex-direction: row;
}

.orbital-toolbar--vertical {
    flex-direction: column;
    align-items: stretch;
    min-height: auto;
    width: fit-content;
}

.orbital-toolbar--small {
    min-height: 32px;
    gap: var(--orb-space-inline-2xs);
}

.orbital-toolbar--large {
    min-height: 48px;
    gap: var(--orb-space-inline-sm);
}
"#
}
