/// Message bar stylesheet.
pub fn message_bar_styles() -> &'static str {
    r#".orbital-message-bar {
    white-space: normal;
    display: grid;
    grid-template: "icon body secondaryActions actions" 1fr / auto 1fr auto auto;
    padding-left: var(--orb-space-inline-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    align-items: center;
    min-height: 36px;
    min-width: 0;
    max-width: 100%;
    box-sizing: border-box;
    background-color: var(--orb-color-surface-subtle);
    color: var(--orb-color-text-primary);
}

.orbital-message-bar.orbital-message-bar--multiline {
    grid-template-areas:
        "icon title actions"
        "icon body actions"
        "secondaryActions secondaryActions secondaryActions";
    grid-template-columns: auto 1fr auto;
    grid-template-rows: auto auto auto;
    padding-block: var(--orb-space-block-mnudge);
    align-items: start;
    white-space: normal;
}

.orbital-message-bar--multiline:not(:has(.orbital-message-bar-actions)) {
    grid-template-areas:
        "icon title actions"
        "icon body actions";
    grid-template-rows: auto auto;
}

.orbital-message-bar--multiline:not(:has(.orbital-message-bar-title)) {
    grid-template-areas:
        "icon body actions"
        "secondaryActions secondaryActions secondaryActions";
    grid-template-rows: auto auto;
}

.orbital-message-bar--multiline:not(:has(.orbital-message-bar-title)):not(
        :has(.orbital-message-bar-actions)
    ) {
    grid-template-areas: "icon body actions";
    grid-template-rows: auto;
}

.orbital-message-bar--singleline:has(.orbital-message-bar-title):has(.orbital-message-bar-body) {
    grid-template-areas: "icon title body secondaryActions actions";
    grid-template-columns: auto auto 1fr auto auto;
}

.orbital-message-bar--singleline:has(.orbital-message-bar-title):has(.orbital-message-bar-body)
    > .orbital-message-bar-title {
    grid-area: title;
    margin-right: var(--orb-space-inline-sm);
}

.orbital-message-bar--singleline:has(.orbital-message-bar-title):has(.orbital-message-bar-body)
    > .orbital-message-bar-title::after {
    content: none;
}

.orbital-message-bar--singleline:has(.orbital-message-bar-title):has(.orbital-message-bar-body)
    > .orbital-message-bar-body {
    grid-area: body;
}

.orbital-message-bar--multiline > .orbital-message-bar-title::after {
    content: none;
}

.orbital-message-bar--multiline > .orbital-message-bar__icon {
    align-self: center;
}

.orbital-message-bar--success {
    border-color: var(--orb-color-status-success-border);
    background-color: var(--orb-color-status-success-bg);
}

.orbital-message-bar--success > .orbital-message-bar__icon {
    color: var(--orb-color-status-success-fg);
}

.orbital-message-bar--warning {
    border-color: var(--orb-color-status-warning-border);
    background-color: var(--orb-color-status-warning-bg);
}

.orbital-message-bar--warning > .orbital-message-bar__icon {
    color: var(--orb-color-status-warning-fg);
}

.orbital-message-bar--error {
    border-color: var(--orb-color-status-danger-border);
    background-color: var(--orb-color-status-danger-bg);
}

.orbital-message-bar--error > .orbital-message-bar__icon {
    color: var(--orb-color-status-danger-fg);
}

.orbital-message-bar__icon {
    grid-area: icon;
    font-size: var(--orb-type-size-lg);
    margin-right: var(--orb-space-inline-sm);
    color: var(--orb-color-text-tertiary);
    display: flex;
    align-items: center;
}

.orbital-message-bar__icon > svg {
    display: inline;
    line-height: 0;
}

.orbital-message-bar-body {
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-regular);
    line-height: var(--orb-type-line-md);
    grid-area: body;
    min-width: 0;
    white-space: normal;
    overflow-wrap: break-word;
    padding-right: var(--orb-space-inline-md);
}

.orbital-message-bar-title {
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-md);
    grid-area: body;
    min-width: 0;
    overflow-wrap: break-word;
    white-space: normal;
}

.orbital-message-bar-title::after {
    content: " ";
}

.orbital-message-bar--multiline > .orbital-message-bar-title {
    grid-area: title;
}

.orbital-message-bar--multiline > .orbital-message-bar-body {
    grid-area: body;
}

.orbital-message-bar-actions {
    grid-area: secondaryActions;
    display: flex;
    column-gap: var(--orb-space-inline-md);
    padding-right: var(--orb-space-inline-md);
}

.orbital-message-bar-actions__container-action {
    grid-area: actions;
    padding-right: var(--orb-space-inline-md);
}

.orbital-message-bar--multiline > .orbital-message-bar-actions {
    padding-right: var(--orb-space-block-md);
    margin-bottom: var(--orb-space-block-sm);
    margin-top: var(--orb-space-block-mnudge);
    justify-content: end;
    margin-right: 0;
}
"#
}
