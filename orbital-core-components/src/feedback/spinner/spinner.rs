use leptos::prelude::*;
use orbital_base_components::{BaseSpinner, SpinnerSize};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::spinner_styles;

/// `Spinner` shows that work is in progress when you do not know how long it will take.
///
/// Always set `label` or provide child text so assistive tech announces the wait state. Pick a [`SpinnerSize`](orbital_base_components::SpinnerSize) to match surrounding typography. Indeterminate only — use [`ProgressBar`](crate::ProgressBar) or [`ProgressCircle`](crate::ProgressCircle) when you have a measurable completion value.
///
/// # When to use
///
/// - Inline waits inside panels, cards, or empty states
/// - Button loading states and [`Backdrop`](crate::Backdrop) overlays during async work
///
/// Prefer [`ProgressBar`](crate::ProgressBar) for known totals. Prefer [`LoadingBar`](crate::LoadingBar) for app-wide route transitions.
///
/// # API notes
///
/// - Indeterminate only — use [`ProgressBar`](crate::ProgressBar) or [`ProgressCircle`](crate::ProgressCircle) for known completion values.
/// - Size steps range from `Tiny` to `Huge` via [`SpinnerSize`](orbital_base_components::SpinnerSize).
/// - Always set `label` or provide child text for assistive technology.
///
/// # Best Practices
///
/// ## Do's
///
/// * Always provide `label` or visible child text — never ship an unlabeled spinner
/// * Match `size` to surrounding typography (`Tiny` inline, `Large` in empty states)
///
/// ## Don'ts
///
/// * Do not use for known completion percentages — bind [`ProgressBar`](crate::ProgressBar) or [`ProgressCircle`](crate::ProgressCircle)
/// * Do not use for global navigation chrome — use [`LoadingBar`](crate::LoadingBar)
///
/// # Examples
///
/// ## Default spinner
/// Indeterminate spinner for async waits when no progress percentage is known. Default medium size suits inline placement beside text or inside cards.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="spinner-preview">
///         <Spinner />
///     </div>
/// }
/// ```
///
/// ## With label
/// Always provide an accessible label describing what is loading. The label is linked via `aria-labelledby` so screen readers announce the wait state.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="spinner-labeled">
///         <Spinner label="Loading data..." />
///     </div>
/// }
/// ```
///
/// ## Large size
/// Large spinners draw attention in empty states or full-width loading panels.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::SpinnerSize;
/// view! {
///     <div data-testid="spinner-large">
///         <Spinner size=Signal::from(SpinnerSize::Large) />
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Compare size presets from tiny through huge when aligning spinners with surrounding typography.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::SpinnerSize;
/// view! {
///     <div data-testid="spinner-sizes" style="display: flex; gap: 16px; align-items: center;">
///         <div data-testid="spinner-size-tiny"><Spinner size=Signal::from(SpinnerSize::Tiny) /></div>
///         <div data-testid="spinner-size-huge"><Spinner size=Signal::from(SpinnerSize::Huge) /></div>
///     </div>
/// }
/// ```
///
/// ## Children label
/// Child text can serve as the accessible label when a separate `label` prop is not needed.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="spinner-children">
///         <Spinner>"Loading"</Spinner>
///     </div>
/// }
/// ```
///
/// ## Theme token
/// Spinner stroke colors inherit brand tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="spinner-theme">
///         <Spinner />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "spinner",
    preview_label = "Spinner",
    preview_icon = icondata::AiLoading3QuartersOutlined,
)]
#[component]
pub fn Spinner(
    /// Optional CSS class on the spinner root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Accessible label announced by assistive tech (`aria-labelledby`).
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Size preset from `ExtraTiny` through `Huge`.
    #[prop(optional, into)]
    size: Signal<SpinnerSize>,
    /// Alternative to `label` — child text rendered as the visible label.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-spinner", spinner_styles());

    view! {
        <BaseSpinner class=class label=label size=size>
            {children.map(|c| c())}
        </BaseSpinner>
    }
}
