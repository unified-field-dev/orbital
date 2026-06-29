//! Demo box for layout and shell previews.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Dashed-border layout preview surface for shell demos and unimplemented primitive stubs.
///
/// Use inside layout component previews to show where main content would render. Gap primitive stubs pass `label=placeholder_label("ComponentName")` for consistent todo labels.
///
/// # When to use
///
/// - Component preview pages for layout components - Storybook-style demos where real page content is out of scope - Placeholder stubs for primitives not yet implemented
///
/// # Examples
///
/// ## Default
/// Dashed-border box filling a fixed-height region where main content would render. Use in shell and layout previews when real page content is out of scope.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="demo-box-preview" style="height: 260px;">
///         <DemoBox fill=true />
///     </div>
/// }
/// ```
///
/// ## With Label
/// Same dashed surface with a custom label identifying the content region. Use when multiple demo areas need distinct labels in a layout preview.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="demo-box-labeled" style="height: 260px;">
///         <DemoBox fill=true label="Main content area" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Integrations",
    preview_slug = "demo-box",
    preview_label = "Demo Box",
    preview_icon = icondata::AiAppstoreOutlined,
)]
#[component]
pub fn DemoBox(
    /// Optional label text to display when no children are provided
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// When true, fills the parent container with width and height 100%
    #[prop(optional, default = false)]
    fill: bool,
    /// CSS width value
    #[prop(optional, into)]
    width: MaybeProp<String>,
    /// CSS height value
    #[prop(optional, into)]
    height: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <orbital_base_components::DemoBox label=label fill=fill width=width height=height />
    }
}
