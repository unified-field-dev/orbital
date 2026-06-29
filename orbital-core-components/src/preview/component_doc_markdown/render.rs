use leptos::prelude::*;
use pulldown_cmark::{Event, Tag, TagEnd};

use crate::{Code, Link};

use super::parse_events;
use super::MarkdownClasses;

pub fn render_markdown(source: &str, classes: &MarkdownClasses) -> Vec<AnyView> {
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
            Some(view! { <Code text=code /> }.into_any())
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
                let inline = url.starts_with('/') || url.starts_with('#');
                parts.push(
                    view! {
                        <Link href=url inline=inline>
                            {inner}
                        </Link>
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
