use leptos::prelude::*;
use orbital_base_components::{BaseTooltip, OverlayAppearance};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::tooltip_styles;
use super::types::{TooltipAppearance, TooltipConfig, TooltipPosition};
use crate::overlay::{overlay_surface_class, FloatingPanel};
use crate::MaterialElevation;

/// `Tooltip` shows a brief hint when the wrapped control is hovered or focused — ideal for
/// icon-only buttons and truncated labels. Set `content` to a short phrase; pick `position`
/// and `appearance` for contrast. Always give icon-only triggers an accessible name too;
/// never put required information only in a tooltip.
///
/// # When to use
///
/// - Icon-only buttons, truncated labels, clarifying a control with minimal visible text
/// - One short phrase on hover or focus — not interactive content
/// - Rich panels, pickers, or action lists — use [`Popover`](crate::Popover) or [`Menu`](crate::Menu)
///
/// # Overlay surfaces
///
/// - **Brief non-interactive hint** — `Tooltip` (this component)
/// - **Floating panel with content or inputs** — [`Popover`](crate::Popover)
/// - **List of actions from a trigger** — [`Menu`](crate::Menu)
/// - **Block the page or trap focus** — [`Dialog`](crate::Dialog)
///
/// # Usage
///
/// 1. Wrap the trigger (e.g. [`crate::Button`]) as children.
/// 2. Set `content` to the hint string (or `Signal` for dynamic text).
/// 3. Optional: `position` (`Top` default), `appearance` (`Normal` / `Inverted`).
/// 4. On disabled buttons that must stay focusable, set `disabled_focusable=true` on the
///    wrapped [`Button`](crate::Button) so keyboard users can reach the tooltip.
///
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <Tooltip content="Save changes" position=TooltipPosition::Bottom>
///         <Button appearance=ButtonAppearance::Subtle icon=icondata::AiSaveOutlined />
///     </Tooltip>
/// }
/// ```
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep content to one short phrase * Use `TooltipAppearance::Inverted` on dark surfaces when contrast needs a boost * Provide `aria-label` on icon-only triggers in addition to the tooltip
///
/// ## Don'ts
///
/// * Do not hide essential information only in a tooltip * Do not use tooltips for interactive content — use [`Popover`](crate::Popover) or [`Dialog`](crate::Dialog)
///
/// # TooltipPosition variants
///
/// | Variant | Placement |
/// |---------|-----------|
/// | `Top` (default) | Above trigger, centered |
/// | `Bottom` | Below trigger, centered |
/// | `Left` | Left of trigger, centered |
/// | `Right` | Right of trigger, centered |
/// | `TopStart` / `TopEnd` | Above, aligned to start/end edge |
/// | `BottomStart` / `BottomEnd` | Below, aligned to start/end edge |
/// | `LeftStart` / `LeftEnd` | Left, aligned to start/end edge |
/// | `RightStart` / `RightEnd` | Right, aligned to start/end edge |
///
/// Optional `show_delay_ms` / `hide_delay_ms` control hover timing (defaults: `0` / `100`).
///
/// # Examples
///
/// ## Basic tooltip (top)
/// Short hint on hover or focus for controls with minimal visible labeling. Top is the default when space allows above the trigger.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-preview">
///         <Tooltip content="More information">
///             <Button appearance=ButtonAppearance::Subtle>"Hover me"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Inverted appearance
/// Dark tooltip body on light triggers or when extra contrast is needed on tinted surfaces.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="tooltip-inverted">
///         <Tooltip content="Dark tooltip" appearance=TooltipAppearance::Inverted>
///             <Button>"Target"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Bottom placement
/// Opens below the trigger when the control sits under headers or near the top viewport edge.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-bottom">
///         <Tooltip content="Below" position=TooltipPosition::Bottom>
///             <Button appearance=ButtonAppearance::Subtle>"Bottom"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Left placement
/// Positions the hint to the left of icon-only toolbar buttons or triggers on the right side of the layout.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-left">
///         <Tooltip content="To the left" position=TooltipPosition::Left>
///             <Button appearance=ButtonAppearance::Subtle>"Left"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Right placement
/// Positions the hint to the right when the trigger sits on the left margin or left-aligned controls.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-right">
///         <Tooltip content="To the right" position=TooltipPosition::Right>
///             <Button appearance=ButtonAppearance::Subtle>"Right"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Top start (aligned to start edge)
/// Aligns the tooltip with the start edge of wide triggers—multi-word buttons or split controls.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="tooltip-top-start">
///         <Tooltip content="Top start" position=TooltipPosition::TopStart>
///             <Button>"Top start"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Bottom end
/// Aligns with the end edge when the trigger spans most of a row or sits flush to the trailing margin.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="tooltip-bottom-end">
///         <Tooltip content="Bottom end" position=TooltipPosition::BottomEnd>
///             <Button>"Bottom end"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Disabled focusable button
/// Wrap a disabled button with `disabled_focusable=true` so keyboard users can focus it and read the tooltip.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-disabled-focusable">
///         <Tooltip content="Save is unavailable until you fix validation errors">
///             <Button appearance=ButtonAppearance::Primary disabled=true disabled_focusable=true>
///                 "Save"
///             </Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Show delay
/// Tooltip opens only after hovering for the configured show delay.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-show-delay">
///         <Tooltip content="Delayed hint" show_delay_ms=400>
///             <Button appearance=ButtonAppearance::Subtle>"Hover and wait"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
///
/// ## Theme token
/// Tooltip surface inherits neutral background tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="tooltip-theme">
///         <Tooltip content="Themed tooltip">
///             <Button appearance=ButtonAppearance::Subtle>"Theme"</Button>
///         </Tooltip>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "tooltip",
    preview_label = "Tooltip",
    preview_icon = icondata::AiQuestionCircleOutlined,
)]
#[component]
pub fn Tooltip<T>(
    /// Hint text; omit only when using child-driven content (rare).
    #[prop(optional, into)]
    content: Option<Signal<String>>,
    /// Optional CSS class on the tooltip surface.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Placement relative to the trigger (`Top` default).
    #[prop(optional)]
    position: TooltipPosition,
    /// Visual variant: `Normal` (default) or `Inverted`.
    #[prop(optional, into)]
    appearance: Signal<TooltipAppearance>,
    /// Milliseconds before the tooltip opens on hover (`0` = immediate).
    #[prop(default = 0u64)]
    show_delay_ms: u64,
    /// Milliseconds before the tooltip closes after pointer leave.
    #[prop(default = 100u64)]
    hide_delay_ms: u64,
    /// Portal mount target. When omitted, the panel portals to `document.body`.
    #[prop(default = None)]
    mount: Option<NodeRef<leptos::html::Div>>,
    /// Trigger element (button, icon wrapper, etc.).
    children: leptos::children::TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    inject_style("orbital-tooltip", tooltip_styles());

    let config = TooltipConfig {
        position,
        appearance,
    };

    let appearance_val: OverlayAppearance = config.appearance.get_untracked().into();
    let surface_class = Signal::derive(move || {
        overlay_surface_class("orbital-tooltip-content", appearance_val, None)
    });

    match content {
        Some(content) => view! {
            <BaseTooltip
                placement=config.position.into()
                appearance=appearance_val
                show_delay_ms=show_delay_ms
                hide_delay_ms=hide_delay_ms
                class=class
                mount=mount
                trigger=children
            >
                <FloatingPanel
                    class=surface_class
                    body_class="orbital-tooltip-body"
                    elevation=MaterialElevation::Raised
                    role="tooltip"
                >
                    {move || content.get()}
                </FloatingPanel>
            </BaseTooltip>
        }
        .into_any(),
        None => view! {
            <BaseTooltip
                placement=config.position.into()
                appearance=appearance_val
                show_delay_ms=show_delay_ms
                hide_delay_ms=hide_delay_ms
                class=class
                mount=mount
                trigger=children
            >
                <FloatingPanel
                    class=surface_class
                    body_class="orbital-tooltip-body"
                    elevation=MaterialElevation::Raised
                    role="tooltip"
                >
                    ""
                </FloatingPanel>
            </BaseTooltip>
        }
        .into_any(),
    }
}
