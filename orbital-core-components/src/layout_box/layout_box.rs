use leptos::{either::Either, prelude::*};
use orbital_base_components::{BorderRadius, Shadow, SpacingInset, StrokeWidth, ThemeColor};
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

/// Generic layout wrapper for spacing, sizing, and surface styling on a single node.
///
/// `Box` is a theme-aware container for one element—like a `<div>` with optional token-based padding, margin, width, and extra CSS. Use it when a single wrapper needs spacing or sizing without distributing gaps between siblings. `foreground` sets theme text color — not the CSS `color` shorthand prop. For even gaps between children, prefer [`Stack`](crate::Stack) or [`Flex`](crate::Flex).
///
/// # When to use
///
/// - One wrapper needs padding, margin, max-width, or surface styling - A card-like region without the full [`Material`](crate::Material) API - Custom layout chrome where [`Stack`](crate::Stack) / [`Flex`](crate::Flex) are too opinionated
///
/// # Usage
///
/// 1. Put content in the default slot. 2. Set typed `padding`, `margin`, `background`, `radius`, and related token props. 3. Pass additional rules through `style` when you need escape-hatch CSS. 4. Add utility classes through `class` when needed.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`SpacingInset`] for `padding` / `margin` * Use [`ThemeColor`], [`BorderRadius`], and [`Shadow`] for surface styling * Pair `Box` with typography presets ([`Body1`](crate::Body1)) for readable demo and app copy
///
/// ## Don'ts
///
/// * Do not use `Box` to space multiple siblings — prefer [`Stack`](crate::Stack) or [`Flex`](crate::Flex) * Avoid hard-coded hex colors; use [`ThemeColor`] tokens
///
/// # Examples
///
/// ## Default
/// A bordered box with sample text—the quickest way to see the container bounds.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::Box;
/// use orbital_base_components::{BorderRadius, SpacingInset, StrokeWidth, ThemeColor};
/// view! {
///     <div data-testid="box-preview">
///         <Box
///             padding=SpacingInset::all_m()
///             background=ThemeColor::NeutralBackground1
///             foreground=ThemeColor::NeutralForeground1
///             border_color=ThemeColor::NeutralStroke1
///             radius=BorderRadius::Medium
///             border_style="dashed"
///             border_width=StrokeWidth::Thin
///         >
///             "This Box wraps content with theme-aware spacing and stroke tokens."
///         </Box>
///     </div>
/// }
/// ```
///
/// ## Padding
/// The `padding` prop adds inner space without extra wrapper markup.
/// <!-- preview -->
/// ```rust
/// use crate::Box;
/// use orbital_base_components::{BorderRadius, SpacingInset, StrokeWidth, ThemeColor};
/// view! {
///     <div data-testid="box-padding">
///         <Box
///             padding=SpacingInset::all_l()
///             foreground=ThemeColor::NeutralForeground1
///             border_color=ThemeColor::NeutralStroke1
///             radius=BorderRadius::Medium
///             border_style="dashed"
///             border_width=StrokeWidth::Thin
///         >
///             "Extra padding keeps content away from the dashed border."
///         </Box>
///     </div>
/// }
/// ```
///
/// ## Margin
/// The `margin` prop offsets the box from surrounding layout—visible against a tinted backdrop.
/// <!-- preview -->
/// ```rust
/// use crate::Box;
/// use orbital_base_components::{BorderRadius, SpacingInset, StrokeWidth, ThemeColor};
/// view! {
///     <div data-testid="box-margin" style="padding: var(--orb-space-inline-sm); background: var(--orb-color-surface-subtle); border-radius: var(--orb-radius-md);">
///         <Box
///             margin=SpacingInset::all_l()
///             padding=SpacingInset::all_m()
///             foreground=ThemeColor::NeutralForeground1
///             border_color=ThemeColor::NeutralStroke1
///             radius=BorderRadius::Medium
///             border_style="dashed"
///             border_width=StrokeWidth::Thin
///         >
///             "Margin pushes this box away from the outer frame."
///         </Box>
///     </div>
/// }
/// ```
///
/// ## Width
/// Constrain width on forms, side panels, and compact callouts.
/// <!-- preview -->
/// ```rust
/// use crate::Box;
/// use orbital_base_components::{BorderRadius, SpacingInset, StrokeWidth, ThemeColor};
/// view! {
///     <div data-testid="box-width">
///         <Box
///             width="280px"
///             padding=SpacingInset::all_m()
///             foreground=ThemeColor::NeutralForeground1
///             border_color=ThemeColor::NeutralStroke1
///             radius=BorderRadius::Medium
///             border_style="dashed"
///             border_width=StrokeWidth::Thin
///         >
///             "A fixed width keeps this callout readable in wide layouts."
///         </Box>
///     </div>
/// }
/// ```
///
/// ## Surface styling
/// Combine typed surface props for a subtle raised panel.
/// <!-- preview -->
/// ```rust
/// use crate::Box;
/// use orbital_base_components::{BorderRadius, Shadow, SpacingInset, StrokeWidth, ThemeColor};
/// view! {
///     <div data-testid="box-surface">
///         <Box
///             padding=SpacingInset::all_l()
///             background=ThemeColor::NeutralBackground1
///             foreground=ThemeColor::NeutralForeground1
///             border_color=ThemeColor::NeutralStroke1
///             radius=BorderRadius::Medium
///             border_width=StrokeWidth::Thin
///             shadow=Shadow::Shadow4
///         >
///             "Surface tokens give the box a resting elevation on the page canvas."
///         </Box>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "box",
    preview_label = "Box",
    preview_icon = icondata::AiBorderOutlined,
)]
#[component]
pub fn Box(
    /// Extra CSS class names merged onto the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Additional inline CSS declarations merged after typed token props.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Theme-aware padding using Orbital spacing tokens.
    #[prop(optional, into)]
    padding: MaybeProp<SpacingInset>,
    /// Theme-aware margin using Orbital spacing tokens.
    #[prop(optional, into)]
    margin: MaybeProp<SpacingInset>,
    /// CSS width value (for example `280px`, `100%`, or `min(100%, 480px)`).
    #[prop(optional, into)]
    width: MaybeProp<String>,
    /// Background fill from the active theme palette.
    #[prop(optional, into)]
    background: MaybeProp<ThemeColor>,
    /// Text color from the active theme palette.
    #[prop(optional, into)]
    foreground: MaybeProp<ThemeColor>,
    /// Border color from the active theme palette.
    #[prop(optional, into)]
    border_color: MaybeProp<ThemeColor>,
    /// Border radius token.
    #[prop(optional, into)]
    radius: MaybeProp<BorderRadius>,
    /// Border width token (defaults to thin when `border_color` is set).
    #[prop(optional, into)]
    border_width: MaybeProp<StrokeWidth>,
    /// Border style (for example `solid` or `dashed`).
    #[prop(optional, into)]
    border_style: MaybeProp<String>,
    /// Elevation shadow token.
    #[prop(optional, into)]
    shadow: MaybeProp<Shadow>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .BoxRoot {
            box-sizing: border-box;
        }
    };

    let merged_style = move || {
        let mut parts = Vec::new();
        if let Some(p) = padding.get() {
            parts.push(p.padding_css().trim_end_matches(';').to_string());
        }
        if let Some(m) = margin.get() {
            parts.push(m.margin_css().trim_end_matches(';').to_string());
        }
        if let Some(w) = width.get() {
            if !w.is_empty() {
                parts.push(format!("width: {w}"));
            }
        }
        if let Some(bg) = background.get() {
            parts.push(format!("background: {}", bg.css_var()));
        }
        if let Some(fg) = foreground.get() {
            parts.push(format!("color: {}", fg.css_var()));
        }
        if let Some(bc) = border_color.get() {
            let width = border_width.get().unwrap_or(StrokeWidth::Thin).css_var();
            let style = border_style.get().unwrap_or_else(|| "solid".into());
            parts.push(format!("border: {width} {style} {}", bc.css_var()));
        }
        if let Some(r) = radius.get() {
            parts.push(format!("border-radius: {}", r.css_var()));
        }
        if let Some(s) = shadow.get() {
            parts.push(format!("box-shadow: {}", s.css_var()));
        }
        if let Some(extra) = style.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join("; ")
    };

    view! {
        <style>{style_sheet}</style>
        <div class=move || {
            match class.get() {
                Some(extra) if !extra.trim().is_empty() => {
                    format!("{} {}", class_names.box_root, extra)
                }
                _ => class_names.box_root.to_string(),
            }
        } style=merged_style>
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </div>
    }
}
