pub fn presence_badge_styles() -> &'static str {
    r#"
.orbital-presence-badge {
    position: relative;
    display: inline-flex;
    vertical-align: middle;
    overflow: visible;
}

.orbital-presence-badge:not(.orbital-presence-badge--standalone) .orbital-avatar {
    position: relative;
    z-index: 0;
}

.orbital-presence-badge__indicator {
    position: absolute;
    bottom: 0;
    right: 0;
    box-sizing: border-box;
    border-radius: var(--orb-radius-circular);
    border: var(--orb-stroke-thick) solid var(--orb-color-surface-canvas);
    background-color: var(--orb-color-text-quaternary);
    z-index: 1;
}

.orbital-presence-badge__indicator--available {
    background-color: var(--orb-color-status-success-fg);
}

.orbital-presence-badge__indicator--away {
    background-color: var(--orb-color-status-warning-fg);
}

.orbital-presence-badge__indicator--busy {
    background-color: var(--orb-color-status-danger-fg);
}

.orbital-presence-badge__indicator--offline {
    background-color: var(--orb-color-text-quaternary);
}

.orbital-presence-badge__indicator--out-of-office {
    background-color: var(--orb-color-status-warning-fg);
    border-color: var(--orb-color-surface-canvas);
    box-shadow: inset 0 0 0 1px var(--orb-color-border-subtle);
}

.orbital-presence-badge__indicator--unknown {
    background-color: transparent;
    border-color: var(--orb-color-text-quaternary);
}

.orbital-presence-badge--extra-small .orbital-presence-badge__indicator {
    width: 6px;
    height: 6px;
}

.orbital-presence-badge--small .orbital-presence-badge__indicator {
    width: 8px;
    height: 8px;
}

.orbital-presence-badge--medium .orbital-presence-badge__indicator {
    width: 10px;
    height: 10px;
}

.orbital-presence-badge--large .orbital-presence-badge__indicator {
    width: 12px;
    height: 12px;
}

.orbital-presence-badge--extra-large .orbital-presence-badge__indicator {
    width: 14px;
    height: 14px;
}

.orbital-presence-badge--standalone .orbital-presence-badge__indicator {
    position: static;
}
"#
}
