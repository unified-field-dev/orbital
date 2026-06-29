use leptos::prelude::*;
use orbital_base_components::BaseButtonGroup;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::button_group_styles;

/// Visually merges related buttons into one segmented control — shared outer borders, consistent spacing, optional vertical stacking.
///
/// Wrap two or more [`Button`](crate::Button) components; set `vertical=true` for narrow panels. Style each child button explicitly — the group only handles layout. Does not enforce single selection. Disable the whole group with `<fieldset disabled>` wrapping the group.
///
/// # When to use
///
/// - Save/Cancel or Approve/Reject action pairs - Toolbar segments (Cut/Copy/Paste) - Vertical action stacks in narrow panels
///
/// # Usage
///
/// 1. Wrap two or more [`Button`](crate::Button) components inside `ButtonGroup`. 2. Set `vertical=true` for stacked layouts. 3. Wrap preview examples in a native element with `data-testid` for E2E selectors.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep grouped actions logically related * Use consistent [`ButtonAppearance`](crate::ButtonAppearance) within a group
///
/// ## Don'ts
///
/// * Do not mix unrelated primary actions in one group * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default group
/// Horizontal primary and secondary pair with merged borders for related form actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, ButtonGroup};
/// view! {
///     <div data-testid="button-group-preview">
///         <ButtonGroup>
///             <Button appearance=ButtonAppearance::Primary>"Save"</Button>
///             <Button appearance=ButtonAppearance::Secondary>"Cancel"</Button>
///         </ButtonGroup>
///     </div>
/// }
/// ```
///
/// ## Vertical stack
/// Stacks buttons vertically for narrow panels, sidebars, or mobile layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, ButtonGroup};
/// view! {
///     <div data-testid="button-group-vertical">
///         <ButtonGroup vertical=true>
///             <Button appearance=ButtonAppearance::Primary>"Top"</Button>
///             <Button appearance=ButtonAppearance::Secondary>"Middle"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Bottom"</Button>
///         </ButtonGroup>
///     </div>
/// }
/// ```
///
/// ## Subtle toolbar actions
/// Low-emphasis segmented toolbar for Cut, Copy, Paste, and similar utility actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, ButtonGroup};
/// view! {
///     <div data-testid="button-group-toolbar">
///         <ButtonGroup>
///             <Button appearance=ButtonAppearance::Subtle>"Cut"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Copy"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Paste"</Button>
///         </ButtonGroup>
///     </div>
/// }
/// ```
///
/// ## Icon actions
/// Icon-only buttons grouped into one compact toolbar segment with shared borders.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, ButtonGroup};
/// view! {
///     <div data-testid="button-group-icons">
///         <ButtonGroup>
///             <Button appearance=ButtonAppearance::Secondary icon=icondata::AiPlusOutlined />
///             <Button appearance=ButtonAppearance::Secondary icon=icondata::AiEditOutlined />
///             <Button appearance=ButtonAppearance::Secondary icon=icondata::AiDeleteOutlined />
///         </ButtonGroup>
///     </div>
/// }
/// ```
///
/// ## Disabled in fieldset
/// Entire group disabled via a native `<fieldset>` when the workflow is temporarily unavailable.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, ButtonGroup};
/// view! {
///     <div data-testid="button-group-disabled">
///         <fieldset disabled>
///             <ButtonGroup>
///                 <Button appearance=ButtonAppearance::Primary>"Approve"</Button>
///                 <Button appearance=ButtonAppearance::Secondary>"Reject"</Button>
///             </ButtonGroup>
///         </fieldset>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "button-group",
    preview_label = "Button Group",
    preview_icon = icondata::AiBorderOutlined,
)]
#[component]
pub fn ButtonGroup(
    /// Extra CSS class names on the group container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// When true, stacks buttons vertically instead of horizontally.
    #[prop(optional)]
    vertical: bool,
    /// Buttons or other controls in the group.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-button-group", button_group_styles());
    view! {
        <BaseButtonGroup class=class vertical=vertical>
            {children()}
        </BaseButtonGroup>
    }
}
