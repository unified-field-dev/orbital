pub fn menu_button_styles() -> &'static str {
    r#"
.orbital-menu-button__trigger {
    gap: var(--orb-space-inline-xs);
}

.orbital-menu-button__chevron {
    display: inline-flex;
    align-items: center;
    margin-inline-start: var(--orb-space-inline-2xs);
    line-height: 0;
}
"#
}
