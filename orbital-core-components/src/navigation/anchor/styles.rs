pub fn anchor_styles() -> &'static str {
    r#".orbital-anchor {
    position: relative;
    padding-left: 4px;
}

.orbital-anchor .orbital-anchor-link + .orbital-anchor-link,
.orbital-anchor .orbital-anchor-link > .orbital-anchor-link {
    margin-top: 0.5em;
}

.orbital-anchor-rail {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 4px;
    border-radius: 2px;
    overflow: hidden;
    transition: background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    background-color: var(--orb-color-border-subtle);
}

.orbital-anchor-rail__bar {
    position: absolute;
    left: 0;
    width: 4px;
    height: 21px;
    transition: top 0.15s cubic-bezier(0.4, 0, 0.2, 1),
        background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.orbital-anchor-rail__bar--active {
    background-color: var(--orb-color-brand-bg);
}

.orbital-anchor-link {
    padding: 0 0 0 16px;
    position: relative;
    line-height: var(--orb-type-line-sm);
    font-size: var(--orb-type-size-xs);
    min-height: 1.5em;
    display: flex;
    flex-direction: column;
}

.orbital-anchor-link--active > .orbital-anchor-link__title {
    color: var(--orb-color-text-primary);
}

.orbital-anchor-link__title {
    max-width: 100%;
    text-decoration: none;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    cursor: pointer;
    display: inline-block;
    padding-right: 16px;
    transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    color: var(--orb-color-text-secondary);
}

.orbital-anchor-link__title:hover {
    color: var(--orb-color-text-secondary-hover);
}
"#
}
