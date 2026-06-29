use leptos::prelude::*;

#[cfg(feature = "hydrate")]
const COMPOSER_TEXTAREA_SELECTOR: &str = "[data-testid=\"discussion-composer-input\"] textarea";

/// Wrap or insert markdown markers using textarea selection when available.
pub fn apply_markdown_wrap(
    draft: RwSignal<String>,
    _composer: crate::ComposerContext,
    prefix: &str,
    suffix: &str,
) {
    let prefix = prefix.to_string();
    let suffix = suffix.to_string();

    #[cfg(feature = "hydrate")]
    {
        if let Some(textarea) = composer_textarea_element() {
            let body = draft.get_untracked();
            let start = textarea.selection_start().ok().flatten().unwrap_or(0);
            let end = textarea.selection_end().ok().flatten().unwrap_or(start);
            let (new_body, sel_start, sel_end) = compute_wrap(&body, start, end, &prefix, &suffix);
            draft.set(new_body.clone());
            restore_textarea_selection(textarea, &new_body, sel_start, sel_end);
            return;
        }
    }

    let body = draft.get_untracked();
    let end_utf16 = utf16_len(&body);
    let (new_body, _, _) = compute_wrap(&body, end_utf16, end_utf16, &prefix, &suffix);
    draft.set(new_body);
}

/// Insert text at the textarea caret (or append when selection is unavailable).
pub fn insert_at_caret(draft: RwSignal<String>, _composer: crate::ComposerContext, insert: &str) {
    let insert = insert.to_string();

    #[cfg(feature = "hydrate")]
    {
        if let Some(textarea) = composer_textarea_element() {
            let body = draft.get_untracked();
            let start = textarea.selection_start().ok().flatten().unwrap_or(0);
            let end = textarea.selection_end().ok().flatten().unwrap_or(start);
            let (new_body, sel_start, sel_end) = insert_range(&body, start, end, &insert);
            draft.set(new_body.clone());
            restore_textarea_selection(textarea, &new_body, sel_start, sel_end);
            return;
        }
    }

    insert_markdown_suffix(draft, &insert);
}

#[cfg(feature = "hydrate")]
fn composer_textarea_element() -> Option<web_sys::HtmlTextAreaElement> {
    use wasm_bindgen::JsCast;

    let doc = web_sys::window()?.document()?;
    let node = doc.query_selector(COMPOSER_TEXTAREA_SELECTOR).ok()??;
    node.dyn_into::<web_sys::HtmlTextAreaElement>().ok()
}

/// Insert text at the end of the draft (append).
pub fn insert_markdown_suffix(draft: RwSignal<String>, suffix: &str) {
    draft.update(|body| {
        *body = if body.is_empty() {
            suffix.to_string()
        } else if body.ends_with('\n') || body.ends_with(' ') {
            format!("{body}{suffix}")
        } else {
            format!("{body} {suffix}")
        };
    });
}

/// Insert text at the end of the draft.
pub fn insert_markdown_prefix(draft: RwSignal<String>, prefix: &str) {
    insert_markdown_suffix(draft, prefix);
}

/// Insert a markdown image reference after upload.
pub fn insert_markdown_image(draft: RwSignal<String>, alt: &str, url: &str) {
    insert_markdown_suffix(draft, &format!("![{alt}]({url})"));
}

/// Insert a markdown link.
pub fn insert_markdown_link(draft: RwSignal<String>, label: &str, url: &str) {
    insert_markdown_suffix(draft, &format!("[{label}]({url})"));
}

/// Insert a citation reference token.
pub fn insert_citation_ref(draft: RwSignal<String>, citation_id: &str) {
    insert_markdown_suffix(draft, &format!("[^{citation_id}]"));
}

/// Wrap the entire draft with prefix/suffix (legacy fallback).
pub fn wrap_markdown_selection(draft: RwSignal<String>, prefix: &str, suffix: &str) {
    draft.update(|body| *body = format!("{prefix}{body}{suffix}"));
}

