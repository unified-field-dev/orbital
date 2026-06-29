use leptos::prelude::*;
use orbital_base_components::{BaseLabel, LabelSize, LabelWeight};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::label_styles;

/// Label renders accessible caption text for a control or section heading.
///
/// Set `html_for` to associate a standalone control, or use [`Field`](crate::Field) when the label, required marker, and validation message belong together.
/// Size and weight adjust typographic hierarchy without changing semantic meaning.
/// Orbital does not use floating labels — labels stay outside the control boundary.
///
/// # When to use
///
/// - Standalone labels adjacent to custom controls - Section headings in dense forms when Field layout is not needed - Required-field indicators beside control labels
///
/// # Usage
///
/// 1. Wrap label text in the component children. 2. Set `required=true` when the paired control must have a value. 3. Set `disabled=true` when the labeled control is unavailable. 4. Use `size` and `weight` for typographic hierarchy (small captions vs large section titles).
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer [`Field`](crate::Field) when the label pairs with one input and validation * Set `required` when the field is mandatory — do not rely on color alone * Match `disabled` to the associated control state
///
/// ## Don'ts
///
/// * Do not use as the only accessible name for a control — associate via Field or `for` * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default label
/// Baseline label typography for form fields and inline copy.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="label-preview">
///         <Label>"Display name"</Label>
///     </div>
/// }
/// ```
///
/// ## Required
/// Shows the required asterisk beside the label text.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="label-required">
///         <Label required=true>"Email"</Label>
///     </div>
/// }
/// ```
///
/// ## Sizes
/// Small, medium, and large type scales for captions, default fields, and section titles.
/// <!-- preview -->
/// ```rust
/// use crate::{Label, LabelSize};
/// view! {
///     <div data-testid="label-size-matrix">
///         <div data-testid="label-size-sm"><Label size=LabelSize::Small>"Small"</Label></div>
///         <div data-testid="label-size-md"><Label>"Medium"</Label></div>
///         <div data-testid="label-size-lg"><Label size=LabelSize::Large>"Large"</Label></div>
///     </div>
/// }
/// ```
///
/// ## Weights
/// Regular and semibold weights for emphasis without changing size.
/// <!-- preview -->
/// ```rust
/// use crate::{Label, LabelWeight};
/// view! {
///     <div data-testid="label-weight-matrix">
///         <Label weight=LabelWeight::Regular>"Regular"</Label>
///         <Label weight=LabelWeight::Semibold>"Semibold"</Label>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Muted label when the associated control is disabled.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="label-disabled">
///         <Label disabled=true>"Unavailable field"</Label>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "label",
    preview_label = "Label",
    preview_icon = icondata::AiTagOutlined,
)]
#[component]
pub fn Label(
    /// Extra CSS class names merged onto the root `<label>` element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Typographic size: small, medium (default), or large.
    #[prop(optional, into)]
    size: Signal<LabelSize>,
    /// Font weight: regular (default) or semibold.
    #[prop(optional, into)]
    weight: Signal<LabelWeight>,
    /// When true, renders a required-field asterisk after the label text.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// When true, applies disabled label styling.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Associates the label with a control by `id` (`for` attribute).
    #[prop(optional, into)]
    html_for: MaybeProp<String>,
    /// Label text or inline content.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-label", label_styles());

    let size_class = Signal::derive(move || format!("orbital-label--{}", size.get().as_str()));
    let weight_class = Signal::derive(move || format!("orbital-label--{}", weight.get().as_str()));

    let modifier_class = Signal::derive(move || {
        let mut parts = vec!["orbital-label".to_string()];
        if disabled.get() {
            parts.push("orbital-label--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <BaseLabel
            class=modifier_class
            size=size_class
            weight=weight_class
            label_size=size
            label_weight=weight
            required=Signal::from(false)
            disabled=disabled
            attr_for=html_for
        >
            {children()}
            {move || {
                required.get().then(|| view! {
                    <span aria-hidden="true" class="orbital-label__required">"*"</span>
                })
            }}
        </BaseLabel>
    }
}
