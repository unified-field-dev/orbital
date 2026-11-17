/// Stylesheet for rendered Markdown: tables, code, images, task lists, alerts,
/// quotes, rules, headings and `<details>`.
///
/// Rules on the classes [`render_to_html`](crate::render_to_html) adds (tables,
/// code blocks, images, alerts) work inside any container. Heading, inline
/// code, task-list and `<details>` rules apply under an `orbital-markdown`
/// root, such as Orbital's `MarkdownView`. Inject it once with
/// `inject_style("orbital-markdown", markdown_styles())`.
#[must_use]
pub const fn markdown_styles() -> &'static str {
    r#"
.orbital-markdown {
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-md);
    overflow-wrap: anywhere;
    min-width: 0;
}

.orbital-markdown > :first-child {
    margin-top: 0;
}

.orbital-markdown > :last-child {
    margin-bottom: 0;
}

.orbital-markdown :is(h1, h2, h3, h4, h5, h6) {
    margin: var(--orb-space-block-lg) 0 var(--orb-space-block-xs);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
}

.orbital-markdown h1 {
    font-size: var(--orb-type-size-2xl);
    line-height: var(--orb-type-line-xl);
}

.orbital-markdown h2 {
    font-size: var(--orb-type-size-xl);
    line-height: var(--orb-type-line-xl);
}

.orbital-markdown h3 {
    font-size: var(--orb-type-size-lg);
    line-height: var(--orb-type-line-lg);
}

.orbital-markdown :is(h4, h5, h6) {
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-md);
}

.orbital-markdown__blockquote {
    margin: var(--orb-space-block-sm) 0;
    padding: var(--orb-space-block-2xs) var(--orb-space-inline-md);
    border-left: var(--orb-stroke-thicker) solid var(--orb-color-border-default);
    color: var(--orb-color-text-secondary);
}

.orbital-markdown__blockquote > :first-child {
    margin-top: 0;
}

.orbital-markdown__blockquote > :last-child {
    margin-bottom: 0;
}

.orbital-markdown__rule {
    margin: var(--orb-space-block-lg) 0;
    border: none;
    border-top: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
}

.orbital-markdown :not(pre) > code {
    padding: 0 var(--orb-space-inline-2xs);
    border-radius: var(--orb-radius-sm);
    background-color: var(--orb-color-code-bg);
    color: var(--orb-color-code-fg);
    font-family: var(--orb-type-family-mono);
    font-size: 0.9em;
}

.orbital-markdown__code-block {
    margin: var(--orb-space-block-sm) 0;
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
    overflow-x: auto;
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-code-bg);
    color: var(--orb-color-code-fg);
    font-family: var(--orb-type-family-mono);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    overflow-wrap: normal;
    white-space: pre;
}

.orbital-markdown__code-block code {
    font: inherit;
    color: inherit;
}

.orbital-markdown__image {
    display: block;
    max-width: 100%;
    height: auto;
    margin: var(--orb-space-block-xs) 0;
    border-radius: var(--orb-radius-sm);
}

.orbital-markdown li:has(> .orbital-markdown__task),
.orbital-markdown li:has(> p > .orbital-markdown__task) {
    list-style: none;
}

.orbital-markdown__task {
    margin: 0 var(--orb-space-inline-xs) 0 calc(-1 * var(--orb-space-inline-lg));
    vertical-align: middle;
    accent-color: var(--orb-color-brand-bg);
}

.orbital-markdown__alert {
    --orbital-markdown-alert-accent: var(--orb-color-brand-stroke);
    --orbital-markdown-alert-title: var(--orb-color-brand-fg);
    margin: var(--orb-space-block-sm) 0;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-md);
    border-left: var(--orb-stroke-thicker) solid var(--orbital-markdown-alert-accent);
    border-radius: var(--orb-radius-sm);
    background-color: var(--orb-color-surface-subtle);
}

.orbital-markdown__alert > :last-child {
    margin-bottom: 0;
}

.orbital-markdown__alert-title {
    margin: 0 0 var(--orb-space-block-2xs);
    color: var(--orbital-markdown-alert-title);
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-markdown__alert--tip {
    --orbital-markdown-alert-accent: var(--orb-color-status-success-border);
    --orbital-markdown-alert-title: var(--orb-color-status-success-fg);
}

.orbital-markdown__alert--warning {
    --orbital-markdown-alert-accent: var(--orb-color-status-warning-border);
    --orbital-markdown-alert-title: var(--orb-color-status-warning-fg);
}

.orbital-markdown__alert--caution {
    --orbital-markdown-alert-accent: var(--orb-color-status-danger-border);
    --orbital-markdown-alert-title: var(--orb-color-status-danger-fg);
}

.orbital-markdown details {
    margin: var(--orb-space-block-sm) 0;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
}

.orbital-markdown summary {
    cursor: pointer;
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-markdown details[open] > summary {
    margin-bottom: var(--orb-space-block-xs);
}

.orbital-markdown__table-scroll {
    max-width: 100%;
    margin: var(--orb-space-block-sm) 0;
    overflow-x: auto;
    border-radius: var(--orb-radius-sm);
}

.orbital-markdown__table-scroll:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-border-focus);
    outline-offset: 2px;
}

.orbital-markdown__table {
    border-collapse: collapse;
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-sm);
    overflow-wrap: normal;
}

.orbital-markdown__table :is(th, td) {
    padding: var(--orb-space-block-xs) var(--orb-space-inline-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
    text-align: left;
    vertical-align: top;
}

.orbital-markdown__table th {
    background-color: var(--orb-color-surface-subtle);
    font-weight: var(--orb-type-weight-semibold);
}
"#
}
