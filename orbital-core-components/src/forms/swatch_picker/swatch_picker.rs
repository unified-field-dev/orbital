use leptos::prelude::*;
use orbital_base_components::{BaseSwatchPicker, BaseSwatchPickerItem};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::swatch_picker_styles;
use super::types::{SwatchPickerLayout, SwatchPickerShape, SwatchPickerSize};

/// Keyboard-friendly radiogroup for choosing one color from a fixed palette.
///
/// SwatchPicker offers a compact, keyboard-friendly way to choose one color from a fixed palette — theme accents, status colors, or categorical highlights. Declare [`SwatchPickerItem`] children with `value`, `color`, and `label`, then control selection with `selected_value` or seed defaults with `default_selected_value`. For free-form color editing, use [`ColorPicker`](crate::ColorPicker) instead; SwatchPicker is for predefined choices only.
///
/// # When to use
///
/// - Pick theme, accent, or categorical colors from a fixed palette
/// - Compact color choice without a full color picker surface
/// - Keyboard-navigable palette selection in forms and settings
///
/// # SwatchPicker vs ColorPicker
///
/// | Need | Component |
/// |------|-----------|
/// | Fixed predefined palette | `SwatchPicker` |
/// | Continuous sRGB editing | `ColorPicker` |
///
/// # API notes
///
/// - Control selection with `selected_value` or seed defaults with `default_selected_value`.
/// - Set `layout`, `shape`, and `size` via [`SwatchPickerLayout`](super::types::SwatchPickerLayout), [`SwatchPickerShape`](super::types::SwatchPickerShape), and [`SwatchPickerSize`](super::types::SwatchPickerSize).
/// - Listen to `on_selection_change` for side effects such as theme updates.
///
/// # Usage
///
/// 1. Render [`SwatchPickerItem`] children with unique `value` ids and `color` hex values.
/// 2. Control selection with `selected_value` or seed uncontrolled state with `default_selected_value`.
/// 3. Listen to `on_selection_change` for side effects such as theme updates.
///
/// # Best Practices
///
/// ## Do's
///
/// * Offer enough contrast between swatches for recognition
/// * Label the picker for screen readers when used in forms
///
/// ## Don'ts
///
/// * Do not use swatch pickers for continuous color selection — prefer [`ColorPicker`](crate::ColorPicker)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default row
/// Horizontal palette with the first swatch selected.
/// <!-- preview -->
/// ```rust
/// use crate::{SwatchPicker, SwatchPickerItem};
/// view! {
///     <div data-testid="swatch-picker-preview">
///         <SwatchPicker default_selected_value="#2563EB">
///             <SwatchPickerItem value="#2563EB" color="#2563EB" label="Blue" />
///             <SwatchPickerItem value="#DB2777" color="#DB2777" label="Magenta" />
///             <SwatchPickerItem value="#059669" color="#059669" label="Green" />
///             <SwatchPickerItem value="#EA580C" color="#EA580C" label="Orange" />
///         </SwatchPicker>
///     </div>
/// }
/// ```
///
/// ## Grid layout
/// Wrapped grid for denser palettes.
/// <!-- preview -->
/// ```rust
/// use crate::{SwatchPicker, SwatchPickerItem, SwatchPickerLayout};
/// view! {
///     <div data-testid="swatch-picker-grid">
///         <SwatchPicker layout=SwatchPickerLayout::Grid default_selected_value="#7C3AED">
///             <SwatchPickerItem value="#2563EB" color="#2563EB" label="Blue" />
///             <SwatchPickerItem value="#DB2777" color="#DB2777" label="Magenta" />
///             <SwatchPickerItem value="#059669" color="#059669" label="Green" />
///             <SwatchPickerItem value="#7C3AED" color="#7C3AED" label="Purple" />
///             <SwatchPickerItem value="#EA580C" color="#EA580C" label="Orange" />
///             <SwatchPickerItem value="#CA8A04" color="#CA8A04" label="Yellow" />
///         </SwatchPicker>
///     </div>
/// }
/// ```
///
/// ## Controlled selection
/// Parent-owned selection updates when another swatch is clicked.
/// <!-- preview -->
/// ```rust
/// use crate::{SwatchPicker, SwatchPickerItem};
/// let selected = RwSignal::new("#2563EB".to_string());
/// view! {
///     <div data-testid="swatch-picker-controlled">
///         <SwatchPicker
///             selected_value=selected
///             on_selection_change=Callback::new(move |value: String| selected.set(value))
///         >
///             <SwatchPickerItem value="#2563EB" color="#2563EB" label="Blue" />
///             <SwatchPickerItem value="#DB2777" color="#DB2777" label="Magenta" />
///             <SwatchPickerItem value="#059669" color="#059669" label="Green" />
///         </SwatchPicker>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "swatch-picker",
    preview_label = "Swatch Picker",
    preview_icon = icondata::AiBgColorsOutlined,
)]
#[component]
pub fn SwatchPicker(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Layout mode (`row` or `grid`).
    #[prop(optional, into)]
    layout: Signal<SwatchPickerLayout>,
    /// Spacing between swatches in pixels.
    #[prop(optional, into)]
    spacing: Signal<Option<i32>>,
    /// Swatch shape (`rounded` or `square`).
    #[prop(optional, into)]
    shape: Signal<SwatchPickerShape>,
    /// Swatch size (`small` or `medium`).
    #[prop(optional, into)]
    size: Signal<SwatchPickerSize>,
    /// Controlled selected swatch id.
    #[prop(optional, into)]
    selected_value: MaybeProp<String>,
    /// Initial selected swatch id (uncontrolled).
    #[prop(optional, into)]
    default_selected_value: MaybeProp<String>,
    /// Fired when the selected swatch changes.
    #[prop(optional)]
    on_selection_change: Option<Callback<String>>,
    /// Swatch children.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-swatch-picker", swatch_picker_styles());

    let root_class = MaybeProp::derive(move || {
        let mut parts = vec!["orbital-swatch-picker".to_string()];
        parts.push(
            match layout.get() {
                SwatchPickerLayout::Row => "orbital-swatch-picker--row",
                SwatchPickerLayout::Grid => "orbital-swatch-picker--grid",
            }
            .to_string(),
        );
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <BaseSwatchPicker
            class=root_class
            layout=layout
            spacing=spacing
            shape=shape
            size=size
            selected_value=selected_value
            default_selected_value=default_selected_value
            nostrip:on_selection_change=on_selection_change
        >
            {children()}
        </BaseSwatchPicker>
    }
}

/// Single color swatch inside [`SwatchPicker`].
#[component]
pub fn SwatchPickerItem(
    /// Optional CSS class on the swatch button.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Unique swatch id used for selection (typically the color hex).
    #[prop(into)]
    value: String,
    /// CSS color value rendered as the swatch fill.
    #[prop(into)]
    color: String,
    /// Disables selection for this swatch.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Accessible name; defaults to `value`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
) -> impl IntoView {
    let picker = orbital_base_components::SwatchPickerInjection::expect_context();
    let value_stored = StoredValue::new(value);
    let is_selected = Memo::new({
        let picker = picker.clone();
        move |_| value_stored.with_value(|value| picker.is_selected(value))
    });
    let item_class = MaybeProp::derive(move || {
        let mut parts = vec!["orbital-swatch-picker__item".to_string()];
        parts.push(
            match picker.shape.get() {
                SwatchPickerShape::Rounded => "orbital-swatch-picker__item--rounded",
                SwatchPickerShape::Square => "orbital-swatch-picker__item--square",
            }
            .to_string(),
        );
        parts.push(
            match picker.size.get() {
                SwatchPickerSize::Small => "orbital-swatch-picker__item--small",
                SwatchPickerSize::Medium => "orbital-swatch-picker__item--medium",
            }
            .to_string(),
        );
        if is_selected.get() {
            parts.push("orbital-swatch-picker__item--selected".to_string());
        }
        if disabled.get() {
            parts.push("orbital-swatch-picker__item--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <BaseSwatchPickerItem
            class=item_class
            value=value_stored.get_value()
            color=color
            disabled=disabled
            label=label
        />
    }
}
