/// Stable-class scrollbar chrome for [`ScrollArea`](super::ScrollArea).
///
/// Chrome 121+ ignores `::-webkit-scrollbar` when `scrollbar-width` or `scrollbar-color` are non-initial, so standard properties are gated behind `@supports not selector(::-webkit-scrollbar)` (Firefox-only).
pub fn scroll_area_styles() -> &'static str {
    r#"
.orbital-scroll-area {
    display: block;
    min-height: 0;
    min-width: 0;
    box-sizing: border-box;
}

.orbital-scroll-area::-webkit-scrollbar {
    width: var(--orbital-scrollbar-size, 8px);
    height: var(--orbital-scrollbar-size, 8px);
    background: transparent;
}

.orbital-scroll-area::-webkit-scrollbar-thumb {
    background-color: var(--orb-color-text-tertiary);
    border-radius: var(--orb-radius-circular);
}

.orbital-scroll-area::-webkit-scrollbar-thumb:hover {
    background-color: var(--orb-color-text-secondary);
}

.orbital-scroll-area::-webkit-scrollbar-track {
    background: transparent;
}

.orbital-scroll-area--hide-chrome {
    scrollbar-width: none;
}

.orbital-scroll-area--hide-chrome::-webkit-scrollbar {
    display: none;
}

@supports not selector(::-webkit-scrollbar) {
    .orbital-scroll-area {
        scrollbar-width: thin;
        scrollbar-color: var(--orb-color-text-tertiary) transparent;
    }
}
"#
}
