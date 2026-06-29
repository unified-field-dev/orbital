use leptos::prelude::*;
use orbital_base_components::BaseInteractionTag;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::tag::styles::tag_styles;
use crate::tag::types::{TagAppearance, TagSize};

/// Clickable filter or toggle tag with optional split dismiss.
///
/// Put the label in [`InteractionTagPrimary`] and optionally add [`SecondaryActionTag`] for a separate dismiss or menu action — set `has_secondary_action` on primary when the layout needs two targets. Inherits `size` and `appearance` from a parent [`crate::TagGroup`] when not set locally. For static or dismissible labels use [`crate::Tag`](crate::Tag); for multi-select from predefined options use [`crate::TagPicker`](crate::TagPicker).
///
/// # When to use
///
/// - Filter or toggle tags with a primary click action - Toolbar modes that look like tags - Selectable categories where label click and dismiss are separate actions
///
/// # Usage
///
/// 1. Wrap [`InteractionTagPrimary`] for the main clickable region. 2. Optionally add [`SecondaryActionTag`] with `has_secondary_action=true` on primary. 3. Set `size` or inherit from a parent [`crate::TagGroup`].
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`InteractionTagPrimary`] for the primary label and click target * Match size to surrounding tags when nested in [`crate::TagGroup`] * Provide `aria_label` on [`SecondaryActionTag`] (e.g. `"Remove"`)
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default interaction tag
/// Primary region inside an interaction tag acts as the clickable filter or toggle target.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary};
/// view! {
///     <div data-testid="interaction-tag-preview">
///         <InteractionTag>
///             <InteractionTagPrimary>"Filter"</InteractionTagPrimary>
///         </InteractionTag>
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Medium, small, and extra-small presets scale interaction tags with surrounding UI.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary, TagSize};
/// view! {
///     <div data-testid="interaction-tag-sizes">
///         <div data-testid="interaction-tag-size-medium">
///             <InteractionTag size=Signal::from(TagSize::Medium)>
///                 <InteractionTagPrimary>"Medium"</InteractionTagPrimary>
///             </InteractionTag>
///         </div>
///         <div data-testid="interaction-tag-size-small">
///             <InteractionTag size=Signal::from(TagSize::Small)>
///                 <InteractionTagPrimary>"Small"</InteractionTagPrimary>
///             </InteractionTag>
///         </div>
///         <div data-testid="interaction-tag-size-extra-small">
///             <InteractionTag size=Signal::from(TagSize::ExtraSmall)>
///                 <InteractionTagPrimary>"XS"</InteractionTagPrimary>
///             </InteractionTag>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Primary click target
/// The primary button is the interactive region for filter selection.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary};
/// view! {
///     <div data-testid="interaction-tag-click">
///         <InteractionTag>
///             <InteractionTagPrimary>"Click me"</InteractionTagPrimary>
///         </InteractionTag>
///     </div>
/// }
/// ```
///
/// ## Inherits group size
/// Interaction tags inside a [`crate::TagGroup`] inherit the group size when `size` is omitted.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary, TagGroup, TagSize};
/// view! {
///     <div data-testid="interaction-tag-in-group">
///         <TagGroup size=Signal::from(TagSize::Small)>
///             <InteractionTag>
///                 <InteractionTagPrimary>"Grouped"</InteractionTagPrimary>
///             </InteractionTag>
///         </TagGroup>
///     </div>
/// }
/// ```
///
/// ## With secondary action
/// Pair [`InteractionTagPrimary`] with [`SecondaryActionTag`] for a separate dismiss button.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary, SecondaryActionTag};
/// view! {
///     <div data-testid="interaction-tag-secondary">
///         <InteractionTag>
///             <InteractionTagPrimary has_secondary_action=true>"Filter"</InteractionTagPrimary>
///             <SecondaryActionTag aria_label="Remove".to_string() />
///         </InteractionTag>
///     </div>
/// }
/// ```
///
/// ## Secondary removes tag
/// Wire `on_click` on [`SecondaryActionTag`] to update reactive state.
/// <!-- preview -->
/// ```rust
/// use leptos::ev;
/// use orbital_base_components::Handler;
/// use crate::{InteractionTag, InteractionTagPrimary, SecondaryActionTag};
/// let tags = RwSignal::new(vec!["a".to_string(), "b".to_string()]);
/// view! {
///     <div data-testid="interaction-tag-secondary-dismiss">
///         {move || tags.get().into_iter().map(|value| {
///             let label = value.clone();
///             let remove = value.clone();
///             view! {
///                 <InteractionTag>
///                     <InteractionTagPrimary has_secondary_action=true>{label}</InteractionTagPrimary>
///                     <SecondaryActionTag
///                         aria_label="Remove".to_string()
///                         on_click=Handler::on(move |_: ev::MouseEvent| {
///                             tags.update(|list| list.retain(|tag| tag != &remove));
///                         })
///                     />
///                 </InteractionTag>
///             }
///         }).collect_view()}
///     </div>
/// }
/// ```
///
/// ## Appearances
/// Filled, outline, and brand presets match [`crate::Tag`] styling.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary, SecondaryActionTag, TagAppearance};
/// view! {
///     <div data-testid="interaction-tag-appearances" style="display: flex; gap: 8px; flex-wrap: wrap;">
///         <InteractionTag appearance=Signal::from(TagAppearance::Filled)>
///             <InteractionTagPrimary has_secondary_action=true>"Filled"</InteractionTagPrimary>
///             <SecondaryActionTag aria_label="Remove".to_string() />
///         </InteractionTag>
///         <InteractionTag appearance=Signal::from(TagAppearance::Outline)>
///             <InteractionTagPrimary has_secondary_action=true>"Outline"</InteractionTagPrimary>
///             <SecondaryActionTag aria_label="Remove".to_string() />
///         </InteractionTag>
///         <InteractionTag appearance=Signal::from(TagAppearance::Brand)>
///             <InteractionTagPrimary has_secondary_action=true>"Brand"</InteractionTagPrimary>
///             <SecondaryActionTag aria_label="Remove".to_string() />
///         </InteractionTag>
///     </div>
/// }
/// ```
///
/// ## With icon and secondary
/// Primary can show a leading icon alongside a secondary dismiss action.
/// <!-- preview -->
/// ```rust
/// use crate::{InteractionTag, InteractionTagPrimary, SecondaryActionTag};
/// view! {
///     <div data-testid="interaction-tag-icon-secondary">
///         <InteractionTag>
///             <InteractionTagPrimary has_secondary_action=true icon=icondata::AiTagOutlined>
///                 "Design"
///             </InteractionTagPrimary>
///             <SecondaryActionTag aria_label="Remove".to_string() />
///         </InteractionTag>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "interaction-tag",
    preview_label = "Interaction Tag",
    preview_icon = icondata::AiTagOutlined,
)]
#[component]
pub fn InteractionTag(
    /// Optional CSS class on the tag root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Size preset; inherits from parent [`crate::TagGroup`] when omitted.
    #[prop(optional, into)]
    size: Option<Signal<TagSize>>,
    /// Visual style: `Filled`, `Outline`, or `Brand`; inherits from parent [`crate::TagGroup`] when omitted.
    #[prop(optional, into)]
    appearance: Option<Signal<TagAppearance>>,
    /// Primary action and optional secondary slots.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-tag", tag_styles());

    view! {
        <BaseInteractionTag class=class nostrip:size=size nostrip:appearance=appearance>
            {children()}
        </BaseInteractionTag>
    }
}
