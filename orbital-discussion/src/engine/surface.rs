use crate::{DiscussionLabel, DiscussionReply, DiscussionReplySurface};

/// Resolve card surface tint for a reply.
///
/// OP wins when the viewer is also the OP author.
pub fn resolve_reply_surface(
    reply: &DiscussionReply,
    current_user_id: Option<&str>,
) -> DiscussionReplySurface {
    let is_op = reply
        .metadata
        .labels
        .iter()
        .any(|label| matches!(label, DiscussionLabel::Op))
        || reply.parent_id.is_none();

    if is_op {
        return DiscussionReplySurface::Op;
    }

    if let Some(user_id) = current_user_id {
        if reply.author.id == user_id {
            return DiscussionReplySurface::Viewer;
        }
    }

    DiscussionReplySurface::Neutral
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DiscussionAuthor, DiscussionAuthorRole, DiscussionMetadata, DiscussionPart,
        DiscussionReplyStatus,
    };
    use chrono::Utc;

    fn reply(id: &str, parent: Option<&str>, op: bool, author_id: &str) -> DiscussionReply {
        DiscussionReply {
            id: id.to_string(),
            parent_id: parent.map(str::to_string),
            author: DiscussionAuthor::new(author_id, "Author")
                .with_role(DiscussionAuthorRole::User),
            metadata: DiscussionMetadata {
                created_at: Utc::now(),
                edited_at: None,
                labels: if op {
                    vec![DiscussionLabel::Op]
                } else {
                    vec![]
                },
            },
            parts: vec![DiscussionPart::text("body")],
            citations: vec![],
            status: DiscussionReplyStatus::Ready,
            hidden_child_count: 0,
        }
    }

    #[test]
    fn op_label_wins() {
        let r = reply("r1", Some("root"), true, "u1");
        assert_eq!(
            resolve_reply_surface(&r, Some("u1")),
            DiscussionReplySurface::Op
        );
    }

    #[test]
    fn viewer_when_not_op() {
        let r = reply("r1", Some("root"), false, "u2");
        assert_eq!(
            resolve_reply_surface(&r, Some("u2")),
            DiscussionReplySurface::Viewer
        );
    }

    #[test]
    fn neutral_default() {
        let r = reply("r1", Some("root"), false, "u3");
        assert_eq!(
            resolve_reply_surface(&r, Some("u2")),
            DiscussionReplySurface::Neutral
        );
    }

    #[test]
    fn root_without_op_label_is_op_surface() {
        let mut r = reply("root", None, false, "u1");
        r.metadata.labels.clear();
        assert_eq!(resolve_reply_surface(&r, None), DiscussionReplySurface::Op);
    }
}
