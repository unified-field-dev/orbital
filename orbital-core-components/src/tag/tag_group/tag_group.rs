use leptos::prelude::*;
use orbital_base_components::{BaseTagGroup, Handler};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::tag::styles::tag_group_styles;
use crate::tag::types::{TagAppearance, TagSize};

/// Lays out [`crate::Tag`] children in a horizontal row with shared `size` and `appearance`.
///
/// Enable `dismissible` and handle `on_dismiss` with the tag's `value` to update a reactive list. Icons are set per child `Tag`, not on the group.
///
/// # When to use
///
/// - Tag lists where size and appearance should stay consistent - Grouped removable selections with centralized dismiss handling - Horizontal tag rows in forms and toolbars
///
/// # Usage
///
/// 1. Nest [`crate::Tag`] children with unique `value` props when dismissible. 2. Set `on_dismiss` to receive the dismissed tag's `value` string. 3. Set `size` once on the group instead of each child.
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide stable `value` strings for each dismissible child tag * Use `Handler<String>` for group-level dismiss callbacks
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default tag group
/// Grouped tags share horizontal spacing via the group container.
/// <!-- preview -->
/// ```rust
/// use crate::{Tag, TagGroup};
/// view! {
///     <div data-testid="tag-group-preview">
///         <TagGroup>
///             <Tag>"Alpha"</Tag>
///             <Tag>"Beta"</Tag>
///         </TagGroup>
///     </div>
/// }
/// ```
///
/// ## Dismissible group
/// Use `dismissible` to show dismiss buttons on child tags, and `on_dismiss` to handle removal. Each child needs a unique `value`; render tags from reactive state so the list updates when a tag is dismissed.
///
/// - **`dismissible`** — shows the dismiss affordance on tags that have `value` - **`on_dismiss`** — called with the dismissed tag's `value`; update your tag list signal here (e.g. `list.retain(|v| v != &value)`)
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::Handler;
/// use crate::{Tag, TagGroup};
/// let tags = RwSignal::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
/// view! {
///     <div data-testid="tag-group-dismissible">
///         <TagGroup
///             dismissible=true
///             on_dismiss=Handler::on(move |value: String| {
///                 tags.update(|list| list.retain(|tag| tag != &value));
///             })
///         >
///             {move || tags.get().into_iter().map(|value| {
///                 let label = value.clone();
///                 view! { <Tag value=value.clone()>{label}</Tag> }
///             }).collect_view()}
///         </TagGroup>
///     </div>
/// }
/// ```
///
/// ## Small size group
/// Set `size` on the group to scale all child tags together.
/// <!-- preview -->
/// ```rust
/// use crate::{Tag, TagGroup, TagSize};
/// view! {
///     <div data-testid="tag-group-small">
///         <TagGroup size=Signal::from(TagSize::Small)>
///             <Tag value="a".to_string()>"Small A"</Tag>
///             <Tag value="b".to_string()>"Small B"</Tag>
///         </TagGroup>
///     </div>
/// }
/// ```
///
/// ## Appearances
/// Set `appearance` on the group to apply filled, outline, or brand styling to all child tags.
/// <!-- preview -->
/// ```rust
/// use crate::{Tag, TagGroup, TagAppearance};
/// view! {
///     <div data-testid="tag-group-appearances" style="display: flex; gap: 8px; flex-wrap: wrap;">
///         <TagGroup appearance=Signal::from(TagAppearance::Filled)>
///             <Tag value="a".to_string()>"Filled"</Tag>
///         </TagGroup>
///         <TagGroup appearance=Signal::from(TagAppearance::Outline)>
///             <Tag value="b".to_string()>"Outline"</Tag>
///         </TagGroup>
///         <TagGroup appearance=Signal::from(TagAppearance::Brand)>
///             <Tag value="c".to_string()>"Brand"</Tag>
///         </TagGroup>
///     </div>
/// }
/// ```
///
/// ## Many tags layout
/// Inline flex layout keeps multiple tags on one row with consistent gap.
/// <!-- preview -->
/// ```rust
/// use crate::{Tag, TagGroup};
/// view! {
///     <div data-testid="tag-group-many">
///         <TagGroup>
///             <Tag>"Red"</Tag>
///             <Tag>"Green"</Tag>
///             <Tag>"Blue"</Tag>
///             <Tag>"Yellow"</Tag>
///             <Tag>"Purple"</Tag>
///         </TagGroup>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "tag-group",
    preview_label = "Tag Group",
    preview_icon = icondata::AiTagsOutlined,
)]
#[component]
pub fn TagGroup(
    /// Optional CSS class on the group root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Shared size for child tags.
    #[prop(optional, into)]
    size: Signal<TagSize>,
    /// Shared appearance for child tags.
    #[prop(optional, into)]
    appearance: Signal<TagAppearance>,
    /// Called with the dismissed tag's `value` string.
    #[prop(optional, into)]
    on_dismiss: Option<Handler<String>>,
    /// When true, child tags show dismiss affordances.
    #[prop(optional, into)]
    dismissible: Signal<bool>,
    /// Child [`crate::Tag`] elements.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-tag-group", tag_group_styles());

    view! {
        <BaseTagGroup class=class size=size appearance=appearance nostrip:on_dismiss=on_dismiss dismissible=dismissible>
            {children()}
        </BaseTagGroup>
    }
}
