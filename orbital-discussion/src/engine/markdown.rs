use orbital_markdown::{
    render_to_html, CitationRef as MarkdownCitationRef, OrbitalMarkdownOptions, RenderContext,
};

use crate::DiscussionCitation;

/// Render markdown for a discussion reply body part.
pub fn render_markdown(
    markdown: &str,
    citations: &[DiscussionCitation],
    attachment_urls: &[&str],
) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }

    let citation_refs: Vec<MarkdownCitationRef<'_>> = citations
        .iter()
        .enumerate()
        .map(|(i, c)| MarkdownCitationRef {
            id: c.id.as_str(),
            display_index: i + 1,
        })
        .collect();

    render_to_html(
        markdown,
        &OrbitalMarkdownOptions::discussion_body(),
        &RenderContext {
            citations: &citation_refs,
            attachment_urls,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiscussionCitation;

    #[test]
    fn bold_and_citation_ref() {
        let citations = vec![DiscussionCitation::new("cit-1", "Doc")];
        let html = render_markdown("See [^cit-1] here.", &citations, &[]);
        assert!(html.contains("discussion-citation-ref-cit-1"));
    }
}
