pub fn tag_styles() -> &'static str {
    r#"
.orbital-tag {
    display: inline-grid;
    align-items: center;
    grid-template-areas:
        "media primary dismissIcon"
        "media secondary dismissIcon";
    width: fit-content;
    height: 32px;
    padding: 0 7px;
    background-color: var(--orb-color-surface-subtle);
    color: var(--orb-color-text-secondary);
    font-family: inherit;
    appearance: button;
    text-align: unset;
    box-sizing: border-box;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-transparent);
    border-radius: var(--orb-radius-md);
}

.orbital-tag--filled {
    background-color: var(--orb-color-surface-subtle);
    color: var(--orb-color-text-secondary);
    border-color: var(--orb-color-border-transparent);
}

.orbital-tag--outline {
    background-color: transparent;
    color: var(--orb-color-text-secondary);
    border-color: var(--orb-color-border-default);
}

.orbital-tag--brand {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
    border-color: var(--orb-color-border-transparent);
}

.orbital-tag--small {
    height: 24px;
    padding: 0 5px;
}

.orbital-tag--extra-small {
    height: 20px;
    padding: 0 5px;
}

.orbital-tag--dismissible {
    padding-right: 0;
}

.orbital-tag--with-media {
    padding-left: 0;
}

.orbital-tag__media {
    grid-area: media;
    display: flex;
    align-items: center;
    justify-content: center;
    padding-left: 7px;
    padding-right: var(--orb-space-inline-2xs);
    line-height: 0;
}

.orbital-tag--small .orbital-tag__media {
    padding-left: 5px;
    font-size: 16px;
}

.orbital-tag--extra-small .orbital-tag__media {
    padding-left: 5px;
    font-size: 12px;
}

.orbital-tag__primary-text {
    grid-row-end: secondary;
    grid-row-start: primary;
    grid-column-start: primary;
    white-space: nowrap;
    padding: 0 var(--orb-space-inline-2xs) var(--orb-space-inline-2xs);
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: inherit;
}

.orbital-tag--small .orbital-tag__primary-text,
.orbital-tag--extra-small .orbital-tag__primary-text {
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
}

.orbital-tag__dismiss {
    grid-area: dismissIcon;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    padding-right: 7px;
    padding-left: var(--orb-space-inline-xs);
    font-size: 20px;
    background-color: transparent;
    border: none;
    cursor: pointer;
    color: inherit;
}

.orbital-tag--small .orbital-tag__dismiss {
    padding-left: var(--orb-space-inline-2xs);
    font-size: 16px;
    padding-right: 5px;
}

.orbital-tag--extra-small .orbital-tag__dismiss {
    padding-left: var(--orb-space-inline-2xs);
    font-size: 12px;
    padding-right: 5px;
}

.orbital-tag__dismiss > svg {
    display: inline;
    line-height: 0;
}

.orbital-tag__dismiss:hover {
    color: var(--orb-color-brand-compound-fg-hover);
}

.orbital-tag__dismiss:active {
    color: var(--orb-color-brand-compound-fg-pressed);
}

.orbital-tag--brand .orbital-tag__dismiss:hover {
    color: var(--orb-color-text-on-brand);
    opacity: 0.85;
}

.orbital-tag--brand .orbital-tag__dismiss:active {
    opacity: 0.7;
}

.orbital-interaction-tag {
    display: inline-flex;
    align-items: stretch;
    box-sizing: border-box;
    width: fit-content;
    height: 32px;
}

.orbital-interaction-tag--small {
    height: 24px;
}

.orbital-interaction-tag--extra-small {
    height: 20px;
}

button.orbital-tag.orbital-tag--primary-action {
    cursor: pointer;
    height: 100%;
}

.orbital-tag--primary-action.orbital-tag--with-secondary {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
    border-right: none;
}

button.orbital-tag.orbital-tag--primary-action:hover {
    color: var(--orb-color-text-secondary-hover);
    background-color: var(--orb-color-surface-subtle-hover);
}

button.orbital-tag.orbital-tag--primary-action.orbital-tag--outline:hover {
    background-color: var(--orb-color-surface-subtle-hover);
}

button.orbital-tag.orbital-tag--primary-action.orbital-tag--brand:hover {
    background-color: var(--orb-color-brand-bg-hover);
    color: var(--orb-color-text-on-brand);
}

button.orbital-tag.orbital-tag--primary-action:active {
    color: var(--orb-color-text-secondary-pressed);
    background-color: var(--orb-color-surface-subtle-pressed);
}

button.orbital-tag.orbital-tag--primary-action.orbital-tag--brand:active {
    background-color: var(--orb-color-brand-bg-pressed);
}

button.orbital-tag__dismiss.orbital-tag--secondary-action {
    grid-area: unset;
    height: 100%;
    border-top-right-radius: var(--orb-radius-md);
    border-bottom-right-radius: var(--orb-radius-md);
    border-left: none;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-transparent);
}

button.orbital-tag__dismiss.orbital-tag--secondary-action.orbital-tag--filled {
    background-color: var(--orb-color-surface-subtle);
    color: var(--orb-color-text-secondary);
    border-color: var(--orb-color-border-transparent);
}

button.orbital-tag__dismiss.orbital-tag--secondary-action.orbital-tag--outline {
    background-color: transparent;
    color: var(--orb-color-text-secondary);
    border-color: var(--orb-color-border-default);
    border-left: none;
}

button.orbital-tag__dismiss.orbital-tag--secondary-action.orbital-tag--brand {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
    border-color: var(--orb-color-border-transparent);
}

button.orbital-tag__dismiss.orbital-tag--secondary-action.orbital-tag--brand:hover {
    color: var(--orb-color-text-on-brand);
    opacity: 0.85;
}

button.orbital-tag__dismiss.orbital-tag--secondary-action.orbital-tag--brand:active {
    opacity: 0.7;
}
"#
}

pub fn tag_group_styles() -> &'static str {
    r#"
.orbital-tag-group {
    display: inline-flex;
    column-gap: var(--orb-space-inline-sm);
}
"#
}
