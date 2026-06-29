use leptos::prelude::*;
use orbital_base_components::{BaseToolbar, ToolbarSize};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::{Button, ButtonAppearance};

use super::styles::toolbar_styles;

/// `Toolbar` lays out a row or column of commands for an editor, viewer, or dialog header.
/// Set `vertical` for side tool strips; nest [`ToolbarButton`](crate::ToolbarButton),
/// [`ButtonGroup`](crate::ButtonGroup), [`Divider`](crate::Divider), and [`Overflow`](crate::Overflow)
/// so actions collapse gracefully on narrow widths. Give the toolbar root an `aria-label`.
/// For exclusive modes, compose [`ToggleButton`](crate::ToggleButton) rows instead of a dedicated radio toolbar.
///
/// # Examples
///
/// ## Default toolbar
/// Horizontal command strip with common actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Toolbar, Button, ButtonAppearance};
/// view! {
///     <div data-testid="toolbar-preview">
///         <Toolbar>
///             <Button appearance=ButtonAppearance::Subtle>"New"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Open"</Button>
///             <Button appearance=ButtonAppearance::Primary>"Save"</Button>
///         </Toolbar>
///     </div>
/// }
/// ```
///
/// ## Vertical layout
/// Column toolbar for side panels.
/// <!-- preview -->
/// ```rust
/// use crate::{Toolbar, Button, ButtonAppearance};
/// view! {
///     <div data-testid="toolbar-vertical">
///         <Toolbar vertical=true>
///             <Button appearance=ButtonAppearance::Subtle>"Bold"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Italic"</Button>
///         </Toolbar>
///     </div>
/// }
/// ```
///
/// ## With overflow
/// Responsive toolbar using overflow for narrow widths.
/// <!-- preview -->
/// ```rust
/// use crate::{Toolbar, Button, ButtonAppearance, Overflow, OverflowMenuItems, MenuItem};
/// view! {
///     <div data-testid="toolbar-overflow" style="width: 180px;">
///         <Toolbar>
///             <Overflow>
///                 <Button appearance=ButtonAppearance::Subtle>"Undo"</Button>
///                 <Button appearance=ButtonAppearance::Subtle>"Redo"</Button>
///                 <Button appearance=ButtonAppearance::Subtle>"Cut"</Button>
///                 <OverflowMenuItems slot>
///                     <MenuItem value="redo".to_string()>"Redo"</MenuItem>
///                     <MenuItem value="cut".to_string()>"Cut"</MenuItem>
///                 </OverflowMenuItems>
///             </Overflow>
///         </Toolbar>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "toolbar",
    preview_label = "Toolbar",
    preview_icon = icondata::AiMenuOutlined,
)]
#[component]
pub fn Toolbar(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Renders a vertical toolbar when true.
    #[prop(optional)]
    vertical: bool,
    /// Toolbar density preset.
    #[prop(optional, into)]
    size: Signal<ToolbarSize>,
    /// Toolbar buttons and groups.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-toolbar", toolbar_styles());

    view! {
        <BaseToolbar class=class vertical=vertical size=size>
            {children()}
        </BaseToolbar>
    }
}

/// Toolbar-styled button alias with compact defaults.
#[component]
pub fn ToolbarButton(
    /// Optional CSS class.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Button label.
    children: Children,
) -> impl IntoView {
    view! {
        <Button class=class appearance=ButtonAppearance::Subtle>
            {children()}
        </Button>
    }
}
