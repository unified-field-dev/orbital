//! Client-side auth session helpers.

use crate::context::AuthContext;
use crate::models::auth::AuthSession;
use leptos::prelude::*;

/// Keep [`AuthContext`] aligned with the current session signal.
///
/// Standalone preview hosts use the provided context as-is. Integrators replace this with a resource backed by their session server function.
pub fn init_auth_resource(auth: AuthContext) -> Resource<Result<AuthSession, ServerFnError>> {
    let reload_token = auth.reload_token();
    let session_signal = auth.session();

    Resource::new(
        move || reload_token.get(),
        move |_| {
            let session = session_signal.get_untracked();
            async move { Ok(session) }
        },
    )
}
