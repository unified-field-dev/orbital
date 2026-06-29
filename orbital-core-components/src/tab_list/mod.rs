//! Tab list primitives for switching related views at the same level.

use leptos::prelude::*;
use orbital_base_components::{BaseTab, BaseTabList};
use orbital_macros::component_doc;

const TAB_LIST_CSS: &str = include_str!("tab-list.css");
const TAB_CSS: &str = include_str!("tab.css");

/// `TabList` renders a selectable tab strip for switching related views. Add [`Tab`] children
/// with distinct `value` strings and bind `selected_value` to a parent `RwSignal`; render panel
/// content in the parent keyed on that signal — Orbital does not ship a bundled panel wrapper.
///
/// # When to use
///
/// - Settings sections, record detail areas, peer views at one hierarchy level - Replacing a row of toggle buttons when labels need tab affordance
///
/// # Usage
///
/// 1. Create `selected_value` as `RwSignal::new("tab-id".to_string())`. 2. Pass it to [`TabList`] and give each [`Tab`] a matching `value`. 3. Below the list, render a panel keyed off `selected.get()` (or `<Show>` / `match`). 4. Keyboard: arrow keys move focus; Enter or Space activates a tab.
///
/// # Examples
///
/// ## Two tabs
/// Minimal tab strip with two peer views and a parent-owned `selected_value` signal. Use [`TabList`] when switching between related panels at the same hierarchy level.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// let selected = RwSignal::new("first".to_string());
/// view! {
///     <div data-testid="tab-list-preview">
///         <TabList selected_value=selected>
///             <Tab value="first">"First"</Tab>
///             <Tab value="second">"Second"</Tab>
///         </TabList>
///     </div>
/// }
/// ```
///
/// ## Three tabs with initial selection
/// Three labeled tabs with the middle tab selected on load. Demonstrates binding `selected_value` to a non-first tab for detail areas such as Overview, Details, and History.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// let selected = RwSignal::new("b".to_string());
/// view! {
///     <div data-testid="tab-list-three">
///         <TabList selected_value=selected>
///             <Tab value="a">"Overview"</Tab>
///             <Tab value="b">"Details"</Tab>
///             <Tab value="c">"History"</Tab>
///         </TabList>
///     </div>
/// }
/// ```
///
/// ## Controlled tab list + panel
/// Tab list wired to a sibling panel that reads the same `selected_value` signal. This is the standard controlled pattern—tabs update selection and the panel renders the matching content below.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// let selected = RwSignal::new("one".to_string());
/// view! {
///     <div data-testid="tab-list-controlled">
///         <TabList selected_value=selected>
///             <Tab value="one">"One"</Tab>
///             <Tab value="two">"Two"</Tab>
///         </TabList>
///         <div data-testid="tab-list-panel">
///             {move || if selected.get() == "one" { "Panel one" } else { "Panel two" }}
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "tab-list",
    preview_label = "Tab List",
    preview_icon = icondata::AiUnorderedListOutlined,
)]
#[component]
pub fn TabList(
    /// Extra CSS class names merged onto the tab list root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Parent-owned signal for the active tab `value`; defaults to an internal signal when omitted.
    #[prop(optional, into)]
    selected_value: Option<RwSignal<String>>,
    /// [`Tab`] children — one per selectable tab label.
    children: Children,
) -> impl IntoView {
    let selected_value = selected_value.unwrap_or_else(|| RwSignal::new(String::new()));

    view! {
        <style>{TAB_LIST_CSS}</style>
        <style>{TAB_CSS}</style>
        <BaseTabList class=class selected_value=selected_value>
            {children()}
        </BaseTabList>
    }
}
/// Single tab label within a [`TabList`].
#[component]
pub fn Tab(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: String,
    children: Children,
) -> impl IntoView {
    view! {
        <BaseTab class=class value=value>
            {children()}
        </BaseTab>
    }
}
