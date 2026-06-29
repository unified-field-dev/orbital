/// Shared stylesheet for floating overlay panels (Menu, Popover, Tooltip).
pub fn floating_panel_styles() -> &'static str {
    r#".orbital-floating-panel.orbital-material {
    --orbital-material-width: max-content;
    overflow: visible;
    border: 1px solid var(--orb-color-border-transparent);
}

.orbital-menu,
.orbital-popover-shell,
.orbital-tooltip-shell {
    width: fit-content;
    transform-origin: inherit;
}

.orbital-tooltip-shell {
    position: relative;
}

[data-orbital-placement="top-start"] > .orbital-positioning-content,
[data-orbital-placement="top-end"] > .orbital-positioning-content,
[data-orbital-placement="top"] > .orbital-positioning-content {
    margin-bottom: 4px;
}

[data-orbital-placement="bottom-start"] > .orbital-positioning-content,
[data-orbital-placement="bottom-end"] > .orbital-positioning-content,
[data-orbital-placement="bottom"] > .orbital-positioning-content {
    margin-top: 4px;
}

[data-orbital-placement="left-start"] > .orbital-positioning-content,
[data-orbital-placement="left-end"] > .orbital-positioning-content,
[data-orbital-placement="left"] > .orbital-positioning-content {
    margin-right: 4px;
}

[data-orbital-placement="right-start"] > .orbital-positioning-content,
[data-orbital-placement="right-end"] > .orbital-positioning-content,
[data-orbital-placement="right"] > .orbital-positioning-content {
    margin-left: 4px;
}

[data-orbital-placement="top-start"] > .orbital-menu,
[data-orbital-placement="top-end"] > .orbital-menu,
[data-orbital-placement="top"] > .orbital-menu,
.orbital-menu[data-orbital-placement="top-start"],
.orbital-menu[data-orbital-placement="top-end"],
.orbital-menu[data-orbital-placement="top"],
[data-orbital-placement="top-start"] > .orbital-popover-shell,
[data-orbital-placement="top-end"] > .orbital-popover-shell,
[data-orbital-placement="top"] > .orbital-popover-shell,
.orbital-popover-shell[data-orbital-placement="top-start"],
.orbital-popover-shell[data-orbital-placement="top-end"],
.orbital-popover-shell[data-orbital-placement="top"],
[data-orbital-placement="top-start"] > .orbital-tooltip-shell,
[data-orbital-placement="top-end"] > .orbital-tooltip-shell,
[data-orbital-placement="top"] > .orbital-tooltip-shell,
.orbital-tooltip-shell[data-orbital-placement="top-start"],
.orbital-tooltip-shell[data-orbital-placement="top-end"],
.orbital-tooltip-shell[data-orbital-placement="top"] {
    margin-bottom: 4px;
}

[data-orbital-placement="bottom-start"] > .orbital-menu,
[data-orbital-placement="bottom-end"] > .orbital-menu,
[data-orbital-placement="bottom"] > .orbital-menu,
.orbital-menu[data-orbital-placement="bottom-start"],
.orbital-menu[data-orbital-placement="bottom-end"],
.orbital-menu[data-orbital-placement="bottom"],
[data-orbital-placement="bottom-start"] > .orbital-popover-shell,
[data-orbital-placement="bottom-end"] > .orbital-popover-shell,
[data-orbital-placement="bottom"] > .orbital-popover-shell,
.orbital-popover-shell[data-orbital-placement="bottom-start"],
.orbital-popover-shell[data-orbital-placement="bottom-end"],
.orbital-popover-shell[data-orbital-placement="bottom"],
[data-orbital-placement="bottom-start"] > .orbital-tooltip-shell,
[data-orbital-placement="bottom-end"] > .orbital-tooltip-shell,
[data-orbital-placement="bottom"] > .orbital-tooltip-shell,
.orbital-tooltip-shell[data-orbital-placement="bottom-start"],
.orbital-tooltip-shell[data-orbital-placement="bottom-end"],
.orbital-tooltip-shell[data-orbital-placement="bottom"] {
    margin-top: 4px;
}

[data-orbital-placement="left-start"] > .orbital-menu,
[data-orbital-placement="left-end"] > .orbital-menu,
[data-orbital-placement="left"] > .orbital-menu,
.orbital-menu[data-orbital-placement="left-start"],
.orbital-menu[data-orbital-placement="left-end"],
.orbital-menu[data-orbital-placement="left"],
[data-orbital-placement="left-start"] > .orbital-popover-shell,
[data-orbital-placement="left-end"] > .orbital-popover-shell,
[data-orbital-placement="left"] > .orbital-popover-shell,
.orbital-popover-shell[data-orbital-placement="left-start"],
.orbital-popover-shell[data-orbital-placement="left-end"],
.orbital-popover-shell[data-orbital-placement="left"],
[data-orbital-placement="left-start"] > .orbital-tooltip-shell,
[data-orbital-placement="left-end"] > .orbital-tooltip-shell,
[data-orbital-placement="left"] > .orbital-tooltip-shell,
.orbital-tooltip-shell[data-orbital-placement="left-start"],
.orbital-tooltip-shell[data-orbital-placement="left-end"],
.orbital-tooltip-shell[data-orbital-placement="left"] {
    margin-right: 4px;
}

[data-orbital-placement="right-start"] > .orbital-menu,
[data-orbital-placement="right-end"] > .orbital-menu,
[data-orbital-placement="right"] > .orbital-menu,
.orbital-menu[data-orbital-placement="right-start"],
.orbital-menu[data-orbital-placement="right-end"],
.orbital-menu[data-orbital-placement="right"],
[data-orbital-placement="right-start"] > .orbital-popover-shell,
[data-orbital-placement="right-end"] > .orbital-popover-shell,
[data-orbital-placement="right"] > .orbital-popover-shell,
.orbital-popover-shell[data-orbital-placement="right-start"],
.orbital-popover-shell[data-orbital-placement="right-end"],
.orbital-popover-shell[data-orbital-placement="right"],
[data-orbital-placement="right-start"] > .orbital-tooltip-shell,
[data-orbital-placement="right-end"] > .orbital-tooltip-shell,
[data-orbital-placement="right"] > .orbital-tooltip-shell,
.orbital-tooltip-shell[data-orbital-placement="right-start"],
.orbital-tooltip-shell[data-orbital-placement="right-end"],
.orbital-tooltip-shell[data-orbital-placement="right"] {
    margin-left: 4px;
}
"#
}
