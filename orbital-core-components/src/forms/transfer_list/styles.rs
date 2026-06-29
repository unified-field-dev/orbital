pub fn transfer_list_styles() -> &'static str {
    r#".orbital-transfer-list {
    display: flex;
    gap: 16px;
    align-items: center;
    width: 100%;
    max-width: 720px;
}

.orbital-transfer-list__panel {
    flex: 1 1 0;
    min-width: 0;
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-surface-canvas);
    display: flex;
    flex-direction: column;
    min-height: 220px;
}

.orbital-transfer-list__title {
    padding: var(--orb-space-block-md) var(--orb-space-inline-lg) var(--orb-space-block-sm);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
    border-bottom: 1px solid var(--orb-color-border-subtle);
}

.orbital-transfer-list__header {
    padding: var(--orb-space-block-sm) var(--orb-space-inline-lg);
    border-bottom: 1px solid var(--orb-color-border-subtle);
}

.orbital-transfer-list__select-all .orbital-checkbox__label {
    color: var(--orb-color-text-secondary);
}

.orbital-transfer-list__list {
    list-style: none;
    margin: 0;
    padding: var(--orb-space-block-sm) 0;
    overflow: auto;
    flex: 1 1 auto;
}

.orbital-transfer-list__item {
    margin: 0;
    padding: 0 var(--orb-space-inline-sm);
}

.orbital-transfer-list__controls {
    display: flex;
    flex-direction: column;
    gap: 8px;
}
"#
}
