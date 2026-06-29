pub fn spotlight_styles() -> &'static str {
    r#"
.orbital-spotlight {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-md);
    max-width: 320px;
}

.orbital-spotlight__header {
    font-size: var(--orb-type-size-md);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
}

.orbital-spotlight__body {
    font-size: var(--orb-type-size-sm);
    color: var(--orb-color-text-secondary);
    line-height: var(--orb-type-line-md);
}

.orbital-spotlight__media {
    border-radius: var(--orb-radius-md);
    overflow: hidden;
}

.orbital-spotlight__actions {
    display: flex;
    gap: var(--orb-space-inline-sm);
    justify-content: flex-end;
}

.orbital-spotlight__footer {
    font-size: var(--orb-type-size-xs);
    color: var(--orb-color-text-tertiary);
}

.orbital-spotlight-portal {
    position: relative;
    z-index: 1000;
}

.orbital-spotlight-portal__backdrop {
    z-index: 0;
}

.orbital-spotlight-portal__surface {
    z-index: 1;
}

/* Portaled tips/tours must read as elevated surfaces over preview cards (also bg1). */
.orbital-popover-shell.orbital-spotlight .orbital-popover-surface.orbital-material--solid:not(.orbital-popover-surface--brand):not(.orbital-popover-surface--inverted) {
    background-color: var(--orb-color-surface-static);
    border-color: var(--orb-color-border-subtle);
}
"#
}
