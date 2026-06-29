pub fn progress_bar_styles() -> &'static str {
    r#"
.orbital-progress-bar {
    display: block;
    width: 100%;
    /* 4px track height — slightly taller for catalog/preview legibility */
    height: 4px;
    background-color: var(--orb-color-surface-sunken);
    overflow: hidden;
    border-radius: var(--orb-radius-md);
}

.orbital-progress-bar__bar {
    transition-timing-function: ease;
    transition-duration: 0.3s;
    transition-property: width;
    height: 100%;
    background-color: var(--orb-color-brand-compound-bg);
    border-radius: inherit;
}

.orbital-progress-bar--error .orbital-progress-bar__bar {
    background-color: var(--orb-color-palette-red-bg);
}

.orbital-progress-bar--warning .orbital-progress-bar__bar {
    background-color: var(--orb-color-palette-orange-bg);
}

.orbital-progress-bar--success .orbital-progress-bar__bar {
    background-color: var(--orb-color-palette-green-bg);
}
"#
}

pub fn progress_circle_styles() -> &'static str {
    r#"
.orbital-progress-circle {
    width: var(--orbital-progress-circle-size);
    height: var(--orbital-progress-circle-size);
    display: inline-block;
    position: relative;
}

.orbital-progress-circle svg {
    width: 100%;
    height: 100%;
    display: block;
    overflow: visible;
}

.orbital-progress-circle__fill {
    transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1),
        stroke 0.3s cubic-bezier(0.4, 0, 0.2, 1),
        stroke-dasharray 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.orbital-progress-circle__fill--empty {
    opacity: 0;
}

.orbital-progress-circle__content {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translateX(-50%) translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
}

.orbital-progress-circle__content--text {
    font-size: calc(var(--orbital-progress-circle-size) * 0.23);
    text-align: center;
    white-space: nowrap;
}
"#
}
