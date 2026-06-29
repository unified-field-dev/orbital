//! Frontend-facing authentication session models.
//!
//! Orbital keeps browser-visible auth state intentionally small: enough information
//! to personalize the UI, gate pages, and show account status without exposing the
//! full backend user model. These types are the shared contract between the SSR
//! session loader and the client-side component tree.
//!
//! ## Example
//!
//! ```rust
//! use orbital::{AnonymousUser, AuthSession, AuthenticatedUser};
//!
//! let anonymous = AuthSession::Anonymous(AnonymousUser::default());
//! assert!(!anonymous.is_authenticated());
//! assert_eq!(anonymous.display_label(), "Guest");
//!
//! let authenticated = AuthSession::Authenticated(AuthenticatedUser {
//!     user_id: "user-123".to_string(),
//!     email: Some("alex@example.com".to_string()),
//!     display_name: Some("Alex".to_string()),
//!     avatar_url: None,
//!     roles: vec!["admin".to_string()],
//!     email_verified: true,
//! });
//!
//! assert!(authenticated.is_authenticated());
//! assert_eq!(authenticated.display_label(), "Alex");
//! ```

use serde::{Deserialize, Serialize};

/// Represents the current authentication session for the frontend.
///
/// UI code generally branches on this enum to decide whether to render anonymous onboarding, authenticated content, or additional account verification prompts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthSession {
    /// No authenticated user is present.
    Anonymous(AnonymousUser),
    /// An authenticated user is available with profile information.
    Authenticated(AuthenticatedUser),
}

impl Default for AuthSession {
    fn default() -> Self {
        Self::Anonymous(AnonymousUser::default())
    }
}

impl AuthSession {
    /// Returns `true` when the session represents an authenticated user.
    pub fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated(_))
    }

    /// Convenience accessor for the authenticated user, if present.
    pub fn user(&self) -> Option<&AuthenticatedUser> {
        match self {
            Self::Authenticated(user) => Some(user),
            Self::Anonymous(_) => None,
        }
    }

    /// Provides a display-friendly identifier for UI surfaces.
    pub fn display_label(&self) -> String {
        match self {
            Self::Authenticated(user) => user
                .display_name
                .as_ref()
                .or(user.email.as_ref())
                .cloned()
                .unwrap_or_else(|| user.user_id.clone()),
            Self::Anonymous(_) => "Guest".to_string(),
        }
    }
}

/// Additional context for an anonymous visitor.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnonymousUser {
    /// Optional explanation for why the user is anonymous (e.g. session expired).
    pub reason: Option<String>,
}

/// Core identity and profile details for an authenticated user.
///
/// This is the browser-safe subset of user information commonly needed by page headers, permission gates, account menus, and audit-friendly UI.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    /// Stable identifier for the user (e.g. database key).
    pub user_id: String,
    /// Email address when available.
    pub email: Option<String>,
    /// Preferred display name.
    pub display_name: Option<String>,
    /// Optional avatar image URL.
    pub avatar_url: Option<String>,
    /// Ordered list of role or capability identifiers.
    pub roles: Vec<String>,
    /// Whether the account's primary email is verified.
    pub email_verified: bool,
}
