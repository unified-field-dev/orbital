pub fn avatar_group_styles() -> &'static str {
    r#"
.orbital-avatar-group {
    display: inline-flex;
    align-items: center;
    vertical-align: middle;
}

.orbital-avatar-group--spread {
    gap: 4px;
}

.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow) {
    position: relative;
    margin-inline-start: calc(var(--orbital-avatar-group-size, 32px) * -0.35);
    box-shadow: 0 0 0 2px var(--orb-color-surface-canvas);
}

.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):first-child {
    margin-inline-start: 0;
    z-index: 1;
}

.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(2) { z-index: 2; }
.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(3) { z-index: 3; }
.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(4) { z-index: 4; }
.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(5) { z-index: 5; }
.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(6) { z-index: 6; }
.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(7) { z-index: 7; }
.orbital-avatar-group--stack > :not(.orbital-avatar-group__overflow):nth-child(8) { z-index: 8; }

.orbital-avatar-group__overflow {
    position: relative;
    z-index: 20;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--orbital-avatar-group-size, 32px);
    height: var(--orbital-avatar-group-size, 32px);
    border-radius: var(--orb-radius-circular);
    background-color: var(--orb-color-surface-subtle);
    color: var(--orb-color-text-primary);
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-semibold);
    margin-inline-start: calc(var(--orbital-avatar-group-size, 32px) * -0.35);
    box-shadow: 0 0 0 2px var(--orb-color-surface-canvas);
    flex-shrink: 0;
}
"#
}
