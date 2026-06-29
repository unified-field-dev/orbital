use leptos::prelude::*;
use orbital_base_components::BaseTreeRoot;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::api::{build_tree_state, install_tree_reorder_listeners, TreeApiHandle, TreeApiRef};
use super::styles::tree_styles;
use super::types::{TreeAppearance, TreeBehavior, TreeExpansion, TreeSelection};

/// Hand-composed hierarchy for sidebars, pickers, and file lists where you control every row.
///
/// Wrap rows in nested [`TreeItem`] components with [`TreeItemLayout`] slots. Bind [`TreeExpansion`] and [`TreeSelection`] with Leptos signals for open and selected ids. When items come from an API, lazy loading, or virtualization, use [`RichTree`](super::rich_tree::RichTree) instead — same selection and expansion APIs, data adapters instead of nested components.
///
/// # When to use
///
/// - Hand-authored hierarchies with custom slots (icons, aside actions, motion) - Sidebars and pickers where item count is modest - Full control over row layout via [`TreeItemLayout`]
///
/// # Usage
///
/// 1. Wrap rows in [`Tree`] + nested [`TreeItem`] components. 2. Bind [`TreeExpansion::open_items`] for controlled expand/collapse. 3. Choose [`TreeSelection`] mode (`single`, `multi`, `checkbox`, or `none`).
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`TreeApiRef`](super::api::TreeApiRef) to focus items from external controls * Set `data-testid` on preview wrappers for E2E coverage * Prefer [`RichTree`](super::rich_tree::RichTree) when items come from an API
///
/// ## Don'ts
///
/// * Don't render thousands of static [`TreeItem`] nodes — use [`RichTree`](super::rich_tree::RichTree) with virtualization * Don't rely on mount order for keyboard range selection — the registry orders visible items
///
/// # Examples
///
/// ## Default nested tree
/// Branch pre-expanded via `open_items`; nested leaf under Documents.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::from(["root".to_string()]));
/// view! {
///     <div data-testid="tree-view-preview">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("root")>
///                 <TreeItemLayout slot>
///                     <span data-testid="tree-node-docs">"Documents"</span>
///                 </TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("readme")>
///                     <TreeItemLayout slot>
///                         <span data-testid="tree-node-readme">"readme.md"</span>
///                     </TreeItemLayout>
///                 </TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Small size
/// Compact row height for dense sidebars.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeAppearance, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection, TreeSize};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let appearance = TreeAppearance::new(Signal::derive(|| TreeSize::Small));
/// view! {
///     <div data-testid="tree-small">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() appearance=appearance>
///             <TreeItem config=TreeItemConfig::leaf("a")>
///                 <TreeItemLayout slot>"Item"</TreeItemLayout>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Deep nesting
/// Three-level branch chain demonstrates nested subtrees.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::from(["l1".to_string(), "l2".to_string()]));
/// view! {
///     <div data-testid="tree-deep">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("l1")>
///                 <TreeItemLayout slot>"Level 1"</TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::branch("l2")>
///                     <TreeItemLayout slot>"Level 2"</TreeItemLayout>
///                     <TreeItem config=TreeItemConfig::leaf("l3")>
///                         <TreeItemLayout slot>"Level 3"</TreeItemLayout>
///                     </TreeItem>
///                 </TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Click to expand
/// Starts collapsed; clicking the branch toggles `open_items`.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="tree-toggle">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("toggle")>
///                 <TreeItemLayout slot>
///                     <span data-testid="tree-branch-toggle">"Expand me"</span>
///                 </TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("child")>
///                     <TreeItemLayout slot>"Hidden until expanded"</TreeItemLayout>
///                 </TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Flat leaf list
/// All leaves — no chevrons.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="tree-flat">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::leaf("1")>
///                 <TreeItemLayout slot>"One"</TreeItemLayout>
///             </TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("2")>
///                 <TreeItemLayout slot>"Two"</TreeItemLayout>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Single selection
/// Click rows to select one item at a time.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::{SignalModel, TreeSelectionMode};
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let selected = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="tree-selection">
///         <Tree
///             expansion=TreeExpansion::new(SignalModel::from(open))
///             selection=TreeSelection::single(SignalModel::from(selected))
///         >
///             <TreeItem config=TreeItemConfig::leaf("a")>
///                 <TreeItemLayout slot>"Alpha"</TreeItemLayout>
///             </TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("b")>
///                 <TreeItemLayout slot>"Beta"</TreeItemLayout>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Theme surfaces
/// Items use theme background tokens.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::from(["theme".to_string()]));
/// view! {
///     <div data-testid="tree-theme">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("theme")>
///                 <TreeItemLayout slot>
///                     <span data-testid="tree-theme-cell" style="background: var(--orb-color-surface-canvas); padding: 4px 8px;">"Themed row"</span>
///                 </TreeItemLayout>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## RichTree data-driven
/// Pass structured data instead of composing nested items.
/// <!-- code-only -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "docs".into(), label: "Documents".into(), children: vec![
///         Node { id: "readme".into(), label: "readme.md".into(), children: vec![] },
///     ]},
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone());
/// let open = RwSignal::new(HashSet::from(["docs".to_string()]));
/// view! {
///     <div data-testid="tree-rich">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
///
/// ## Icons and aside actions
/// Leading icon, trailing badge, and hover-revealed aside actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemIconAfter, TreeItemIconBefore, TreeItemAside, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::from(["docs".to_string()]));
/// view! {
///     <div data-testid="tree-icons-aside">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("docs")>
///                 <TreeItemLayout slot>
///                     <TreeItemIconBefore slot>
///                         <span>"📁"</span>
///                     </TreeItemIconBefore>
///                     "Documents"
///                     <TreeItemIconAfter slot>
///                         <span>"3"</span>
///                     </TreeItemIconAfter>
///                     <TreeItemAside slot>
///                         <Button appearance=ButtonAppearance::Subtle icon=icondata::AiEllipsisOutlined />
///                     </TreeItemAside>
///                 </TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("readme")>
///                     <TreeItemLayout slot>"readme.md"</TreeItemLayout>
///                 </TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Multi selection (ctrl + shift)
/// Ctrl/Meta toggles; Shift selects a visible range.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::{SignalModel, TreeSelectionMode};
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let selected = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="tree-multi">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::multi(SignalModel::from(selected))>
///             <TreeItem config=TreeItemConfig::leaf("a")><TreeItemLayout slot>"Alpha"</TreeItemLayout></TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("b")><TreeItemLayout slot>"Beta"</TreeItemLayout></TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("c")><TreeItemLayout slot>"Gamma"</TreeItemLayout></TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Checkbox selection with cascade
/// Parent checkbox selects descendants when `cascade` is enabled.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::from(["root".to_string()]));
/// let selected = RwSignal::new(HashSet::new());
/// let selection = TreeSelection::checkbox(SignalModel::from(selected)).with_cascade(true);
/// view! {
///     <div data-testid="tree-checkbox">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=selection>
///             <TreeItem config=TreeItemConfig::branch("root")>
///                 <TreeItemLayout slot>"Root"</TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("child")><TreeItemLayout slot>"Child"</TreeItemLayout></TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Disabled items (focusable)
/// Disabled rows remain focusable for screen-reader access.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeBehavior, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let behavior = TreeBehavior::new()
///     .with_disabled_items(SignalModel::new(HashSet::from(["b".to_string()])))
///     .with_disabled_items_focusable(Signal::from(true));
/// view! {
///     <div data-testid="tree-disabled">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() behavior=behavior>
///             <TreeItem config=TreeItemConfig::leaf("a")><TreeItemLayout slot>"Enabled"</TreeItemLayout></TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("b")><TreeItemLayout slot>"Disabled"</TreeItemLayout></TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Icon-container expansion
/// Only the chevron toggles expansion; row click selects.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::{ExpansionTrigger, SignalModel};
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let selected = RwSignal::new(HashSet::new());
/// let expansion = TreeExpansion::new(SignalModel::from(open)).with_expansion_trigger(ExpansionTrigger::IconContainer);
/// view! {
///     <div data-testid="tree-icon-expand">
///         <Tree expansion=expansion selection=TreeSelection::single(SignalModel::from(selected))>
///             <TreeItem config=TreeItemConfig::branch("branch")>
///                 <TreeItemLayout slot><span data-testid="tree-icon-branch">"Branch"</span></TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("leaf")><TreeItemLayout slot>"Leaf"</TreeItemLayout></TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Expand / collapse all
/// External button uses [`TreeApiRef`](super::api::TreeApiRef) to toggle every branch.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, Tree, TreeApiRef, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let api = TreeApiRef::default();
/// let api_for_button = api.clone();
/// view! {
///     <div data-testid="tree-expand-all">
///         <Button on_click=Callback::new(move |_| {
///             api_for_button.with(|handle| {
///                 for (id, _) in handle.get_item_tree() {
///                     handle.set_item_expansion(id, true);
///                 }
///             });
///         })>"Expand all"</Button>
///         <Tree api_ref=api expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("a")>
///                 <TreeItemLayout slot>"A"</TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("a1")><TreeItemLayout slot>"A1"</TreeItemLayout></TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Connector borders
/// Indentation guide lines between levels.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeAppearance, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::from(["root".to_string()]));
/// let appearance = TreeAppearance::new(Signal::from(orbital_base_components::TreeSize::Medium)).with_connectors(Signal::from(true));
/// view! {
///     <div data-testid="tree-connectors">
///         <Tree appearance=appearance expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("root")>
///                 <TreeItemLayout slot>"Root"</TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("leaf")><TreeItemLayout slot>"Leaf"</TreeItemLayout></TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Focus via button
/// Imperative focus through [`TreeApiRef::focus_item`].
/// <!-- preview -->
/// ```rust
/// use crate::{Button, Tree, TreeApiRef, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let api = TreeApiRef::default();
/// let api_for_button = api.clone();
/// view! {
///     <div data-testid="tree-focus-button">
///         <Button on_click=Callback::new(move |_| { api_for_button.with(|handle| handle.focus_item("target".into())); })>"Focus target"</Button>
///         <Tree api_ref=api expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::leaf("target")><TreeItemLayout slot>"Target row"</TreeItemLayout></TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Keyboard navigation
/// Arrow keys move focus; typeahead jumps to matching labels.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="tree-keyboard">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::leaf("alpha")><TreeItemLayout slot>"Alpha"</TreeItemLayout></TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("beta")><TreeItemLayout slot>"Beta"</TreeItemLayout></TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Inline editing
/// Double-click or Enter on a focused row edits the label.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeBehavior, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::{Handler, SignalModel};
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let log = RwSignal::new(String::new());
/// let behavior = TreeBehavior::new()
///     .with_editable(Signal::from(true))
///     .with_on_label_change(Handler::on(move |(id, label)| log.set(format!("{id}:{label}"))));
/// view! {
///     <div data-testid="tree-editing">
///         <Tree behavior=behavior expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::leaf("edit")><TreeItemLayout slot>"Rename me"</TreeItemLayout></TreeItem>
///         </Tree>
///         <span data-testid="tree-edit-log">{move || log.get()}</span>
///     </div>
/// }
/// ```
///
/// ## Drag reorder
/// Pointer drag reorders siblings when `reorderable` is enabled.
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeBehavior, TreeExpansion, TreeItem, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// let behavior = TreeBehavior::new().with_reorderable(Signal::from(true));
/// view! {
///     <div data-testid="tree-reorder">
///         <Tree behavior=behavior expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::leaf("one")><TreeItemLayout slot>"One"</TreeItemLayout></TreeItem>
///             <TreeItem config=TreeItemConfig::leaf("two")><TreeItemLayout slot>"Two"</TreeItemLayout></TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
///
/// ## Custom collapse motion
/// Override the CSS transition name via [`TreeItemCollapse`].
/// <!-- preview -->
/// ```rust
/// use crate::{Tree, TreeExpansion, TreeItem, TreeItemCollapse, TreeItemConfig, TreeItemLayout, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="tree-custom-motion">
///         <Tree expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none()>
///             <TreeItem config=TreeItemConfig::branch("motion")>
///                 <TreeItemLayout slot>
///                     "Animated branch"
///                     <TreeItemCollapse slot motion_name="orbital-tree-custom-collapse".to_string() />
///                 </TreeItemLayout>
///                 <TreeItem config=TreeItemConfig::leaf("child")><TreeItemLayout slot>"Child"</TreeItemLayout></TreeItem>
///             </TreeItem>
///         </Tree>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Tree Views",
    preview_slug = "tree-view",
    preview_label = "Tree View",
    preview_icon = icondata::AiApartmentOutlined,
)]
#[component]
pub fn Tree(
    /// Expansion state and triggers.
    #[prop(optional, into)]
    expansion: TreeExpansion,
    /// Selection mode and selected item ids.
    #[prop(optional, into)]
    selection: TreeSelection,
    /// Editing, reorder, and interaction callbacks.
    #[prop(optional, into)]
    behavior: TreeBehavior,
    /// Visual density and motion settings.
    #[prop(optional, into)]
    appearance: TreeAppearance,
    /// Imperative tree API handle.
    #[prop(default = TreeApiRef::default())]
    api_ref: TreeApiRef,
    /// Extra CSS class names merged onto the tree root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// [`TreeItem`] children forming the expandable hierarchy.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-tree", tree_styles());

    let tree_state = StoredValue::new(build_tree_state(
        expansion,
        selection,
        behavior,
        appearance.clone(),
    ));
    let state = tree_state.get_value();
    let drag_listeners = install_tree_reorder_listeners(state.clone());
    on_cleanup(move || drag_listeners.remove());
    api_ref.load(TreeApiHandle::from_state(state.clone()));

    let connectors = appearance.connectors;
    let merged_class = Signal::derive(move || {
        let mut parts = Vec::new();
        if connectors.get() {
            parts.push("orbital-tree--connectors".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <BaseTreeRoot class=merged_class tree_state=state>{children()}</BaseTreeRoot>
    }
}
