/// Well-known shell-level path constants for auth and account flows.
///
/// Shared route constants used by shell components and auth guards (for example
/// [`orbital::routes::RequireAuthenticated`]) without hardcoded path strings.
pub const AUTH_SIGNIN: &str = "/auth/signin";
pub const AUTH_SIGNUP: &str = "/auth/signup";
pub const AUTH_LOGOUT: &str = "/auth/logout";
pub const AUTH_RESET_PASSWORD_REQUEST: &str = "/auth/reset/request";
pub const AUTH_RESET_PASSWORD_CONFIRM: &str = "/auth/reset/confirm";
pub const USER_PROFILE: &str = "/user/profile";
pub const USER_ACCOUNT_SETTINGS: &str = "/user/account-settings";
pub const NOTIFICATIONS_INBOX: &str = "/notifications";
pub const PERMISSION_PERMISSIONS: &str = "/permission/permissions";
