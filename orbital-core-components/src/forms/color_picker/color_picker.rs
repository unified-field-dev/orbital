use leptos::prelude::*;
use orbital_base_components::{BaseColorPicker, Color, ColorBind, FormBind, OptionBind};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::color_picker_styles;

/// Popover color editor for continuous opaque sRGB selection.
///
/// ColorPicker is a popover color editor for continuous sRGB selection: drag in the saturation-value plane and adjust hue on the slider. The bound value is an Orbital [`Color`](orbital_base_components::Color) struct — serialize with theme helpers, not raw hex strings. The trigger shows the selected color for quick confirmation. Alpha transparency is not supported yet; values are fully opaque. For predefined palette choices, use [`SwatchPicker`](crate::SwatchPicker) instead.
///
/// # When to use
///
/// - Theme accents and branding customization where users need any color in the sRGB space
/// - Form fields for accent or highlight colors in design tools
/// - Flows that need visual color selection rather than a fixed swatch list
///
/// # API notes
///
/// - Bind `OptionBind<Color>` or `FormBind<Color>` — serialize with theme helpers, not raw hex strings.
/// - Opaque sRGB only — alpha transparency is not supported yet.
/// - Popover editor with SV plane + hue slider for continuous color selection.
///
/// # ColorPicker vs SwatchPicker
///
/// | Need | Component |
/// |------|-----------|
/// | Continuous color, SV plane + hue slider | `ColorPicker` |
/// | Fixed brand or status palette | `SwatchPicker` |
///
/// # Usage
///
/// 1. Bind with `OptionBind<Color>` or required `FormBind<Color>` via [`ColorPickerBind`].
/// 2. Set `appearance.disabled` to prevent opening the panel.
/// 3. Wrap in [`Field`](crate::Field) for labeled form usage.
///
/// # Best Practices
///
/// ## Do's
///
/// * Show the selected color on the trigger for at-a-glance confirmation
/// * Persist colors as Orbital `Color` values in app state
///
/// ## Don'ts
///
/// * Do not use for non-color numeric ranges — prefer [`Slider`](crate::Slider)
/// * Do not use for fixed palette selection — prefer [`SwatchPicker`](crate::SwatchPicker)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## OptionBind signal
/// Optional value binding with `OptionBind<Color>`.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(None::<Color>);
/// view! {
///     <div data-testid="color-picker-preview">
///     <div data-testid="CP-01">
///         <ColorPicker bind=value />
///     </div>
///     </div>
/// }
/// ```
///
/// ## FormBind signal
/// Required value binding with `FormBind<Color>`.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(Color::default());
/// view! {
///     <div data-testid="CP-02">
///         <ColorPicker bind=value />
///     </div>
/// }
/// ```
///
/// ## Preselected value
/// Trigger reflects an initial non-default color.
/// <!-- preview -->
/// ```rust
/// use palette::Srgb;
/// let value = RwSignal::new(Color::from(Srgb::new(0.2, 0.5, 0.9)));
/// view! {
///     <div data-testid="CP-03">
///         <ColorPicker bind=value />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Trigger is visible but cannot open.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(Color::default());
/// view! {
///     <div data-testid="CP-04">
///         <ColorPicker bind=value appearance=ColorPickerAppearance::disabled() />
///     </div>
/// }
/// ```
///
/// ## Field composition
/// Typical labeled form usage.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// let value = RwSignal::new(Color::default());
/// view! {
///     <div data-testid="CP-05">
///         <Field label="Accent color">
///             <ColorPicker bind=value />
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "color-picker",
    preview_label = "Color Picker",
    preview_icon = icondata::AiBgColorsOutlined,
)]
#[component]
pub fn ColorPicker(
    /// Color value binding and optional field identity.
    #[prop(optional, into)]
    bind: ColorPickerBind,
    /// Disabled state and other visual options.
    #[prop(optional, into)]
    appearance: ColorPickerAppearance,
    /// Extra CSS class names merged onto the picker root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-color-picker", color_picker_styles());

    let ColorPickerBind { value, id, name } = bind;
    let ColorPickerAppearance { disabled } = appearance;

    view! {
        <BaseColorPicker
            class=class
            id=id
            name=name
            value=value
            disabled=disabled
        />
    }
}

#[derive(Default)]
pub struct ColorPickerBind {
    pub value: ColorBind,
    pub id: MaybeProp<String>,
    pub name: MaybeProp<String>,
}

impl ColorPickerBind {
    pub fn new(value: impl Into<ColorBind>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<ColorBind> for ColorPickerBind {
    fn from(value: ColorBind) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<Color>> for ColorPickerBind {
    fn from(value: OptionBind<Color>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<Color>> for ColorPickerBind {
    fn from(value: FormBind<Color>) -> Self {
        Self::new(value)
    }
}

impl From<RwSignal<Option<Color>>> for ColorPickerBind {
    fn from(value: RwSignal<Option<Color>>) -> Self {
        Self::new(value)
    }
}

impl From<RwSignal<Color>> for ColorPickerBind {
    fn from(value: RwSignal<Color>) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Copy)]
pub struct ColorPickerAppearance {
    pub disabled: Signal<bool>,
}

impl Default for ColorPickerAppearance {
    fn default() -> Self {
        Self {
            disabled: Signal::from(false),
        }
    }
}

impl ColorPickerAppearance {
    pub fn disabled() -> Self {
        Self {
            disabled: Signal::from(true),
        }
    }
}
