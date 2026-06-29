use leptos::context::Provider;
use leptos::prelude::*;
use orbital_base_components::{
    BaseToastStack, FeedbackIntent, ToastStackPosition, ToasterConfig, DEFAULT_TOAST_LIMIT,
    DEFAULT_TOAST_TIMEOUT_MS,
};

pub use orbital_base_components::ToastStackPosition as ToastPosition;
pub use orbital_base_components::{
    ToastAction, ToastId, ToastOffset, ToastOptions, ToasterInjection,
    DEFAULT_TOAST_TIMEOUT_MS as ToastDefaultTimeoutMs,
};

/// Severity preset for dispatched toasts.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastIntent {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl From<ToastIntent> for FeedbackIntent {
    fn from(value: ToastIntent) -> Self {
        match value {
            ToastIntent::Info => FeedbackIntent::Info,
            ToastIntent::Success => FeedbackIntent::Success,
            ToastIntent::Warning => FeedbackIntent::Warning,
            ToastIntent::Error => FeedbackIntent::Error,
        }
    }
}

/// Provides toast dispatch context and mounts the toast stack.
#[component]
pub fn ToasterProvider(
    /// Default position for dispatched toasts.
    #[prop(default = ToastStackPosition::BottomEnd)]
    position: ToastPosition,
    /// Default intent for dispatched toasts.
    #[prop(optional)]
    intent: ToastIntent,
    /// Auto-dismiss timeout in milliseconds. Use `-1` for persistent toasts.
    #[prop(default = DEFAULT_TOAST_TIMEOUT_MS)]
    timeout: i32,
    /// Pause the dismiss timer while the pointer is over a toast.
    #[prop(default = false)]
    pause_on_hover: bool,
    /// Maximum number of visible toasts before queueing.
    #[prop(default = DEFAULT_TOAST_LIMIT)]
    limit: u32,
    /// Viewport inset for the toast stack.
    #[prop(optional)]
    offset: ToastOffset,
    /// Render toasts in DOM order instead of a portal.
    #[prop(optional)]
    inline: bool,
    children: Children,
) -> impl IntoView {
    let config = ToasterConfig {
        default_position: position,
        default_intent: intent.into(),
        default_timeout: timeout,
        default_pause_on_hover: pause_on_hover,
        limit,
        offset,
        inline,
    };

    let injection = ToasterInjection::new(config);

    view! {
        <Provider value=injection>
            {children()}
            <BaseToastStack />
        </Provider>
    }
}
