pub fn action_menu_button_styles() -> &'static str {
    r#"
.orbital-action-menu-button {
    display: inline-flex;
}

.orbital-action-menu-button__primary {
    border-top-left-radius: var(--orb-radius-md) !important;
    border-bottom-left-radius: var(--orb-radius-md) !important;
    border-top-right-radius: 0 !important;
    border-bottom-right-radius: 0 !important;
}

.orbital-action-menu-button .orbital-action-menu-button__menu {
    border-top-left-radius: 0 !important;
    border-bottom-left-radius: 0 !important;
    border-top-right-radius: var(--orb-radius-md) !important;
    border-bottom-right-radius: var(--orb-radius-md) !important;
    min-width: 32px;
    padding-inline: var(--orb-space-inline-snudge) !important;
}

.orbital-action-menu-button .orbital-action-menu-button__menu.orbital-button--circular {
    border-top-right-radius: var(--orb-radius-circular) !important;
    border-bottom-right-radius: var(--orb-radius-circular) !important;
}

.orbital-action-menu-button .orbital-action-menu-button__menu.orbital-button--square {
    border-top-right-radius: var(--orb-radius-none) !important;
    border-bottom-right-radius: var(--orb-radius-none) !important;
}
"#
}
