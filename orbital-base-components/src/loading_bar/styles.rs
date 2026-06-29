/// Loading bar stylesheet.
pub fn loading_bar_styles() -> &'static str {
    r#".orbital-loading-bar-container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
}

.orbital-loading-bar {
    height: 2px;
    max-width: 0;
    background-color: var(--orb-color-brand-bg);
}
"#
}
