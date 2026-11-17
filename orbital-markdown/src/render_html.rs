use pulldown_cmark::{html, Options, Parser};

use crate::citations::{replace_citation_refs, strip_duplicate_images, CitationRef};
use crate::links::style_markdown_links;
use crate::mentions::{prepare_mention_markdown, replace_mention_links};
use crate::options::OrbitalMarkdownOptions;
use crate::sanitize::sanitize_html;

/// Context for resolving citation refs and deduping images.
#[derive(Clone, Debug, Default)]
pub struct RenderContext<'a> {
    pub citations: &'a [CitationRef<'a>],
    pub attachment_urls: &'a [&'a str],
}

/// Render markdown to sanitized HTML.
pub fn render_to_html(
    markdown: &str,
    options: &OrbitalMarkdownOptions,
    ctx: &RenderContext<'_>,
) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }

    let mut source = if options.enable_images {
        strip_duplicate_images(markdown, ctx.attachment_urls)
    } else {
        markdown.to_string()
    };

    if options.enable_mention_refs {
        source = prepare_mention_markdown(&source);
    }

    let mut parse_options = Options::empty();
    parse_options.insert(Options::ENABLE_STRIKETHROUGH);
    parse_options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&source, parse_options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut html = html_output;
    if options.enable_mention_refs {
        html = replace_mention_links(&html, options.mention_style);
    }

    html = sanitize_html(
        &html,
        options.enable_images,
        options.image_url_schemes.as_deref(),
    );

    html = add_block_classes(&html);

    if options.enable_images {
        html = html.replace(
            "<img src=",
            "<img class=\"orbital-markdown__image\" loading=\"lazy\" src=",
        );
    }

    if options.enable_citation_refs {
        html = replace_citation_refs(&html, ctx.citations, options.citation_style);
    }

    style_markdown_links(&html)
}

