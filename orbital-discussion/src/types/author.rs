use serde::{Deserialize, Serialize};

/// Role of a reply author in a discussion thread.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionAuthorRole {
    #[default]
    User,
    Agent,
    Assistant,
    Moderator,
    System,
}

/// Author metadata for a single reply.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionAuthor {
    pub id: String,
    pub display_name: String,
    pub role: DiscussionAuthorRole,
    pub avatar_url: Option<String>,
    pub avatar_name: String,
}

impl DiscussionAuthor {
    /// Convenience constructor for previews and tests.
    pub fn new(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        let display_name = display_name.into();
        Self {
            id: id.into(),
            display_name: display_name.clone(),
            role: DiscussionAuthorRole::User,
            avatar_url: None,
            avatar_name: display_name,
        }
    }

    /// Mark this author as an agent participant.
    pub fn with_role(mut self, role: DiscussionAuthorRole) -> Self {
        self.role = role;
        self
    }
}