/// Compute wrapped markdown for UTF-16 textarea selection indices.
pub fn compute_wrap(
    body: &str,
    start_utf16: u32,
    end_utf16: u32,
    prefix: &str,
    suffix: &str,
) -> (String, u32, u32) {
    let (start_utf16, end_utf16) = if start_utf16 <= end_utf16 {
        (start_utf16, end_utf16)
    } else {
        (end_utf16, start_utf16)
    };

    let start_char = utf16_offset_to_char_idx(body, start_utf16);
    let end_char = utf16_offset_to_char_idx(body, end_utf16);

    if start_char >= end_char {
        let before: String = body.chars().take(start_char).collect();
        let after: String = body.chars().skip(start_char).collect();
        let new_body = format!("{before}{prefix}{suffix}{after}");
        let cursor = start_utf16 + utf16_len(prefix);
        (new_body, cursor, cursor)
    } else {
        let before: String = body.chars().take(start_char).collect();
        let selected: String = body
            .chars()
            .skip(start_char)
            .take(end_char.saturating_sub(start_char))
            .collect();
        let after: String = body.chars().skip(end_char).collect();
        let new_body = format!("{before}{prefix}{selected}{suffix}{after}");
        let inner_start = start_utf16 + utf16_len(prefix);
        let inner_end = inner_start + utf16_len(&selected);
        (new_body, inner_start, inner_end)
    }
}

#[cfg(feature = "hydrate")]
fn insert_range(body: &str, start_utf16: u32, end_utf16: u32, insert: &str) -> (String, u32, u32) {
    let (start_utf16, end_utf16) = if start_utf16 <= end_utf16 {
        (start_utf16, end_utf16)
    } else {
        (end_utf16, start_utf16)
    };

    let start_char = utf16_offset_to_char_idx(body, start_utf16);
    let end_char = utf16_offset_to_char_idx(body, end_utf16);
    let before: String = body.chars().take(start_char).collect();
    let after: String = body.chars().skip(end_char).collect();
    let new_body = format!("{before}{insert}{after}");
    let cursor = start_utf16 + utf16_len(insert);
    (new_body, cursor, cursor)
}

fn utf16_offset_to_char_idx(body: &str, utf16_offset: u32) -> usize {
    let mut utf16_count = 0u32;
    for (char_idx, ch) in body.chars().enumerate() {
        if utf16_count >= utf16_offset {
            return char_idx;
        }
        utf16_count += ch.len_utf16() as u32;
    }
    body.chars().count()
}

fn utf16_len(text: &str) -> u32 {
    text.chars().map(|ch| ch.len_utf16() as u32).sum()
}

#[cfg(feature = "hydrate")]
fn restore_textarea_selection(
    textarea: web_sys::HtmlTextAreaElement,
    value: &str,
    sel_start: u32,
    sel_end: u32,
) {
    let value = value.to_string();
    request_animation_frame(move || {
        textarea.set_value(&value);
        let _ = textarea.focus();
        let _ = textarea.set_selection_start(Some(sel_start));
        let _ = textarea.set_selection_end(Some(sel_end));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::Owner;

    fn with_owner<F: FnOnce()>(f: F) {
        Owner::new().with(f);
    }

    #[test]
    fn insert_citation_ref_appends() {
        with_owner(|| {
            let draft = RwSignal::new("See ".to_string());
            insert_citation_ref(draft, "cit-1");
            assert_eq!(draft.get_untracked(), "See [^cit-1]");
        });
    }

    #[test]
    fn insert_image_markdown() {
        with_owner(|| {
            let draft = RwSignal::new(String::new());
            insert_markdown_image(draft, "chart", "https://example.com/a.png");
            assert_eq!(draft.get_untracked(), "![chart](https://example.com/a.png)");
        });
    }

    #[test]
    fn wraps_selected_range() {
        let (body, _, _) = compute_wrap("hello world", 0, 5, "**", "**");
        assert_eq!(body, "**hello** world");
    }

    #[test]
    fn inserts_markers_at_collapsed_caret() {
        let (body, _, _) = compute_wrap("hello", 5, 5, "**", "**");
        assert_eq!(body, "hello****");
    }

    #[cfg(feature = "hydrate")]
    #[test]
    fn insert_range_replaces_selection() {
        let (body, _, _) = insert_range("See here", 4, 8, "[^cit-1]");
        assert_eq!(body, "See [^cit-1]");
    }
}