/// Class the block elements the sanitizer kept, and wrap each table so it
/// scrolls sideways on its own when it's wider than the page. The wrapper
/// takes focus so keyboard users can scroll it too.
///
/// Runs on sanitized output, where every tag is lowercase and only `th`/`td`
/// carry attributes, so plain string matches are exact.
fn add_block_classes(html: &str) -> String {
    html.replace(
        "<table>",
        "<div class=\"orbital-markdown__table-scroll\" tabindex=\"0\"><table class=\"orbital-markdown__table\">",
    )
    .replace("</table>", "</table></div>")
    .replace("<blockquote>", "<blockquote class=\"orbital-markdown__blockquote\">")
    .replace("<hr>", "<hr class=\"orbital-markdown__rule\">")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CitationLinkStyle;

    #[test]
    fn bold_renders() {
        let html = render_to_html(
            "Hello **bold**",
            &OrbitalMarkdownOptions::default(),
            &RenderContext::default(),
        );
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn citation_ref_in_body() {
        let citations = [CitationRef {
            id: "cit-1",
            display_index: 1,
        }];
        let ctx = RenderContext {
            citations: &citations,
            attachment_urls: &[],
        };
        let html = render_to_html(
            "See [^cit-1] here.",
            &OrbitalMarkdownOptions {
                enable_citation_refs: true,
                enable_mention_refs: false,
                enable_images: false,
                image_url_schemes: None,
                citation_style: CitationLinkStyle::discussion(),
                mention_style: Default::default(),
            },
            &ctx,
        );
        assert!(html.contains("discussion-citation-ref-cit-1"));
        assert!(html.contains("orbital-link orbital-link--inline"));
    }

    #[test]
    fn markdown_link_uses_orbital_link_classes() {
        let html = render_to_html(
            "See [design doc](https://example.com).",
            &OrbitalMarkdownOptions::default(),
            &RenderContext::default(),
        );
        assert!(html.contains("orbital-link orbital-link--inline"));
        assert!(html.contains("design doc"));
    }

    #[test]
    fn mention_ref_in_body() {
        let html = render_to_html(
            "Hi @[Jordan Lee](u1)!",
            &OrbitalMarkdownOptions {
                enable_citation_refs: false,
                enable_mention_refs: true,
                enable_images: false,
                image_url_schemes: None,
                citation_style: CitationLinkStyle::history(),
                mention_style: crate::MentionLinkStyle::history(),
            },
            &RenderContext::default(),
        );
        assert!(html.contains("data-mention-id=\"u1\""));
        assert!(html.contains("orbital-history__mention-ref"));
        assert!(html.contains("orbital-link orbital-link--inline"));
    }

    fn render_images(md: &str, schemes: Option<&[&str]>) -> String {
        render_to_html(
            md,
            &OrbitalMarkdownOptions {
                enable_images: true,
                image_url_schemes: schemes.map(|s| s.iter().map(|s| s.to_string()).collect()),
                ..Default::default()
            },
            &RenderContext::default(),
        )
    }

    #[test]
    fn image_schemes_none_keeps_default_set() {
        assert!(render_images("![a](https://x.test/a.png)", None).contains("<img"));
        assert!(render_images("![a](mailto:a@x.test)", None).contains("src=\"mailto:"));
    }

    #[test]
    fn image_schemes_https_only() {
        let s = Some(&["https"][..]);
        assert!(render_images("![a](https://x.test/a.png)", s).contains("src=\"https://"));
        assert!(render_images("![a](HTTPS://x.test/a.png)", s).contains("<img"));
        for url in [
            "http://x.test/a.png",
            "mailto:a@x.test",
            "data:image/png;base64,AA",
        ] {
            let html = render_images(&format!("![a]({url})"), s);
            assert!(!html.contains("src="), "{url} kept: {html}");
        }
    }

    #[test]
    fn image_schemes_keep_relative_src() {
        let s = Some(&["https"][..]);
        assert!(render_images("![a](/files/a.png)", s).contains("src=\"/files/a.png\""));
    }

    #[test]
    fn image_schemes_leave_links_alone() {
        let html = render_images("[l](http://x.test)", Some(&["https"][..]));
        assert!(html.contains("href=\"http://x.test\""));
    }

    #[test]
    fn image_schemes_cannot_widen() {
        let html = render_images("![a](ftp://x.test/a.png)", Some(&["ftp"][..]));
        assert!(!html.contains("src="));
    }

    fn render_plain(md: &str) -> String {
        render_to_html(md, &OrbitalMarkdownOptions::default(), &RenderContext::default())
    }

    const TABLE: &str = "| Feature | Status |\n|---|:---:|\n| [Rich content](/magnon/projects/1) | `Built` |\n| Charts | Planned |\n";

    #[test]
    fn gfm_table_renders_with_head_body_and_scroll_wrapper() {
        let html = render_plain(TABLE);
        assert!(html.contains(
            "<div class=\"orbital-markdown__table-scroll\" tabindex=\"0\"><table class=\"orbital-markdown__table\"><thead>"
        ), "{html}");
        assert!(html.contains("<tbody>"), "{html}");
        assert!(html.contains("</table></div>"), "{html}");
        assert!(html.contains("<th>Feature</th>"), "{html}");
    }

    #[test]
    fn gfm_table_cells_keep_links_code_and_alignment() {
        let html = render_plain(TABLE);
        assert!(html.contains("href=\"/magnon/projects/1\""), "{html}");
        assert!(html.contains("orbital-link orbital-link--inline"), "{html}");
        assert!(html.contains("<code>Built</code>"), "{html}");
        assert!(html.contains("<td style=\"text-align: center\">"), "{html}");
    }

    #[test]
    fn table_cell_style_other_than_alignment_is_dropped() {
        let html = render_plain(
            "<table><tr><td style=\"background:url(javascript:alert(1))\">x</td><td style=\"text-align: right; color: red\">y</td></tr></table>",
        );
        assert!(!html.contains("style="), "{html}");
        assert!(html.contains("<td>x</td>"), "{html}");
    }

    #[test]
    fn pipe_text_without_delimiter_row_stays_a_paragraph() {
        let html = render_plain("a | b\nc | d");
        assert!(!html.contains("<table"), "{html}");
        assert!(html.starts_with("<p>"), "{html}");
    }

    #[test]
    fn headings_blockquotes_and_rules_render() {
        let html = render_plain("## Vision\n\n> **Tagline**\n\n---\n\n#### Small");
        assert!(html.contains("<h2>Vision</h2>"), "{html}");
        assert!(html.contains(
            "<blockquote class=\"orbital-markdown__blockquote\">\n<p><strong>Tagline</strong></p>"
        ), "{html}");
        assert!(html.contains("<hr class=\"orbital-markdown__rule\">"), "{html}");
        assert!(html.contains("<h4>Small</h4>"), "{html}");
    }

    #[test]
    fn raw_html_block_tags_get_no_classes_or_handlers() {
        let html = render_plain(
            "<blockquote onclick=\"x()\">q</blockquote>\n\n<h2 id=\"spoof\" class=\"evil\">t</h2>",
        );
        assert!(!html.contains("onclick"), "{html}");
        assert!(!html.contains("spoof"), "{html}");
        assert!(!html.contains("evil"), "{html}");
    }
}
