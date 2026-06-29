use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::Icon;

use super::styles::back_to_top_styles;
use super::types::BackToTopLabel;
use crate::floating_button::floating_button_styles;
use orbital_base_components::FloatingButtonVariant;
use orbital_motion::MotionSlot;

/// `BackToTop` is a fixed floating control that appears after the user scrolls past
/// `visibility_height`, then scrolls the nearest scrollport back to the top on click.
/// Place one per long page; use [`BackToTopLabel`] for an extended label.
/// For primary page actions, use [`FloatingButton`](crate::FloatingButton) instead.
///
/// # When to use
///
/// - Long pages where users need a quick return-to-top affordance
///
/// # Usage
///
/// 1. Place `BackToTop` anywhere in the page tree. 2. Tune `visibility_height` to control when the button appears. 3. Nest [`BackToTopLabel`] in the `slot` for a labeled extended variant.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep the control unobtrusive — default offset values work for most pages * Use one BackToTop per page
///
/// ## Don'ts
///
/// * Do not nest multiple BackToTop controls expecting independent scroll thresholds
///
/// # Examples
///
/// ## Default
/// Scroll the page to reveal the control; click returns to the top.
/// <!-- preview -->
/// ```rust
/// use crate::BackToTop;
/// view! {
///     <div data-testid="back-to-top-preview">
///         <div style="min-height: 500px">"Scroll the page to reveal the back-to-top control."</div>
///         <BackToTop right=120 bottom=40 testid="back-to-top-button" />
///     </div>
/// }
/// ```
///
/// ## Change visibility height
/// Higher `visibility_height` delays appearance until the user scrolls further.
/// <!-- preview -->
/// ```rust
/// use crate::{BackToTop, BackToTopLabel};
/// view! {
///     <div data-testid="back-to-top-visibility">
///         <div style="min-height: 500px">"Scroll further before this control appears."</div>
///         <BackToTop right=80 bottom=180 visibility_height=400 testid="back-to-top-visibility-button">
///             <BackToTopLabel slot>"Change visibility height"</BackToTopLabel>
///         </BackToTop>
///     </div>
/// }
/// ```
///
/// ## Change position
/// Adjust `right` and `bottom` to reposition the floating control.
/// <!-- preview -->
/// ```rust
/// use crate::{BackToTop, BackToTopLabel};
/// view! {
///     <div data-testid="back-to-top-position">
///         <div style="min-height: 500px">"Scroll the page — this instance sits between the other examples."</div>
///         <BackToTop right=40 bottom=110 testid="back-to-top-position-button">
///             <BackToTopLabel slot>"Change position"</BackToTopLabel>
///         </BackToTop>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "back-to-top",
    preview_label = "Back To Top",
    preview_icon = icondata::AiVerticalAlignTopOutlined,
)]
#[component]
pub fn BackToTop(
    /// Optional CSS class on the floating button.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Distance from the right edge of the viewport in pixels.
    #[prop(default = 40.into(), into)]
    right: Signal<i32>,
    /// Distance from the bottom edge of the viewport in pixels.
    #[prop(default = 40.into(), into)]
    bottom: Signal<i32>,
    /// Scrollport offset before the control becomes visible.
    #[prop(default = 180.into(), into)]
    visibility_height: Signal<i32>,
    /// Extended FAB label content.
    #[prop(optional)]
    back_to_top_label: Option<BackToTopLabel>,
    /// Optional content replacing the default icon and label.
    #[prop(optional)]
    children: Option<Children>,
    /// Optional enter/exit motion override.
    #[prop(optional)]
    motion: MotionSlot,
    /// Optional `data-testid` on the floating button.
    #[prop(optional, into)]
    testid: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-floating-button", floating_button_styles());
    inject_style("orbital-back-to-top", back_to_top_styles());

    let variant = Signal::stored(if back_to_top_label.is_some() {
        FloatingButtonVariant::Extended
    } else {
        FloatingButtonVariant::Rounded
    });
    let aria_label = MaybeProp::from("Back to top".to_string());

    match (children, back_to_top_label) {
        (Some(children), _) => {
            view! {
                <orbital_base_components::BaseBackToTop
                    class=class
                    right=right
                    bottom=bottom
                    visibility_height=visibility_height
                    variant=variant
                    aria_label=aria_label
                    testid=testid.clone()
                    motion=motion
                >
                    {children()}
                </orbital_base_components::BaseBackToTop>
            }
        }
        (None, Some(label)) => {
            view! {
                <orbital_base_components::BaseBackToTop
                    class=class
                    right=right
                    bottom=bottom
                    visibility_height=visibility_height
                    variant=variant
                    aria_label=aria_label
                    testid=testid.clone()
                    motion=motion
                >
                    <span class="orbital-floating-button__icon">
                        <Icon icon=icondata::AiVerticalAlignTopOutlined />
                    </span>
                    {(label.children)()}
                </orbital_base_components::BaseBackToTop>
            }
        }
        (None, None) => {
            view! {
                <orbital_base_components::BaseBackToTop
                    class=class
                    right=right
                    bottom=bottom
                    visibility_height=visibility_height
                    variant=variant
                    aria_label=aria_label
                    testid=testid
                    motion=motion
                >
                    <span class="orbital-floating-button__icon">
                        <Icon icon=icondata::AiVerticalAlignTopOutlined />
                    </span>
                </orbital_base_components::BaseBackToTop>
            }
        }
    }
}
