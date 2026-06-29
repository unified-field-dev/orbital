//! Markdown renderer for motion preview doc tabs.

use leptos::prelude::*;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use turf::inline_style_sheet_values;

pub struct MarkdownClasses {
    pub paragraph: String,
    pub heading2: String,
    pub heading3: String,
    pub list: String,
    pub code_inline: String,
    pub code_block: String,
    pub link: String,
}

fn parse_events(source: &str) -> Vec<Event<'_>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    Parser::new_ext(source, options).collect()
}

#[component]
pub fn ComponentDocMarkdown(#[prop(into)] source: &'static str) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .DocMarkdown {
            font-size: var(--orb-type-size-sm);
            line-height: 1.6;
            color: var(--orb-color-text-primary);
        }
        .Paragraph {
            margin: 0 0 12px 0;
        }
        .Heading2 {
            margin: 16px 0 8px 0;
            font-weight: var(--orb-type-weight-semibold);
        }
        .Heading3 {
            margin: 12px 0 6px 0;
            font-weight: var(--orb-type-weight-semibold);
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
        .CodeBlock {
            font-family: var(--orb-type-family-mono);
            font-size: var(--orb-type-size-xs);
            background: var(--orb-color-surface-subtle);
            color: var(--orb-color-text-primary);
            padding: var(--orb-space-block-md) var(--orb-space-inline-md);
            border-radius: var(--orb-radius-md);
            overflow-x: auto;
            white-space: pre-wrap;
            margin: 0 0 12px 0;
        }
        .Link {
            color: var(--orb-color-brand-link);
            text-decoration: none;
        }
        .Link:hover {
            text-decoration: underline;
        }
    };

    let classes = MarkdownClasses {
        paragraph: class_names.paragraph.to_string(),
        heading2: class_names.heading_2.to_string(),
        heading3: class_names.heading_3.to_string(),
        list: class_names.list.to_string(),
        code_inline: class_names.code_inline.to_string(),
        code_block: class_names.code_block.to_string(),
        link: class_names.link.to_string(),
    };

    let nodes = render_markdown(source, &classes);

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.doc_markdown>{nodes}</div>
    }
}

fn render_markdown(source: &str, classes: &MarkdownClasses) -> Vec<AnyView> {
    let events = parse_events(source);
    render_events(&events, classes)
}

fn render_events(events: &[Event<'_>], classes: &MarkdownClasses) -> Vec<AnyView> {
    let mut views = Vec::new();
    let mut i = 0;
    while i < events.len() {
        if let Some(view) = render_block(events, &mut i, classes) {
            views.push(view);
        } else {
            i += 1;
        }
    }
    views
}

fn render_block(
    events: &[Event<'_>],
    idx: &mut usize,
    classes: &MarkdownClasses,
) -> Option<AnyView> {
    match &events[*idx] {
        Event::Start(Tag::Paragraph) => {
            *idx += 1;
            let inline = collect_inline(events, idx, TagEnd::Paragraph, classes);
            Some(view! { <p class=classes.paragraph.clone()>{inline}</p> }.into_any())
        }
        Event::Start(Tag::Heading { level, .. }) => {
            let level = *level;
            *idx += 1;
            let inline = collect_inline(events, idx, TagEnd::Heading(level), classes);
            let class = if level == pulldown_cmark::HeadingLevel::H2 {
                classes.heading2.clone()
            } else {
                classes.heading3.clone()
            };
            Some(view! { <div class=class>{inline}</div> }.into_any())
        }
        Event::Start(Tag::List(_)) => {
            *idx += 1;
            let items = collect_list_items(events, idx, classes);
            Some(view! { <ul class=classes.list.clone()>{items}</ul> }.into_any())
        }
        Event::Start(Tag::CodeBlock(_)) => {
            *idx += 1;
            let code = if let Event::Text(text) = &events[*idx] {
                text.to_string()
            } else {
                String::new()
            };
            *idx += 2;
            Some(
                view! { <pre class=classes.code_block.clone()><code>{code}</code></pre> }
                    .into_any(),
            )
        }
        Event::Text(text) if !text.trim().is_empty() => {
            let text = text.to_string();
            *idx += 1;
            Some(view! { <p class=classes.paragraph.clone()>{text}</p> }.into_any())
        }
        _ => None,
    }
}

fn collect_inline(
    events: &[Event<'_>],
    idx: &mut usize,
    end: TagEnd,
    classes: &MarkdownClasses,
) -> Vec<AnyView> {
    let mut parts = Vec::new();
    while *idx < events.len() {
        match &events[*idx] {
            Event::End(tag_end) if *tag_end == end => {
                *idx += 1;
                break;
            }
            Event::Text(text) => {
                parts.push(view! { {text.to_string()} }.into_any());
                *idx += 1;
            }
            Event::Code(text) => {
                parts.push(
                    view! { <code class=classes.code_inline.clone()>{text.to_string()}</code> }
                        .into_any(),
                );
                *idx += 1;
            }
            Event::Start(Tag::Strong) => {
                *idx += 1;
                let inner = collect_inline(events, idx, TagEnd::Strong, classes);
                parts.push(view! { <strong>{inner}</strong> }.into_any());
            }
            Event::Start(Tag::Emphasis) => {
                *idx += 1;
                let inner = collect_inline(events, idx, TagEnd::Emphasis, classes);
                parts.push(view! { <em>{inner}</em> }.into_any());
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                *idx += 1;
                let inner = collect_inline(events, idx, TagEnd::Link, classes);
                let url = dest_url.to_string();
                parts.push(
                    view! {
                        <a class=classes.link.clone() href=url>{inner}</a>
                    }
                    .into_any(),
                );
            }
            _ => {
                *idx += 1;
            }
        }
    }
    parts
}

fn collect_list_items(
    events: &[Event<'_>],
    idx: &mut usize,
    classes: &MarkdownClasses,
) -> Vec<AnyView> {
    let mut items = Vec::new();
    while *idx < events.len() {
        match &events[*idx] {
            Event::End(TagEnd::List(_)) => {
                *idx += 1;
                break;
            }
            Event::Start(Tag::Item) => {
                *idx += 1;
                let inline = collect_inline(events, idx, TagEnd::Item, classes);
                items.push(view! { <li>{inline}</li> }.into_any());
            }
            _ => {
                *idx += 1;
            }
        }
    }
    items
}
