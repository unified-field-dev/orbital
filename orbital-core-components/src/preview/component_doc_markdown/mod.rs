use leptos::prelude::*;
use pulldown_cmark::{Options, Parser};
use turf::inline_style_sheet_values;

mod render;

pub struct MarkdownClasses {
    pub paragraph: String,
    pub heading2: String,
    pub heading3: String,
    pub list: String,
    pub code_inline: String,
}

#[component]
pub fn ComponentDocMarkdown(
    /// Raw markdown body (section content without the top-level `#` title).
    source: &'static str,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        // Must not use `.Root` — turf hashes collide with `Text`'s `.Root` and override Title1 sizing.
        .DocMarkdown {
            font-size: 14px;
            line-height: 1.6;
            color: var(--orb-color-text-primary);
        }
        .Paragraph {
            margin: 0 0 12px 0;
        }
        .Heading2 {
            margin: 16px 0 8px 0;
            font-weight: 600;
        }
        .Heading3 {
            margin: 12px 0 6px 0;
            font-weight: 600;
        }
        .List {
            margin: 0 0 12px 20px;
            padding: 0;
        }
        .CodeInline {
            font-family: var(--orb-type-family-mono);
            background: var(--orb-color-code-bg);
            color: var(--orb-color-code-fg);
            padding: 1px 4px;
            border-radius: var(--orb-radius-sm);
        }
    };

    let classes = MarkdownClasses {
        paragraph: class_names.paragraph.to_string(),
        heading2: class_names.heading_2.to_string(),
        heading3: class_names.heading_3.to_string(),
        list: class_names.list.to_string(),
        code_inline: class_names.code_inline.to_string(),
    };

    let nodes = render::render_markdown(source, &classes);

    view! {
        <div class=class_names.doc_markdown>
            <style>{style_sheet}</style>
            {nodes}
        </div>
    }
}

pub fn parse_events(source: &str) -> Vec<pulldown_cmark::Event<'_>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    Parser::new_ext(source, options).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m01_bold_italic_preserved_in_parse() {
        let events = parse_events("**bold** and *italic*");
        let text: String = events
            .iter()
            .filter_map(|e| match e {
                pulldown_cmark::Event::Text(t) => Some(t.to_string()),
                _ => None,
            })
            .collect();
        assert!(text.contains("bold"));
        assert!(text.contains("italic"));
    }

    #[test]
    fn m03_fenced_code_in_parse() {
        let events = parse_events("```rust\nlet x = 1;\n```");
        assert!(events.iter().any(|e| matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(_))
        )));
    }

    #[test]
    fn m05_ordered_list_in_parse() {
        let events = parse_events("1. First\n2. Second");
        assert!(events.iter().any(|e| matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::List(Some(1)))
        )));
    }

    #[test]
    fn m06_h2_subheading_in_parse() {
        let events = parse_events("## Do's\n\n* item");
        assert!(events.iter().any(|e| matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { .. })
        )));
    }
}
