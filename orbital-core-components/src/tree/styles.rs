/// Tree + motion styles ported from vendor tree CSS with `orbital-` prefixes.
pub fn tree_styles() -> &'static str {
    r#"
.orbital-tree {
    display: flex;
    flex-direction: column;
    row-gap: var(--orb-space-block-2xs);
    min-width: 280px;
}
.orbital-subtree {
    padding-top: var(--orb-space-block-2xs);
}
.orbital-tree-item {
    position: relative;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    color: var(--orb-color-text-secondary);
    padding-right: var(--orb-space-inline-none);
}
.orbital-tree-item--disabled {
    opacity: 0.5;
    pointer-events: none;
}
.orbital-tree-item-layout {
    display: flex;
    flex-direction: row;
    align-items: center;
    min-height: 32px;
    box-sizing: border-box;
    padding-left: calc(
        (var(--orbital-tree-item--level, 1) - 1) * var(--orb-space-inline-2xl)
    );
    padding-right: var(--orb-space-inline-xs);
    font-family: var(--orb-type-family-sans);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    cursor: pointer;
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-subtle-bg);
}
.orbital-tree-item--leaf .orbital-tree-item-layout {
    padding-left: calc(
        var(--orbital-tree-item--level, 1) * var(--orb-space-inline-2xl)
    );
}
.orbital-tree--small .orbital-tree-item-layout {
    min-height: 24px;
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
}
.orbital-tree-item-layout:hover {
    color: var(--orb-color-text-secondary-hover);
    background-color: var(--orb-color-subtle-bg-hover);
}
.orbital-tree-item-layout:active,
.orbital-tree-item-layout--selected,
.orbital-tree-item--selected .orbital-tree-item-layout {
    color: var(--orb-color-text-secondary-pressed);
    background-color: var(--orb-color-subtle-bg-selected);
}
.orbital-tree-item--focused > .orbital-tree-item-layout,
.orbital-tree-item-layout:focus-within {
    outline: 2px solid var(--orb-color-brand-stroke);
    outline-offset: -2px;
}
.orbital-tree-item-layout__expand-icon {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 24px;
    box-sizing: border-box;
    color: var(--orb-color-text-tertiary);
    padding: var(--orb-space-block-xs) 0;
}
.orbital-tree-item-layout:hover .orbital-tree-item-layout__expand-icon {
    color: var(--orb-color-text-tertiary-hover);
}
.orbital-tree-item-layout__icon-before,
.orbital-tree-item-layout__icon-after {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    padding: 0 var(--orb-space-inline-2xs);
}
.orbital-tree-item-layout__main {
    flex: 1 1 auto;
    min-width: 0;
    padding: 0 var(--orb-space-inline-2xs);
}
.orbital-tree-item-layout__label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.orbital-tree-item-layout__aside {
    flex: 0 0 auto;
    opacity: 0;
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-2xs);
    transition: opacity var(--orb-motion-duration-sm) var(--orb-motion-ease-standard);
}
.orbital-tree-item-layout:hover .orbital-tree-item-layout__aside,
.orbital-tree-item-layout:focus-within .orbital-tree-item-layout__aside {
    opacity: 1;
}
.orbital-tree-item-layout__label-input {
    width: 100%;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    padding: 2px 6px;
    font: inherit;
    background: var(--orb-color-surface-canvas);
}
.orbital-tree-item-layout__checkbox {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    padding-right: var(--orb-space-inline-2xs);
    cursor: pointer;
}
.orbital-tree-item-layout__drag-handle {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    padding-right: var(--orb-space-inline-2xs);
    color: var(--orb-color-text-tertiary);
    cursor: grab;
    user-select: none;
    font-size: 10px;
    line-height: 1;
}
.orbital-tree-item-layout__checkbox-box {
    width: 16px;
    height: 16px;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-canvas);
    box-sizing: border-box;
}
.orbital-tree-item-layout__checkbox-box--checked {
    background: var(--orb-color-brand-bg);
    border-color: var(--orb-color-brand-bg);
}
.orbital-tree-item-layout__checkbox-box--indeterminate {
    background: var(--orb-color-brand-bg);
    border-color: var(--orb-color-brand-bg);
    position: relative;
}
.orbital-tree-item-layout__checkbox-box--indeterminate::after {
    content: "";
    position: absolute;
    top: 50%;
    left: 3px;
    right: 3px;
    height: 2px;
    background: var(--orb-color-text-on-brand);
    transform: translateY(-50%);
}
.orbital-tree--connectors .orbital-subtree {
    margin-left: calc(var(--orb-space-inline-2xl) / 2);
    border-left: 1px solid var(--orb-color-border-subtle);
    padding-left: calc(var(--orb-space-inline-2xl) / 2);
}
.orbital-tree-item--dragging {
    opacity: 0.5;
}
.orbital-tree-item--drop-before::before,
.orbital-tree-item--drop-after::after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    height: 2px;
    background: var(--orb-color-brand-stroke);
}
.orbital-tree-item--drop-before::before {
    top: 0;
}
.orbital-tree-item--drop-after::after {
    bottom: 0;
}
.orbital-tree-virtual-scroll {
    width: 100%;
}
.orbital-tree-custom-collapse-enter-from,
.orbital-tree-custom-collapse-leave-to {
    max-height: 0;
    opacity: 0;
}
.orbital-tree-custom-collapse-enter-to,
.orbital-tree-custom-collapse-leave-from {
    opacity: 1;
}
.orbital-tree-custom-collapse-enter-active,
.orbital-tree-custom-collapse-leave-active {
    overflow: hidden;
    transition: max-height var(--orb-motion-duration-xl) var(--orb-motion-ease-standard),
        opacity var(--orb-motion-duration-xl) var(--orb-motion-ease-standard);
}
"#
}
