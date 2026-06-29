/// Feature flags for markdown rendering.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OrbitalMarkdownOptions {
    /// Resolve `[^id]` citation reference syntax to superscript links.
    pub enable_citation_refs: bool,
    /// Render `![alt](url)` as `<img>` (deduped against attachment URLs in context).
    pub enable_images: bool,
}

impl OrbitalMarkdownOptions {
    /// Discussion reply body defaults.
    pub fn discussion_body() -> Self {
        Self {
            enable_citation_refs: true,
            enable_images: true,
        }
    }
}
