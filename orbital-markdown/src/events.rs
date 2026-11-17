//! Parser-event pass: task-list boxes and GitHub alerts become placeholder
//! characters that [`expand_placeholders`] swaps for fixed markup after
//! sanitizing, and bare URLs in text become links.
//!
//! The placeholders are private-use characters that [`strip_placeholders`]
//! removes from the source first, so only the parser can produce them. Raw
//! HTML that imitates the output (an `<input>` or a `markdown-alert-*` class)
//! is still sanitized away.

use linkify::{LinkFinder, LinkKind};
use pulldown_cmark::{BlockQuoteKind, CowStr, Event, LinkType, Tag, TagEnd};

const TASK_OPEN: char = '\u{E000}';
const TASK_DONE: char = '\u{E001}';
/// Alert kinds take `ALERT_BASE + index` in [`ALERTS`].
const ALERT_BASE: u32 = 0xE010;

/// Alert kinds in placeholder order: class suffix and visible title.
const ALERTS: [(&str, &str); 5] = [
    ("note", "Note"),
    ("tip", "Tip"),
    ("important", "Important"),
    ("warning", "Warning"),
    ("caution", "Caution"),
];

const fn alert_index(kind: BlockQuoteKind) -> u32 {
    match kind {
        BlockQuoteKind::Note => 0,
        BlockQuoteKind::Tip => 1,
        BlockQuoteKind::Important => 2,
        BlockQuoteKind::Warning => 3,
        BlockQuoteKind::Caution => 4,
    }
}

fn alert_placeholder(index: u32) -> char {
    char::from_u32(ALERT_BASE + index).unwrap_or(TASK_OPEN)
}

/// Remove every placeholder character from the Markdown source.
pub(crate) fn strip_placeholders(source: &str) -> String {
    source
        .chars()
        .filter(|c| !matches!(u32::from(*c), 0xE000..=0xE01F))
        .collect()
}

/// Rewrite parser events: task markers and alert kinds to placeholders, and
/// bare `http(s)://` URLs outside links, images and code blocks to links.
pub(crate) fn transform<'a>(events: impl Iterator<Item = Event<'a>>) -> Vec<Event<'a>> {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let mut out = Vec::new();
    // Inside a link, image or code block, where text must stay as written.
    let mut literal_depth = 0usize;
    for event in events {
        match event {
            Event::TaskListMarker(done) => {
                let mark = if done { TASK_DONE } else { TASK_OPEN };
                out.push(Event::Text(mark.to_string().into()));
            }
            Event::Start(Tag::BlockQuote(Some(kind))) => {
                out.push(Event::Start(Tag::BlockQuote(None)));
                out.push(Event::Text(
                    alert_placeholder(alert_index(kind)).to_string().into(),
                ));
            }
            Event::End(TagEnd::BlockQuote(Some(_))) => {
                out.push(Event::End(TagEnd::BlockQuote(None)));
            }
            Event::Start(Tag::Link { .. } | Tag::Image { .. } | Tag::CodeBlock(_)) => {
                literal_depth += 1;
                out.push(event);
            }
            Event::End(TagEnd::Link | TagEnd::Image | TagEnd::CodeBlock) => {
                literal_depth = literal_depth.saturating_sub(1);
                out.push(event);
            }
            Event::Text(text) if literal_depth == 0 => linkify_text(&finder, text, &mut out),
            other => out.push(other),
        }
    }
    out
}

fn linkify_text<'a>(finder: &LinkFinder, text: CowStr<'a>, out: &mut Vec<Event<'a>>) {
    let mut links = finder.links(&text).peekable();
    if links.peek().is_none() {
        out.push(Event::Text(text));
        return;
    }
    let mut last = 0;
    for link in links {
        let url = link.as_str();
        let scheme_ok = url.starts_with("https://") || url.starts_with("http://");
        if !scheme_ok {
            continue;
        }
        if link.start() > last {
            out.push(Event::Text(text[last..link.start()].to_string().into()));
        }
        out.push(Event::Start(Tag::Link {
            link_type: LinkType::Autolink,
            dest_url: url.to_string().into(),
            title: CowStr::Borrowed(""),
            id: CowStr::Borrowed(""),
        }));
        out.push(Event::Text(url.to_string().into()));
        out.push(Event::End(TagEnd::Link));
        last = link.end();
    }
    if last < text.len() {
        out.push(Event::Text(text[last..].to_string().into()));
    }
}

/// Swap placeholders in sanitized HTML for task boxes and alert markup.
///
/// Runs before [`add_block_classes`](crate::render_html), so an alert's
/// `<blockquote>` is still bare here.
pub(crate) fn expand_placeholders(html: &str) -> String {
    let mut html = html
        .replace(
            TASK_OPEN,
            "<input type=\"checkbox\" class=\"orbital-markdown__task\" disabled aria-label=\"Task\">",
        )
        .replace(
            TASK_DONE,
            "<input type=\"checkbox\" class=\"orbital-markdown__task\" disabled checked aria-label=\"Task\">",
        );
    for (index, (kind, title)) in (0u32..).zip(ALERTS) {
        let mark = alert_placeholder(index);
        let opened = format!(
            "<blockquote class=\"orbital-markdown__alert orbital-markdown__alert--{kind}\">\n<p class=\"orbital-markdown__alert-title\">{title}</p>"
        );
        html = html.replace(&format!("<blockquote>\n{mark}"), &opened);
        // A placeholder that lost its blockquote (it can't, but never leak it).
        html = html.replace(mark, "");
    }
    html
}
