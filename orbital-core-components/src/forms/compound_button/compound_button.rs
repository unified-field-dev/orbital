use icondata_core::Icon;
use leptos::prelude::*;
use orbital_base_components::{
    ButtonAppearance, ButtonShape, ButtonSize, CompoundButtonIconPosition,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::{Button, Icon};

fn compound_icon(icon: Icon) -> impl IntoView {
    view! {
        <span class="orbital-button__icon">
            <Icon icon=icon width="1em" height="1em" />
        </span>
    }
}

use super::styles::compound_button_styles;

/// Shows a primary label with a shorter supporting line — useful when the action needs context (for example, "Create" + "New workspace").
///
/// Both lines are announced as the control name, so keep each concise (under ~40 characters). Optional icons sit before or after the text block via [`CompoundButtonIconPosition`].
///
/// # When to use
///
/// - Actions that need a short subtitle for clarity without a separate tooltip - Workspace creation, upload destinations, or deployment targets
///
/// # Usage
///
/// 1. Put the main label in `children` and the supporting line in `secondary_content`. 2. Set `icon_position` when an icon should lead or trail the two-line block. 3. Prefer [`Link`](crate::Link) for navigation and [`MenuButton`](crate::MenuButton) for option lists.
///
/// # Examples
///
/// ## Default compound button
/// Primary label with a secondary description line.
/// <!-- preview -->
/// ```rust
/// use crate::{CompoundButton, CompoundButtonIconPosition};
/// use icondata::AiPlusOutlined;
/// view! {
///     <div data-testid="compound-button-preview">
///         <CompoundButton
///             secondary_content="Create a new workspace"
///             icon=AiPlusOutlined
///             icon_position=CompoundButtonIconPosition::Before
///         >
///             "Create"
///         </CompoundButton>
///     </div>
/// }
/// ```
///
/// ## Icon after label
/// Icon trailing the two-line content.
/// <!-- preview -->
/// ```rust
/// use crate::{CompoundButton, CompoundButtonIconPosition};
/// use icondata::AiCloudUploadOutlined;
/// view! {
///     <div data-testid="compound-button-icon-after">
///         <CompoundButton
///             secondary_content="Upload to cloud storage"
///             icon=AiCloudUploadOutlined
///             icon_position=CompoundButtonIconPosition::After
///         >
///             "Upload"
///         </CompoundButton>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "compound-button",
    preview_label = "Compound Button",
    preview_icon = icondata::AiBlockOutlined,
)]
#[component]
pub fn CompoundButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] appearance: Signal<ButtonAppearance>,
    #[prop(optional, into)] shape: Signal<ButtonShape>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] disabled_focusable: Signal<bool>,
    #[prop(optional, into)] icon: MaybeProp<Icon>,
    /// Whether the icon leads or trails the two-line label block.
    #[prop(optional, into)]
    icon_position: Signal<CompoundButtonIconPosition>,
    /// Supporting description line; both lines form the accessible name — keep concise.
    #[prop(optional, into)]
    secondary_content: MaybeProp<String>,
    #[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-compound-button", compound_button_styles());

    let modifier_class = Signal::derive(move || {
        let mut parts = vec![
            "orbital-button".to_string(),
            "orbital-compound-button".to_string(),
        ];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <Button
            class=modifier_class
            appearance=appearance
            shape=shape
            size=size
            disabled=disabled
            disabled_focusable=disabled_focusable
            nostrip:on_click=on_click
        >
            <span class="orbital-compound-button__content">
                {move || {
                    (icon_position.get() == CompoundButtonIconPosition::Before)
                        .then(|| icon.get().map(compound_icon))
                        .flatten()
                }}
                <span class="orbital-compound-button__text">
                    <span class="orbital-compound-button__primary">{children()}</span>
                    {move || secondary_content.get().map(|text| view! {
                        <span class="orbital-compound-button__secondary">{text}</span>
                    })}
                </span>
                {move || {
                    (icon_position.get() == CompoundButtonIconPosition::After)
                        .then(|| icon.get().map(compound_icon))
                        .flatten()
                }}
            </span>
        </Button>
    }
}
