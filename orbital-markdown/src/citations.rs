use regex::Regex;

/// Citation id + 1-based display index for ref resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CitationRef<'a> {
    pub id: &'a str,
    pub display_index: usize,
}

/// Replace `[^id]` in HTML output with superscript anchor links.
pub fn replace_citation_refs(html: &str, citations: &[CitationRef<'_>]) -> String {
    if citations.is_empty() {
        return html.to_string();
    }

    let mut result = html.to_string();
    for citation in citations {
        let pattern = format!(r"\[\^{}\]", regex::escape(citation.id));
        let Ok(re) = Regex::new(&pattern) else {
            continue;
        };
        let replacement = format!(
            r##"<sup class="orbital-markdown__citation-ref"><a href="#discussion-citation-row-{0}" id="discussion-citation-ref-{0}" data-citation-id="{0}">{1}</a></sup>"##,
            citation.id, citation.display_index,
        );
        result = re.replace_all(&result, replacement.as_str()).to_string();
    }
    result
}

/// Strip markdown image syntax for URLs already rendered as File parts.
pub fn strip_duplicate_images(markdown: &str, attachment_urls: &[&str]) -> String {
    if attachment_urls.is_empty() {
        return markdown.to_string();
    }

    let mut result = markdown.to_string();
    for url in attachment_urls {
        let escaped = regex::escape(url);
        let pattern = format!(r"!\[[^\]]*\]\({escaped}\)");
        if let Ok(re) = Regex::new(&pattern) {
            result = re.replace_all(&result, "").to_string();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_citation_ref_in_html() {
        let html = "<p>See [^cit-1] for details.</p>";
        let citations = vec![CitationRef {
            id: "cit-1",
            display_index: 1,
        }];
        let out = replace_citation_refs(html, &citations);
        assert!(out.contains("discussion-citation-row-cit-1"));
        assert!(out.contains(">1</a>"));
        assert!(!out.contains("[^cit-1]"));
    }
}
