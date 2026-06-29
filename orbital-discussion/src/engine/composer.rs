use leptos::prelude::*;

use crate::{DiscussionAttachmentDraft, DiscussionAttachmentValidation, DiscussionCitation};

/// Whether the composer can submit the current draft.
pub fn can_submit(body: &str, disabled: bool) -> bool {
    !disabled && !body.trim().is_empty()
}

/// Remove a draft attachment by id.
pub fn remove_attachment_draft(drafts: RwSignal<Vec<DiscussionAttachmentDraft>>, id: &str) {
    drafts.update(|list| list.retain(|draft| draft.id != id));
}

/// Append a validated draft attachment.
pub fn add_attachment_draft(
    drafts: RwSignal<Vec<DiscussionAttachmentDraft>>,
    draft: DiscussionAttachmentDraft,
) {
    drafts.update(|list| list.push(draft));
}

/// Remove a draft citation by id.
pub fn remove_citation_draft(drafts: RwSignal<Vec<DiscussionCitation>>, id: &str) {
    drafts.update(|list| list.retain(|draft| draft.id != id));
}

/// Append a citation draft.
pub fn add_citation_draft(drafts: RwSignal<Vec<DiscussionCitation>>, draft: DiscussionCitation) {
    drafts.update(|list| list.push(draft));
}

fn mime_matches(accepted: &str, mime: &str) -> bool {
    if accepted.ends_with("/*") {
        let prefix = accepted.trim_end_matches('*');
        mime.starts_with(prefix)
    } else {
        accepted == mime
    }
}

/// Validate file metadata against optional composer rules.
pub fn validate_attachment_metadata(
    name: &str,
    mime: Option<&str>,
    size_bytes: Option<u64>,
    current_count: usize,
    config: Option<&DiscussionAttachmentValidation>,
) -> Result<(), &'static str> {
    let Some(config) = config else {
        return Ok(());
    };

    if let Some(max_count) = config.max_file_count {
        if current_count as u32 >= max_count {
            return Err("file-count");
        }
    }

    if let Some(max_size) = config.max_file_size {
        if let Some(size) = size_bytes {
            if size > max_size {
                return Err("file-size");
            }
        }
    }

    if let Some(accepted) = &config.accepted_mime_types {
        if accepted.is_empty() {
            return Ok(());
        }
        let mime = mime.unwrap_or("application/octet-stream");
        if !accepted.iter().any(|pattern| mime_matches(pattern, mime)) {
            return Err("mime-type");
        }
    }

    let _ = name;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        can_submit, mime_matches, validate_attachment_metadata, DiscussionAttachmentValidation,
    };

    #[test]
    fn can_submit_when_non_empty_and_enabled() {
        assert!(can_submit("hello", false));
    }

    #[test]
    fn cannot_submit_when_empty_or_whitespace() {
        assert!(!can_submit("", false));
        assert!(!can_submit("   ", false));
    }

    #[test]
    fn cannot_submit_when_disabled() {
        assert!(!can_submit("hello", true));
    }

    #[test]
    fn mime_wildcard_matches() {
        assert!(mime_matches("image/*", "image/png"));
        assert!(!mime_matches("image/*", "application/pdf"));
    }

    #[test]
    fn validates_file_count() {
        let config = DiscussionAttachmentValidation {
            max_file_count: Some(1),
            ..Default::default()
        };
        assert!(validate_attachment_metadata(
            "a.png",
            Some("image/png"),
            Some(100),
            0,
            Some(&config)
        )
        .is_ok());
        assert!(validate_attachment_metadata(
            "b.png",
            Some("image/png"),
            Some(100),
            1,
            Some(&config)
        )
        .is_err());
    }
}
