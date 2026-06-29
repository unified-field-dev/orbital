use std::future::Future;
use std::pin::Pin;

use crate::{
    DiscussionAdapter, DiscussionAttachmentDraft, DiscussionComposerSubmit, DiscussionError,
    DiscussionReply, DiscussionUploadedFile,
};

use super::fixtures::{reply_from_submit, PREVIEW_VIEWER_AUTHOR_ID};

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// In-memory adapter for catalog previews — no HTTP or WebSocket.
pub struct MockDiscussionAdapter {
    author_id: String,
    author_name: String,
}

impl MockDiscussionAdapter {
    pub fn preview_viewer() -> Self {
        Self::new(PREVIEW_VIEWER_AUTHOR_ID, "Jordan Lee")
    }

    pub fn new(author_id: impl Into<String>, author_name: impl Into<String>) -> Self {
        Self {
            author_id: author_id.into(),
            author_name: author_name.into(),
        }
    }
}

impl DiscussionAdapter for MockDiscussionAdapter {
    fn submit_reply(
        &self,
        draft: DiscussionComposerSubmit,
    ) -> BoxFuture<Result<DiscussionReply, DiscussionError>> {
        let author_id = self.author_id.clone();
        let author_name = self.author_name.clone();
        Box::pin(async move { Ok(reply_from_submit(&draft, &author_id, &author_name)) })
    }

    fn upload_attachment(
        &self,
        draft: &DiscussionAttachmentDraft,
    ) -> BoxFuture<Result<DiscussionUploadedFile, DiscussionError>> {
        let draft = draft.clone();
        Box::pin(async move {
            Ok(DiscussionUploadedFile {
                url: format!("https://orbital.dev/files/{}", draft.name),
                name: draft.name.clone(),
                mime: draft.mime.clone(),
                size_bytes: draft.size_bytes,
            })
        })
    }

    fn fetch_branch(
        &self,
        parent_id: String,
        _cursor: Option<String>,
    ) -> BoxFuture<Result<Vec<DiscussionReply>, DiscussionError>> {
        Box::pin(async move {
            if parent_id == "r-pat" {
                Ok(vec![
                    reply_from_submit(
                        &DiscussionComposerSubmit {
                            body_markdown: "Lazy-loaded branch reply 1.".into(),
                            parent_id: Some(parent_id.clone()),
                            attachments: vec![],
                            citations: vec![],
                        },
                        "lazy-user",
                        "Lazy User",
                    ),
                    reply_from_submit(
                        &DiscussionComposerSubmit {
                            body_markdown: "Lazy-loaded branch reply 2.".into(),
                            parent_id: Some(parent_id),
                            attachments: vec![],
                            citations: vec![],
                        },
                        "lazy-user",
                        "Lazy User",
                    ),
                ])
            } else {
                Ok(vec![])
            }
        })
    }
}
