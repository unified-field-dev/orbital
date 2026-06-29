use std::future::Future;
use std::pin::Pin;

use super::{
    DiscussionAttachmentDraft, DiscussionComposerSubmit, DiscussionError, DiscussionReply,
    DiscussionUploadedFile,
};

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// Host-owned backend contract for persisting replies and lazy-loading branches.
pub trait DiscussionAdapter: Send + Sync {
    /// Persist a new reply from the composer draft.
    fn submit_reply(
        &self,
        draft: DiscussionComposerSubmit,
    ) -> BoxFuture<Result<DiscussionReply, DiscussionError>>;

    /// Upload a single attachment before or during submit.
    ///
    /// Default returns [`DiscussionError::not_implemented`]; override when the host
    /// provides file storage. Preview adapters may return placeholder URLs.
    fn upload_attachment(
        &self,
        _draft: &DiscussionAttachmentDraft,
    ) -> BoxFuture<Result<DiscussionUploadedFile, DiscussionError>> {
        Box::pin(async { Err(DiscussionError::not_implemented("upload_attachment")) })
    }

    /// Optional lazy-load hook for hidden branch children.
    fn fetch_branch(
        &self,
        _parent_id: String,
        _cursor: Option<String>,
    ) -> BoxFuture<Result<Vec<DiscussionReply>, DiscussionError>> {
        Box::pin(async { Err(DiscussionError::not_implemented("fetch_branch")) })
    }
}
