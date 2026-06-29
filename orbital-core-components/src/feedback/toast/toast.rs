use leptos::prelude::*;
use orbital_base_components::{BaseToast, FeedbackIntent};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::toaster_styles;

/// `Toast` delivers short-lived feedback — saves, uploads, and errors that should not block the UI.
///
/// Mount one [`ToasterProvider`] near your app root, then call [`ToasterInjection::dispatch`] with [`ToastOptions`] or a composed [`Toast`] view. Stack position, timeout (default **8000** ms), queue limit, and hover-pause are provider defaults; override per toast when needed. [`ToasterInjection::dismiss_all`] clears only the nearest provider's toasts.
///
/// # When to use
///
/// - Confirmations that auto-dismiss (saved, copied, uploaded)
/// - Errors with optional retry or undo in the footer
/// - Non-blocking feedback while the user keeps working
///
/// Prefer [`MessageBar`](crate::MessageBar) for persistent page or section status. Prefer [`Field`](crate::Field) validation for errors tied to a single form control.
///
/// # API notes
///
/// - Mount one [`ToasterProvider`] near your app root and call [`ToasterInjection::dispatch`] with [`ToastOptions`].
/// - Rich content via `Toast` + `ToastTitle` / `ToastBody` / `ToastFooter` slots.
/// - [`ToasterInjection::dismiss_all`] clears only the nearest provider's toasts.
/// - Prefer [`MessageBar`](crate::MessageBar) for persistent page or section status.
///
/// # Usage
///
/// 1. Wrap your app shell in a single [`ToasterProvider`].
/// 2. From any descendant, call `ToasterInjection::expect_context().dispatch(ToastOptions::new("…"))`.
/// 3. Set `intent`, `timeout`, or `position` on `ToastOptions`; use `timeout(-1)` for persistent toasts.
/// 4. For full slot control, pass `ToastOptions::composed(|| view! { … })` with [`ToastTitle`], [`ToastBody`], and optional [`ToastFooter`].
/// 5. Dismiss by id with `dismiss`, or clear the provider stack with `dismiss_all` (scoped to that provider only).
///
/// # Best Practices
///
/// ## Do's
///
/// * Mount **one** provider per app shell — same pattern as [`LoadingBarProvider`](crate::LoadingBarProvider)
/// * Use `footer_action_callback` for retry or undo actions on error toasts
/// * Use static `<Toast>` composition for catalog previews; dispatch at runtime in app code
///
/// ## Don'ts
///
/// * Do not use toasts for status that must stay visible until resolved — use [`MessageBar`](crate::MessageBar)
/// * Do not expect `dismiss_all` to clear toasts from other providers on the page
///
/// # Examples
///
/// ## Static composition
/// Compose title, body, and optional footer for inline documentation or layout previews without dispatching.
/// <!-- preview -->
/// ```rust
/// use crate::{Toast, ToastBody, ToastIntent, ToastTitle};
/// view! {
///     <div data-testid="toast-static">
///         <Toast intent=Signal::from(ToastIntent::Success)>
///             <ToastTitle>"Saved"</ToastTitle>
///             <ToastBody>"Your changes were saved."</ToastBody>
///         </Toast>
///     </div>
/// }
/// ```
///
/// ## Provider dispatch
/// Mount [`ToasterProvider`] and dispatch toasts via [`ToasterInjection::dispatch`]. Set `intent` to apply semantic background and border tokens from the theme.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// use orbital_base_components::FeedbackIntent;
/// view! {
///     <div data-testid="toast-provider">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Hello from toast").timeout(1500)
///                 );
///             })>"Show toast"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Saved").intent(FeedbackIntent::Success)
///                 );
///             })>"Show success"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Check settings").intent(FeedbackIntent::Warning)
///                 );
///             })>"Show warning"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Upload failed").intent(FeedbackIntent::Error)
///                 );
///             })>"Show error"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Success intent
/// Success toasts use green semantic tokens for completed actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Toast, ToastBody, ToastIntent, ToastTitle};
/// view! {
///     <div data-testid="toast-success">
///         <Toast intent=Signal::from(ToastIntent::Success)>
///             <ToastTitle>"Uploaded"</ToastTitle>
///             <ToastBody>"File upload completed."</ToastBody>
///         </Toast>
///     </div>
/// }
/// ```
///
/// ## Error with action
/// Error toasts can include a footer action for retry or undo. Use [`ToastOptions::footer_action_callback`] when the action should run custom logic.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// use leptos::prelude::*;
/// use orbital_base_components::FeedbackIntent;
/// view! {
///     <div data-testid="toast-error-action">
///         <ToasterProvider>
///             {
///                 let retry_count = RwSignal::new(0_u32);
///                 let on_retry = Callback::new(move |_| {
///                     retry_count.update(|count| *count += 1);
///                 });
///                 view! {
///                     <p data-testid="toast-retry-count">{move || format!("Retries: {}", retry_count.get())}</p>
///                     <Button on_click=Callback::new(move |_| {
///                         ToasterInjection::expect_context().dispatch(
///                             ToastOptions::new("Upload failed")
///                                 .intent(FeedbackIntent::Error)
///                                 .body("Try again shortly.")
///                                 .footer_action_callback("Retry", true, Some(on_retry))
///                         );
///                     })>"Show error toast"</Button>
///                 }
///             }
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Bottom-end position
/// Stack toasts in the bottom-end corner for non-blocking notifications.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToastPosition, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-position">
///         <ToasterProvider position=ToastPosition::BottomEnd>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Positioned toast")
///                 );
///             })>"Show toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Custom timeout
/// Override auto-dismiss duration per toast. Negative values keep the toast until dismissed.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-timeout">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Timed toast").timeout(500)
///                 );
///             })>"Show timed toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Persistent toast
/// Toasts with a negative timeout stay until explicitly dismissed.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-persistent">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Persistent toast").timeout(-1)
///                 );
///             })>"Show persistent toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Dismiss toast
/// Dispatch with a known id, then dismiss imperatively.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="toast-dismiss">
///         <ToasterProvider>
///             {
///                 let toast_id = RwSignal::new(String::new());
///                 view! {
///                     <Button on_click=Callback::new(move |_| {
///                         let id = ToasterInjection::expect_context().dispatch(
///                             ToastOptions::new("Dismiss me").id("toast-dismiss-example")
///                         );
///                         toast_id.set(id);
///                     })>"Make toast"</Button>
///                     <Button on_click=Callback::new(move |_| {
///                         let id = toast_id.get();
///                         if !id.is_empty() {
///                             ToasterInjection::expect_context().dismiss(&id);
///                         }
///                     })>"Dismiss toast"</Button>
///                 }
///             }
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Dismiss all
/// Clear every visible and queued toast for this provider. Other preview examples on the catalog page use separate providers, so their toasts are not affected.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-dismiss-all">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Queued toast")
///                 );
///             })>"Make toast"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dismiss_all();
///             })>"Dismiss all"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Dismiss with action
/// Footer actions that dismiss use [`ToastTrigger`] under the hood.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// use orbital_base_components::FeedbackIntent;
/// view! {
///     <div data-testid="toast-dismiss-action">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Dismiss me")
///                         .intent(FeedbackIntent::Success)
///                         .footer_action("Dismiss", true)
///                 );
///             })>"Make toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Composed dispatch
/// Same slot-based composition as static `<Toast>`, dispatched via [`ToastOptions::composed`].
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Toast, ToastBody, ToastFooter, ToastIntent, ToastOptions, ToastTitle, ToastTrigger, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-composed-dispatch">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::composed(|| view! {
///                         <Toast intent=Signal::from(ToastIntent::Info)>
///                             <ToastTitle>"Update available"</ToastTitle>
///                             <ToastBody>"Restart to apply changes."</ToastBody>
///                             <ToastFooter>
///                                 <ToastTrigger>
///                                     <Button appearance=ButtonAppearance::Transparent class="orbital-toast-footer__action">"Later"</Button>
///                                 </ToastTrigger>
///                             </ToastFooter>
///                         </Toast>
///                     })
///                 );
///             })>"Make toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Default composition
/// Dispatch with [`ToastTitle`], [`ToastBody`], and [`ToastFooter`] slots. Uses the provider default auto-dismiss timeout (8000ms).
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Toast, ToastBody, ToastFooter, ToastIntent, ToastOptions, ToastTitle, ToastTrigger, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-default-composition">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::composed(|| view! {
///                         <Toast intent=Signal::from(ToastIntent::Success)>
///                             <ToastTitle>"Email sent"</ToastTitle>
///                             <ToastBody>"Your message was delivered."</ToastBody>
///                             <ToastFooter>
///                                 <ToastTrigger>
///                                     <Button appearance=ButtonAppearance::Transparent class="orbital-toast-footer__action">"Undo"</Button>
///                                 </ToastTrigger>
///                             </ToastFooter>
///                         </Toast>
///                     })
///                 );
///             })>"Make toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Offset
/// Adjust viewport inset (px from the stack edge). Position defaults to bottom-end; offset only changes how far the stack sits from the viewport edge.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToastOffset, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-offset">
///         <ToasterProvider offset=ToastOffset { horizontal: 40, vertical: 60 }>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Offset toast")
///                 );
///             })>"Make toast"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Default inset toast")
///                 );
///             })>"Default inset"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Pause on hover
/// Hovering pauses the dismiss timer until the pointer leaves.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-pause-hover">
///         <ToasterProvider>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Hover me")
///                         .timeout(800)
///                         .pause_on_hover(true)
///                 );
///             })>"Make toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Toast limit
/// Extra toasts queue until a visible toast is dismissed.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-limit">
///         <ToasterProvider limit=3>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Limited toast")
///                         .footer_action("Dismiss", true)
///                 );
///             })>"Make toast"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
///
/// ## Inline toaster
/// Render toasts inside a positioned container instead of a portal.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToasterInjection, ToasterProvider};
/// view! {
///     <div data-testid="toast-inline">
///         <div
///             data-testid="toast-inline-container"
///             style="position: relative; width: 500px; height: 500px; border: 2px dashed green; display: flex; align-items: center; justify-content: center;"
///         >
///             "Toasts appear here"
///             <ToasterProvider inline=true>
///                 <Button on_click=Callback::new(|_| {
///                     ToasterInjection::expect_context().dispatch(
///                         ToastOptions::new("Inline toast")
///                     );
///                 })>"Make toast"</Button>
///             </ToasterProvider>
///         </div>
///     </div>
/// }
/// ```
///
/// ## All positions
/// Dispatch toasts to each stack position, including per-toast overrides.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ToastOptions, ToastPosition, ToasterInjection, ToasterProvider};
/// use orbital_base_components::ToastStackPosition;
/// view! {
///     <div data-testid="toast-positions">
///         <ToasterProvider position=ToastPosition::BottomEnd>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Top start").position(ToastStackPosition::TopStart)
///                 );
///             })>"Top start"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Top end").position(ToastStackPosition::TopEnd)
///                 );
///             })>"Top end"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Top").position(ToastStackPosition::Top)
///                 );
///             })>"Top"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Bottom start").position(ToastStackPosition::BottomStart)
///                 );
///             })>"Bottom start"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Bottom end").position(ToastStackPosition::BottomEnd)
///                 );
///             })>"Bottom end"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Bottom").position(ToastStackPosition::Bottom)
///                 );
///             })>"Bottom"</Button>
///             <Button on_click=Callback::new(|_| {
///                 ToasterInjection::expect_context().dispatch(
///                     ToastOptions::new("Override top start").position(ToastStackPosition::TopStart)
///                 );
///             })>"Override top start"</Button>
///         </ToasterProvider>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "toast",
    preview_label = "Toast",
    preview_icon = icondata::AiNotificationOutlined,
)]
#[component]
pub fn Toast(
    /// Optional CSS class on the toast root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Severity preset for icon and color.
    #[prop(optional, into)]
    intent: Signal<super::provider::ToastIntent>,
    /// Title, body, and footer slots.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-toaster", toaster_styles());

    view! {
        <BaseToast
            class=class
            intent=Signal::derive(move || FeedbackIntent::from(intent.get()))
        >
            {children()}
        </BaseToast>
    }
}
