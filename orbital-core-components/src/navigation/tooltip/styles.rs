/// Tooltip surface, body, and arrow stylesheet.
pub fn tooltip_styles() -> &'static str {
    r#".orbital-tooltip-body {
    padding: 4px 11px 6px;
    line-height: var(--orb-type-line-sm);
    font-size: var(--orb-type-size-xs);
    font-family: var(--orb-type-family-sans);
    max-width: 240px;
    overflow-wrap: break-word;
    box-sizing: border-box;
}

.orbital-tooltip-content--normal.orbital-material--solid {
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
}

.orbital-tooltip-content--inverted.orbital-material--solid {
    background-color: var(--orb-color-surface-static);
    color: var(--orb-color-text-on-static);
}

.orbital-tooltip-shell > .orbital-popover-surface__angle {
    background-color: var(--orb-color-surface-canvas);
}

.orbital-tooltip-shell:has(.orbital-tooltip-content--inverted) > .orbital-popover-surface__angle {
    background-color: var(--orb-color-surface-static);
}

.orbital-tooltip-shell > .orbital-popover-surface__angle,
.orbital-popover-surface__angle {
    position: absolute;
    background-color: inherit;
    background-clip: content-box;
    box-sizing: border-box;
    z-index: 1;
    pointer-events: none;

    width: var(--orbital-positioning-arrow-height);
    height: var(--orbital-positioning-arrow-height);
    border-bottom-left-radius: var(--orb-radius-sm);
}

.orbital-tooltip-shell > .orbital-popover-surface__angle::before,
.orbital-popover-surface__angle::before {
    clip-path: polygon(0% 0%, 100% 100%, 0% 100%);
    border-bottom-left-radius: var(--orb-radius-sm);
    height: 100%;
    width: 100%;
    background-color: inherit;
    display: block;
    content: "";
    margin: -1px;
    border: 1px solid var(--orb-color-border-transparent);
}

[data-orbital-placement="top-start"] .orbital-popover-surface__angle,
[data-orbital-placement="top-end"] .orbital-popover-surface__angle,
[data-orbital-placement="top"] .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="top-start"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="top-end"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="top"] > .orbital-popover-surface__angle {
    transform: rotate(-45deg);
    bottom: var(--orbital-positioning-arrow-offset);
}

[data-orbital-placement="bottom-start"] .orbital-popover-surface__angle,
[data-orbital-placement="bottom-end"] .orbital-popover-surface__angle,
[data-orbital-placement="bottom"] .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="bottom-start"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="bottom-end"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="bottom"] > .orbital-popover-surface__angle {
    transform: rotate(135deg);
    top: var(--orbital-positioning-arrow-offset);
}

[data-orbital-placement="left-start"] .orbital-popover-surface__angle,
[data-orbital-placement="left-end"] .orbital-popover-surface__angle,
[data-orbital-placement="left"] .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="left-start"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="left-end"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="left"] > .orbital-popover-surface__angle {
    transform: rotate(225deg);
    right: var(--orbital-positioning-arrow-offset);
}

[data-orbital-placement="right-start"] .orbital-popover-surface__angle,
[data-orbital-placement="right-end"] .orbital-popover-surface__angle,
[data-orbital-placement="right"] .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="right-start"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="right-end"] > .orbital-popover-surface__angle,
.orbital-tooltip-shell[data-orbital-placement="right"] > .orbital-popover-surface__angle {
    transform: rotate(45deg);
    left: var(--orbital-positioning-arrow-offset);
}
"#
}
