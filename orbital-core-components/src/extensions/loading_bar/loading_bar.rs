#![allow(dead_code)]
use leptos::prelude::*;
use orbital_macros::component_doc;

use super::provider::LoadingBarProvider;

/// `LoadingBar` is Orbital's global navigation chrome indicator — a slim teleported bar for route changes and coarse async work.
///
/// Mount a single [`LoadingBarProvider`] (or this [`LoadingBar`] alias) near your app root, then call [`LoadingBarInjection::start`], [`LoadingBarInjection::finish`], or [`LoadingBarInjection::error`] from handlers. The bar itself has no props — the provider teleports it to the viewport top.
///
/// # When to use
///
/// - Route transitions and full-page data fetches
/// - Long-running saves where a slim top indicator is enough
/// - App-wide async that should not block page interaction
///
/// Prefer [`Spinner`](crate::Spinner) for inline panel waits. Prefer [`ProgressBar`](crate::ProgressBar) when you have a known completion percentage.
///
/// # Usage
///
/// 1. Wrap your app shell (or preview subtree) in [`LoadingBarProvider`] or [`LoadingBar`].
/// 2. From a button handler, router hook, or resource callback, call `LoadingBarInjection::expect_context().start()` when work begins.
/// 3. Call `.finish()` when the operation succeeds — the bar animates to 100% and hides.
/// 4. Call `.error()` on failure — the bar completes with danger semantic color.
/// 5. The bar teleports to the top of the viewport; children stay in place — only the provider placement matters.
///
/// ```rust
/// use orbital_core_components::{LoadingBarProvider, Button};
/// use orbital_base_components::LoadingBarInjection;
///
/// view! {
///     <LoadingBarProvider>
///         <AppRoutes />
///     </LoadingBarProvider>
/// }
///
/// // Inside a child component:
/// LoadingBarInjection::expect_context().start();
/// // ... await fetch ...
/// LoadingBarInjection::expect_context().finish();
/// ```
///
/// # Best Practices
///
/// ## Do's
///
/// * Mount **one** provider per app shell — same pattern as [`crate::ToasterProvider`]
/// * Pair `.start()` with exactly one `.finish()` or `.error()` per operation
/// * Use for navigation and coarse-grained async; use [`crate::Spinner`] for inline waits
///
/// ## Don'ts
///
/// * Do not render the bar directly — it is created by the provider portal
/// * Do not call `.start()` without a matching completion — the bar can stall visually
/// * Do not nest multiple providers unless you intentionally scope separate surfaces
/// * Do not use for known-percent uploads — bind [`crate::ProgressBar`] instead
///
/// # Examples
///
/// ## Start loading
/// Wrap content in [`LoadingBarProvider`], then call `LoadingBarInjection::start()` from a child trigger.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, LoadingBarProvider};
/// use orbital_base_components::LoadingBarInjection;
/// view! {
///     <div data-testid="loading-bar-preview">
///         <LoadingBarProvider>
///             <Button on_click=Callback::new(|_| {
///                 LoadingBarInjection::expect_context().start();
///             })>"Start loading"</Button>
///         </LoadingBarProvider>
///     </div>
/// }
/// ```
///
/// ## Finish flow
/// Typical async pattern: `start()` when work begins, `finish()` when it completes successfully.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Flex, FlexGap, LoadingBarProvider};
/// use orbital_base_components::LoadingBarInjection;
/// view! {
///     <div data-testid="loading-bar-finish">
///         <LoadingBarProvider>
///             <Flex gap=FlexGap::Small>
///                 <Button on_click=Callback::new(|_| {
///                     LoadingBarInjection::expect_context().start();
///                 })>"Start"</Button>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     on_click=Callback::new(|_| {
///                         LoadingBarInjection::expect_context().finish();
///                     })
///                 >
///                     "Finish"
///                 </Button>
///             </Flex>
///         </LoadingBarProvider>
///     </div>
/// }
/// ```
///
/// ## Error flow
/// Call `error()` instead of `finish()` so the teleported bar completes with danger tokens.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, LoadingBarProvider};
/// use orbital_base_components::LoadingBarInjection;
/// view! {
///     <div data-testid="loading-bar-error">
///         <LoadingBarProvider>
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 on_click=Callback::new(|_| {
///                     LoadingBarInjection::expect_context().error();
///                 })
///             >
///                 "Trigger error"
///             </Button>
///         </LoadingBarProvider>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "loading-bar",
    preview_label = "Loading Bar",
    preview_icon = icondata::AiLoading3QuartersOutlined,
)]
/// Preview/catalog entry — thin alias over [`LoadingBarProvider`].
///
/// Prefer [`LoadingBarProvider`] in application code; this component exists so the catalog slug `loading-bar` registers provider usage and examples.
#[component]
#[allow(dead_code)]
pub fn LoadingBar(
    /// App subtree wrapped by the loading-bar provider — call `use_loading_bar()` in descendants.
    children: Children,
) -> impl IntoView {
    view! {
        <LoadingBarProvider>
            {children()}
        </LoadingBarProvider>
    }
}
