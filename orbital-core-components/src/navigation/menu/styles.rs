/// Menu shell + surface/body stylesheet.
pub fn menu_styles() -> &'static str {
    r#".orbital-menu-body {
    display: flex;
    flex-direction: column;
    row-gap: var(--orb-space-inline-2xs);
    max-height: 80vh;
    padding: var(--orb-space-inline-xs);
    overflow-y: auto;
}

.orbital-menu--brand.orbital-material--solid {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-menu--inverted.orbital-material--solid {
    background-color: var(--orb-color-surface-static);
    color: var(--orb-color-text-on-static);
}
"#
}

/// Menu item stylesheet.
pub fn menu_item_styles() -> &'static str {
    r#".orbital-menu-item {
    display: flex;
    align-items: center;
    padding: var(--orb-space-block-snudge) var(--orb-space-inline-sm);
    color: inherit;
    cursor: pointer;
    border-radius: var(--orb-radius-md);
}

.orbital-menu-item:hover:not(.orbital-menu-item--disabled) {
    background-color: var(--orb-color-surface-canvas-hover);
    color: var(--orb-color-text-primary-hover);
}

.orbital-menu-item:active:not(.orbital-menu-item--disabled) {
    background-color: var(--orb-color-surface-canvas-pressed);
    color: var(--orb-color-text-primary-pressed);
}

.orbital-menu--brand .orbital-menu-item:hover:not(.orbital-menu-item--disabled) {
    background-color: var(--orb-color-brand-bg-hover);
    color: var(--orb-color-text-on-brand);
}

.orbital-menu--brand .orbital-menu-item:active:not(.orbital-menu-item--disabled) {
    background-color: var(--orb-color-brand-bg-pressed);
    color: var(--orb-color-text-on-brand);
}

.orbital-menu--inverted .orbital-menu-item:hover:not(.orbital-menu-item--disabled) {
    background-color: var(--orb-color-surface-static-hover);
    color: var(--orb-color-text-on-static);
}

.orbital-menu--inverted .orbital-menu-item:active:not(.orbital-menu-item--disabled) {
    background-color: var(--orb-color-surface-static-pressed);
    color: var(--orb-color-text-on-static);
}

.orbital-menu-item.orbital-menu-item--disabled {
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}
"#
}
