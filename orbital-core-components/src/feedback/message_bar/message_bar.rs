use leptos::prelude::*;
use orbital_base_components::{
    BaseMessageBar, FeedbackIntent, MessageBarLayout as BaseMessageBarLayout,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::message_bar_styles;

/// Severity preset controlling icon and color (`Info`, `Success`, `Warning`, `Error`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MessageBarIntent {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl MessageBarIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

impl From<MessageBarIntent> for FeedbackIntent {
    fn from(value: MessageBarIntent) -> Self {
        match value {
            MessageBarIntent::Info => FeedbackIntent::Info,
            MessageBarIntent::Success => FeedbackIntent::Success,
            MessageBarIntent::Warning => FeedbackIntent::Warning,
            MessageBarIntent::Error => FeedbackIntent::Error,
        }
    }
}

pub use orbital_base_components::MessageBarLayout;

/// `MessageBar` shows persistent status at the top of a page or section — session warnings, connectivity loss, or validation summaries that should stay visible until resolved.
///
/// Pick [`MessageBarIntent`] for severity, switch to [`MessageBarLayout::Multiline`] when you need a title, body, and action row. For fleeting confirmations, use [`Toast`](crate::Toast) instead.
///
/// # When to use
///
/// - Page-level or section-level status that must remain visible until the user acts
/// - Warnings with optional retry or dismiss actions in [`MessageBarActions`]
///
/// Prefer [`Toast`](crate::Toast) for short-lived saves or errors that should not block the UI. Prefer [`Field`](crate::Field) validation for errors tied to a single form control.
///
/// # Usage
///
/// 1. Set `intent` to match severity (`Info`, `Success`, `Warning`, `Error`).
/// 2. Put primary copy in [`MessageBarBody`]; add [`MessageBarTitle`] when the message needs a headline.
/// 3. Use `layout=MessageBarLayout::Multiline` when stacking title, body, and [`MessageBarActions`].
/// 4. Place retry or dismiss [`Button`](crate::Button) children in [`MessageBarActions`] — there is no built-in close icon.
///
/// # Examples
///
/// ## Intent matrix
/// Show all four severity presets side by side so teams can pick the right tone for status, success, warning, and error states.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::MessageBarBody;
/// view! {
///     <div data-testid="message-bar-preview">
///     <div data-testid="message-bar-intents" style="display: flex; flex-direction: column; gap: 8px;">
///         <MessageBar intent=MessageBarIntent::Info><MessageBarBody>"Your session expires in 10 minutes."</MessageBarBody></MessageBar>
///         <MessageBar intent=MessageBarIntent::Success><MessageBarBody>"Your profile was updated successfully."</MessageBarBody></MessageBar>
///         <MessageBar intent=MessageBarIntent::Warning><MessageBarBody>"Check your network connection."</MessageBarBody></MessageBar>
///         <MessageBar intent=MessageBarIntent::Error><MessageBarBody>"Upload failed. Try again."</MessageBarBody></MessageBar>
///     </div>
///     </div>
/// }
/// ```
///
/// ## With title
/// Pair a bold title with supporting body copy — one example per intent on the default single-line layout.
/// <!-- preview -->
/// ```rust
/// use crate::{MessageBarBody, MessageBarTitle};
/// view! {
///     <div data-testid="message-bar-with-title" style="display: flex; flex-direction: column; gap: 8px;">
///         <MessageBar intent=MessageBarIntent::Info>
///             <MessageBarTitle>"Session ending"</MessageBarTitle>
///             <MessageBarBody>"Your session expires in 10 minutes."</MessageBarBody>
///         </MessageBar>
///         <MessageBar intent=MessageBarIntent::Success>
///             <MessageBarTitle>"Saved"</MessageBarTitle>
///             <MessageBarBody>"Your profile was updated successfully."</MessageBarBody>
///         </MessageBar>
///         <MessageBar intent=MessageBarIntent::Warning>
///             <MessageBarTitle>"Connection lost"</MessageBarTitle>
///             <MessageBarBody>"Check your network and try again."</MessageBarBody>
///         </MessageBar>
///         <MessageBar intent=MessageBarIntent::Error>
///             <MessageBarTitle>"Upload failed"</MessageBarTitle>
///             <MessageBarBody>"The file exceeds the 10 MB limit."</MessageBarBody>
///         </MessageBar>
///     </div>
/// }
/// ```
///
/// ## Multiline
/// Multiline layout stacks title and body when the message needs more room or a footer action row below.
/// <!-- preview -->
/// ```rust
/// use crate::{MessageBarBody, MessageBarLayout, MessageBarTitle};
/// view! {
///     <div data-testid="message-bar-multiline" style="display: flex; flex-direction: column; gap: 8px;">
///         <MessageBar intent=MessageBarIntent::Info layout=MessageBarLayout::Multiline>
///             <MessageBarTitle>"Session ending"</MessageBarTitle>
///             <MessageBarBody>"Your session expires in 10 minutes. Save any unsaved work before you are signed out."</MessageBarBody>
///         </MessageBar>
///         <MessageBar intent=MessageBarIntent::Success layout=MessageBarLayout::Multiline>
///             <MessageBarTitle>"Saved"</MessageBarTitle>
///             <MessageBarBody>"Your profile was updated successfully. Changes may take a moment to appear elsewhere."</MessageBarBody>
///         </MessageBar>
///         <MessageBar intent=MessageBarIntent::Warning layout=MessageBarLayout::Multiline>
///             <MessageBarTitle>"Connection lost"</MessageBarTitle>
///             <MessageBarBody>"Check your network and try again. Some features may be unavailable until you reconnect."</MessageBarBody>
///         </MessageBar>
///         <MessageBar intent=MessageBarIntent::Error layout=MessageBarLayout::Multiline>
///             <MessageBarTitle>"Upload failed"</MessageBarTitle>
///             <MessageBarBody>"The file exceeds the 10 MB limit. Choose a smaller file and try again."</MessageBarBody>
///         </MessageBar>
///     </div>
/// }
/// ```
///
/// ## With actions
/// Place retry or dismiss actions in the footer so users can respond without leaving the banner context.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MessageBarActions, MessageBarBody, MessageBarTitle};
/// view! {
///     <div data-testid="message-bar-actions">
///         <MessageBar intent=MessageBarIntent::Warning layout=MessageBarLayout::Multiline>
///             <MessageBarTitle>"Connection lost"</MessageBarTitle>
///             <MessageBarBody>"Check your network and try again."</MessageBarBody>
///             <MessageBarActions>
///                 <Button>"Retry"</Button>
///             </MessageBarActions>
///         </MessageBar>
///     </div>
/// }
/// ```
///
/// ## Theme token
/// Message bar borders and backgrounds inherit intent tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::MessageBarBody;
/// view! {
///     <div data-testid="message-bar-theme">
///         <MessageBar intent=MessageBarIntent::Info>
///             <MessageBarBody>"Themed message bar surface"</MessageBarBody>
///         </MessageBar>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "message-bar",
    preview_label = "Message Bar",
    preview_icon = icondata::AiInfoCircleOutlined,
)]
#[component]
pub fn MessageBar(
    /// Optional CSS class on the banner root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Single-line (default) or multiline layout for longer copy and actions.
    #[prop(optional, into)]
    layout: MessageBarLayout,
    /// Severity preset controlling icon and color (`Info`, `Success`, `Warning`, `Error`).
    #[prop(optional, into)]
    intent: Signal<MessageBarIntent>,
    /// Compound tree: [`MessageBarTitle`], [`MessageBarBody`], [`MessageBarActions`].
    children: Children,
) -> impl IntoView {
    inject_style("orbital-message-bar", message_bar_styles());

    let base_layout = match layout {
        MessageBarLayout::Singleline => BaseMessageBarLayout::Singleline,
        MessageBarLayout::Multiline => BaseMessageBarLayout::Multiline,
    };

    view! {
        <BaseMessageBar
            class=class
            layout=base_layout
            intent=Signal::derive(move || intent.get().into())
        >
            {children()}
        </BaseMessageBar>
    }
}
