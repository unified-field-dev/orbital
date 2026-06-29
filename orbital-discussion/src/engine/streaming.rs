use crate::{DiscussionReply, DiscussionReplyStatus};

/// Returns true when any reply in the thread is actively streaming.
pub fn thread_has_streaming_reply(replies: &[DiscussionReply]) -> bool {
    replies
        .iter()
        .any(|reply| reply.status == DiscussionReplyStatus::Streaming)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DiscussionAuthor, DiscussionMetadata, DiscussionPart, DiscussionReply,
        DiscussionReplyStatus,
    };
    use chrono::Utc;

    fn reply(id: &str, status: DiscussionReplyStatus) -> DiscussionReply {
        DiscussionReply {
            id: id.to_string(),
            parent_id: None,
            author: DiscussionAuthor::new(id, id),
            metadata: DiscussionMetadata {
                created_at: Utc::now(),
                edited_at: None,
                labels: vec![],
            },
            parts: vec![DiscussionPart::text("body")],
            citations: vec![],
            status,
            hidden_child_count: 0,
        }
    }

    #[test]
    fn detects_streaming_reply() {
        let replies = vec![
            reply("a", DiscussionReplyStatus::Ready),
            reply("b", DiscussionReplyStatus::Streaming),
        ];
        assert!(thread_has_streaming_reply(&replies));
    }

    #[test]
    fn returns_false_when_no_streaming() {
        let replies = vec![
            reply("a", DiscussionReplyStatus::Ready),
            reply("b", DiscussionReplyStatus::Sending),
        ];
        assert!(!thread_has_streaming_reply(&replies));
    }
}
