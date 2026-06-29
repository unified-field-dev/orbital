pub fn floating_actions_menu_styles() -> &'static str {
    r#".orbital-floating-actions-menu {
    position: relative;
    display: inline-flex;
    overflow: visible;
    z-index: 1900;
}

.orbital-floating-actions-menu--fixed {
    position: fixed;
}

.orbital-floating-actions-menu--anchored {
    position: absolute;
}

.orbital-floating-actions-menu--open {
    overflow: visible;
}

.orbital-floating-actions-menu__actions {
    display: flex;
    gap: 12px;
    align-items: center;
    overflow: visible;
    pointer-events: none;
}

.orbital-floating-actions-menu--open .orbital-floating-actions-menu__actions {
    pointer-events: auto;
}

.orbital-floating-actions-menu--direction-up .orbital-floating-actions-menu__actions {
    position: absolute;
    right: 0;
    bottom: calc(100% + 12px);
    flex-direction: column-reverse;
    align-items: flex-end;
}

.orbital-floating-actions-menu--direction-down .orbital-floating-actions-menu__actions {
    position: absolute;
    right: 0;
    top: calc(100% + 12px);
    flex-direction: column;
    align-items: flex-end;
}

.orbital-floating-actions-menu--direction-left .orbital-floating-actions-menu__actions {
    position: absolute;
    right: calc(100% + 12px);
    bottom: 0;
    flex-direction: row-reverse;
    align-items: center;
}

.orbital-floating-actions-menu--direction-right .orbital-floating-actions-menu__actions {
    position: absolute;
    left: calc(100% + 12px);
    bottom: 0;
    flex-direction: row;
    align-items: center;
}

.orbital-floating-actions-menu__item {
    display: flex;
    align-items: center;
    gap: 8px;
}

.orbital-floating-actions-menu--tooltip-left .orbital-floating-actions-menu__item {
    flex-direction: row;
}

.orbital-floating-actions-menu--tooltip-right .orbital-floating-actions-menu__item {
    flex-direction: row-reverse;
}

.orbital-floating-actions-menu__tooltip {
    padding: 4px 8px;
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    box-shadow: var(--orb-elev-raised-md);
    font-size: var(--orb-type-size-xs);
    white-space: nowrap;
}

.orbital-floating-actions-menu__action {
    width: 40px;
    height: 40px;
    border-radius: var(--orb-radius-floating);
    border: none;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    box-shadow: var(--orb-elev-raised-md);
}

.orbital-floating-actions-menu__action:hover {
    background-color: var(--orb-color-surface-canvas-hover);
}

.orbital-floating-actions-menu__trigger--open .orbital-floating-button {
    transform: rotate(45deg);
}
"#
}
