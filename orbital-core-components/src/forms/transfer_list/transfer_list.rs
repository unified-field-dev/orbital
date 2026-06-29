use std::collections::HashSet;

use leptos::prelude::*;
use orbital_base_components::{move_all, move_checked, Handler, TransferListItem};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::Button;
use crate::ButtonAppearance;

use super::panel::TransferListPanel;
use super::styles::transfer_list_styles;
use super::types::{TransferListChange, TransferListConfig};

fn emit_change(
    left: RwSignal<Vec<TransferListItem>>,
    right: RwSignal<Vec<TransferListItem>>,
) -> TransferListChange {
    TransferListChange {
        left_count: left.get_untracked().len(),
        right_count: right.get_untracked().len(),
    }
}

/// Two-panel shuttle for moving items between available and chosen sets.
///
/// TransferList is a two-panel shuttle for moving items between "available" and "chosen" sets — permissions, team membership, or tag assignment. Supply `left` and `right` item vectors as app-owned [`RwSignal`]s; the component moves checked items between panels and reports counts via `on_change`. Enable `enhanced` for select-all headers and selection counters.
///
/// # When to use
///
/// - Assigning permissions, tags, or group membership between available and chosen sets
/// - Desktop workflows where both source and destination lists should stay visible
///
/// # Usage
///
/// 1. Provide `left` and `right` item vectors as [`RwSignal`]s in [`TransferListConfig`].
/// 2. Handle `on_change` when items move between panels.
/// 3. Enable `enhanced` for select-all headers and selection counters.
///
/// # Examples
///
/// ## Basic shuttle
/// Move checked items between two lists with single-step and move-all controls.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{TransferList, TransferListChange, TransferListConfig};
/// use leptos::prelude::*;
/// use orbital_base_components::TransferListItem;
/// let left = RwSignal::new(vec![
///     TransferListItem::new("1", "List item 1"),
///     TransferListItem::new("2", "List item 2"),
///     TransferListItem::new("3", "List item 3"),
///     TransferListItem::new("4", "List item 4"),
/// ]);
/// let right = RwSignal::new(vec![
///     TransferListItem::new("5", "List item 5"),
///     TransferListItem::new("6", "List item 6"),
/// ]);
/// view! {
///     <div data-testid="transfer-list-preview">
///         <TransferList config=TransferListConfig::basic(left, right) on_change=Handler::on(|_: TransferListChange| {}) />
///     </div>
/// }
/// ```
///
/// ## Enhanced select-all
/// Header checkbox selects all enabled rows; counters show selection progress.
/// <!-- preview -->
/// ```rust
/// use crate::{TransferList, TransferListChange, TransferListConfig};
/// use leptos::prelude::*;
/// use orbital_base_components::TransferListItem;
/// let left = RwSignal::new((1..=4).map(|n| TransferListItem::new(format!("l-{n}"), format!("List item {n}"))).collect());
/// let right = RwSignal::new((5..=8).map(|n| TransferListItem::new(format!("r-{n}"), format!("List item {n}"))).collect());
/// view! {
///     <div data-testid="transfer-list-enhanced">
///         <TransferList
///             config=TransferListConfig {
///                 left,
///                 right,
///                 enhanced: Signal::from(true),
///                 show_move_all: Signal::from(false),
///             }
///             on_change=Handler::on(|_: TransferListChange| {})
///         />
///     </div>
/// }
/// ```
///
/// ## Custom labels
/// Distinct labels remain visible in both panels after items move.
/// <!-- preview -->
/// ```rust
/// use crate::{TransferList, TransferListChange, TransferListConfig};
/// use leptos::prelude::*;
/// use orbital_base_components::TransferListItem;
/// let left = RwSignal::new(vec![
///     TransferListItem::new("alpha", "Alpha team"),
///     TransferListItem::new("beta", "Beta team"),
/// ]);
/// let right = RwSignal::new(vec![TransferListItem::new("gamma", "Gamma team")]);
/// view! {
///     <div data-testid="transfer-list-labels">
///         <TransferList config=TransferListConfig::basic(left, right) on_change=Handler::on(|_: TransferListChange| {}) />
///     </div>
/// }
/// ```
///
/// ## Disabled items
/// Disabled rows cannot be selected or moved between panels.
/// <!-- preview -->
/// ```rust
/// use crate::{TransferList, TransferListChange, TransferListConfig};
/// use leptos::prelude::*;
/// use orbital_base_components::TransferListItem;
/// let left = RwSignal::new(vec![
///     TransferListItem::new("open", "Movable item"),
///     TransferListItem::new("locked", "Locked item").disabled(),
/// ]);
/// let right = RwSignal::new(Vec::<TransferListItem>::new());
/// view! {
///     <div data-testid="transfer-list-disabled">
///         <TransferList config=TransferListConfig::basic(left, right) on_change=Handler::on(|_: TransferListChange| {}) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "transfer-list",
    preview_label = "Transfer List",
    preview_icon = icondata::AiSwapOutlined,
)]
#[component]
pub fn TransferList(
    /// Panel data and feature flags.
    config: TransferListConfig,
    /// Optional CSS class on the root shuttle layout.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Called after items move between panels.
    #[prop(optional)]
    on_change: Option<Handler<TransferListChange>>,
) -> impl IntoView {
    inject_style("orbital-transfer-list", transfer_list_styles());

    let left_checked = RwSignal::new(HashSet::<String>::new());
    let right_checked = RwSignal::new(HashSet::<String>::new());
    let left_items = Signal::derive(move || config.left.get());
    let right_items = Signal::derive(move || config.right.get());

    let move_right = {
        let on_change = on_change.clone();
        move |_| {
            config.left.update(|left| {
                config.right.update(|right| {
                    move_checked(left, right, &left_checked.get_untracked());
                });
            });
            left_checked.set(HashSet::new());
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(emit_change(config.left, config.right));
            }
        }
    };
    let move_left = {
        let on_change = on_change.clone();
        move |_| {
            config.right.update(|right| {
                config.left.update(|left| {
                    move_checked(right, left, &right_checked.get_untracked());
                });
            });
            right_checked.set(HashSet::new());
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(emit_change(config.left, config.right));
            }
        }
    };
    let move_all_right = {
        let on_change = on_change.clone();
        move |_| {
            config.left.update(|left| {
                config.right.update(|right| move_all(left, right));
            });
            left_checked.set(HashSet::new());
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(emit_change(config.left, config.right));
            }
        }
    };
    let move_all_left = {
        let on_change = on_change.clone();
        move |_| {
            config.right.update(|right| {
                config.left.update(|left| move_all(right, left));
            });
            right_checked.set(HashSet::new());
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(emit_change(config.left, config.right));
            }
        }
    };

    let move_right_cb = Callback::new(move_right);
    let move_all_right_cb = Callback::new(move_all_right);
    let move_left_cb = Callback::new(move_left);
    let move_all_left_cb = Callback::new(move_all_left);

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-transfer-list".to_string()];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        }>
            <TransferListPanel
                title="Choices".to_string()
                items=left_items
                checked=left_checked
                enhanced=config.enhanced
                list_testid="transfer-list-left".to_string()
            />
            <div class="orbital-transfer-list__controls">
                <Button appearance=ButtonAppearance::Secondary on_click=move_right_cb>
                    ">"
                </Button>
                <Show when=move || config.show_move_all.get()>
                    <Button appearance=ButtonAppearance::Secondary on_click=move_all_right_cb>
                        ">>"
                    </Button>
                </Show>
                <Button appearance=ButtonAppearance::Secondary on_click=move_left_cb>
                    "<"
                </Button>
                <Show when=move || config.show_move_all.get()>
                    <Button appearance=ButtonAppearance::Secondary on_click=move_all_left_cb>
                        "<<"
                    </Button>
                </Show>
            </div>
            <TransferListPanel
                title="Chosen".to_string()
                items=right_items
                checked=right_checked
                enhanced=config.enhanced
                list_testid="transfer-list-right".to_string()
            />
        </div>
    }
}
