/// Stylesheet for rendered Markdown blocks: tables, headings, blockquotes and
/// rules.
///
/// Table rules key on the classes [`render_to_html`](crate::render_to_html)
/// adds, so a table looks right inside any container. Heading, quote and rule
/// spacing applies under an `orbital-markdown` root, such as Orbital's
/// `MarkdownView`. Inject it once with `inject_style("orbital-markdown", markdown_styles())`.
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
