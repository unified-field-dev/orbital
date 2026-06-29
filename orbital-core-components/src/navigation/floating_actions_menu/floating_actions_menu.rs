use icondata_core::Icon as IconData;
use leptos::{ev, html, prelude::*};
use orbital_base_components::{
    overlay::dom_events::on_click_outside, BaseFloatingButton, FloatingActionsMenuInjection,
    Handler, OpenBind,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::floating_button::floating_button_styles;
use crate::Icon;

use super::styles::floating_actions_menu_styles;
use super::types::FloatingActionsMenuConfig;

#[component]
fn FloatingActionsMenuRoot(
    config: FloatingActionsMenuConfig,
    open: OpenBind,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] trigger_icon: MaybeProp<IconData>,
    #[prop(optional, into)] open_icon: MaybeProp<IconData>,
    children: Children,
) -> impl IntoView {
    inject_style(
        "orbital-floating-actions-menu",
        floating_actions_menu_styles(),
    );
    inject_style("orbital-floating-button", floating_button_styles());

    let root_ref = NodeRef::<html::Div>::new();
    let open_rw = match open {
        OpenBind::Signal(signal) => signal,
        OpenBind::ReadWrite(read, write) => {
            let rw = RwSignal::new(read.get_untracked());
            Effect::new(move |_| rw.set(read.get()));
            Effect::new(move |_| write.set(rw.get()));
            rw
        }
    };

    provide_context(FloatingActionsMenuInjection {
        open: open_rw,
        persistent_tooltips: config.persistent_tooltips,
    });

    on_click_outside(
        move || {
            if !open_rw.get_untracked() {
                return None;
            }
            root_ref.get_untracked().map(|el| vec![el.into()])
        },
        move || open_rw.set(false),
    );

    Effect::new(move |_| {
        if !open_rw.get() {
            return;
        }
        let handle = window_event_listener(ev::keydown, move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                open_rw.set(false);
            }
        });
        on_cleanup(move || handle.remove());
    });

    let toggle = move |_| open_rw.update(|value| *value = !*value);
    let trigger_icon = move || trigger_icon.get().unwrap_or(icondata::AiPlusOutlined);
    let open_icon = move || open_icon.get().unwrap_or(icondata::AiCloseOutlined);

    view! {
        <div
            node_ref=root_ref
            class=move || {
                let mut parts = vec![
                    "orbital-floating-actions-menu".to_string(),
                    format!(
                        "orbital-floating-actions-menu--direction-{}",
                        config.direction.get().as_str()
                    ),
                ];
                if config.viewport_fixed.get()
                    && (config.right.get().is_some() || config.bottom.get().is_some())
                {
                    parts.push("orbital-floating-actions-menu--fixed".to_string());
                } else if config.right.get().is_some() || config.bottom.get().is_some() {
                    parts.push("orbital-floating-actions-menu--anchored".to_string());
                }
                if open_rw.get() {
                    parts.push("orbital-floating-actions-menu--open".to_string());
                }
                parts.push(format!(
                    "orbital-floating-actions-menu--tooltip-{}",
                    config.persistent_tooltip_side.get().as_str()
                ));
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            style=move || {
                let mut styles = Vec::new();
                if let Some(right) = config.right.get() {
                    styles.push(format!("right: {right}px"));
                }
                if let Some(bottom) = config.bottom.get() {
                    styles.push(format!("bottom: {bottom}px"));
                }
                styles.join("; ")
            }
            role="group"
            aria-label=config.aria_label.clone()
        >
            <div
                class="orbital-floating-actions-menu__actions"
                role="menu"
                aria-orientation=move || config.direction.get().aria_orientation()
                aria-expanded=move || open_rw.get().to_string()
            >
                {children()}
            </div>
            <div class=move || {
                if open_rw.get() {
                    "orbital-floating-actions-menu__trigger orbital-floating-actions-menu__trigger--open"
                } else {
                    "orbital-floating-actions-menu__trigger"
                }
            }>
                <BaseFloatingButton
                    class="orbital-floating-button orbital-floating-button--primary orbital-floating-button--medium orbital-floating-button--rounded"
                    aria_label=config.aria_label.clone()
                    on_click=Handler::on(toggle)
                    testid="floating-actions-menu-trigger".to_string()
                >
                    {move || if open_rw.get() {
                        view! { <Icon icon=open_icon() /> }.into_any()
                    } else {
                        view! { <Icon icon=trigger_icon() /> }.into_any()
                    }}
                </BaseFloatingButton>
            </div>
        </div>
    }
}

/// Expands a primary floating trigger into a short list of related actions.
///
/// Compose [`FloatingActionsMenuItem`] children for secondary actions that fan out from the main [`FloatingButton`](crate::FloatingButton) trigger.
///
/// # When to use
///
/// - One primary floating action with three to six related secondaries fanned in a chosen direction - App-level placement via [`FloatingActionsMenuConfig::fixed`] or anchored inside a positioned panel via [`FloatingActionsMenuConfig::anchored`]
///
/// # Usage
///
/// 1. Set `config.aria_label` on the menu root. 2. Add three to six [`FloatingActionsMenuItem`] children with distinct tooltips. 3. Optionally bind `open` for controlled visibility.
///
/// # Examples
///
/// ## Basic direction up
/// Primary trigger opens three related actions stacked above the trigger.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingActionsMenu, FloatingActionsMenuConfig, FloatingActionsMenuItem};
/// view! {
///     <div data-testid="floating-actions-menu-preview" style="position: relative; width: 100%; min-height: 280px; display: flex; align-items: flex-end; justify-content: flex-end; overflow: visible; padding: var(--orb-space-inline-lg);">
///         <FloatingActionsMenu config=FloatingActionsMenuConfig {
///             aria_label: "Create".to_string(),
///             right: Signal::from(None),
///             bottom: Signal::from(None),
///             viewport_fixed: Signal::from(false),
///             ..Default::default()
///         }>
///             <FloatingActionsMenuItem tooltip="Copy".to_string() icon=icondata::AiCopyOutlined testid="fam-copy" />
///             <FloatingActionsMenuItem tooltip="Print".to_string() icon=icondata::AiPrinterOutlined testid="fam-print" />
///             <FloatingActionsMenuItem tooltip="Share".to_string() icon=icondata::AiShareAltOutlined testid="fam-share" />
///         </FloatingActionsMenu>
///     </div>
/// }
/// ```
///
/// ## Custom close icon
/// Swap the trigger icon when the menu is open.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingActionsMenu, FloatingActionsMenuConfig, FloatingActionsMenuItem};
/// use icondata_core::Icon as IconData;
/// view! {
///     <div data-testid="floating-actions-menu-icons" style="position: relative; width: 100%; min-height: 280px; display: flex; align-items: flex-end; justify-content: flex-end; overflow: visible; padding: var(--orb-space-inline-lg);">
///         <FloatingActionsMenu
///             config=FloatingActionsMenuConfig {
///                 aria_label: "Edit".to_string(),
///                 right: Signal::from(None),
///                 bottom: Signal::from(None),
///                 viewport_fixed: Signal::from(false),
///                 ..Default::default()
///             }
///             trigger_icon=icondata::AiEditOutlined
///             open_icon=icondata::AiCloseOutlined
///         >
///             <FloatingActionsMenuItem tooltip="Duplicate".to_string() icon=icondata::AiCopyOutlined />
///         </FloatingActionsMenu>
///     </div>
/// }
/// ```
///
/// ## Controlled open
/// External button toggles the menu; Escape closes it.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, FloatingActionsMenu, FloatingActionsMenuConfig, FloatingActionsMenuItem};
/// use leptos::prelude::*;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="floating-actions-menu-controlled" style="position: relative; width: 100%; min-height: 280px; overflow: visible; padding: var(--orb-space-inline-lg);">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open menu"</Button>
///         <div style="position: absolute; right: var(--orb-space-inline-lg); bottom: var(--orb-space-block-lg);">
///             <FloatingActionsMenu
///                 open=open
///                 config=FloatingActionsMenuConfig {
///                     aria_label: "Actions".to_string(),
///                     right: Signal::from(None),
///                     bottom: Signal::from(None),
///                     viewport_fixed: Signal::from(false),
///                     ..Default::default()
///                 }
///             >
///                 <FloatingActionsMenuItem tooltip="Save".to_string() icon=icondata::AiSaveOutlined />
///             </FloatingActionsMenu>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Direction matrix
/// Compare up, right, down, and left expansion in a bounded frame.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingActionsMenu, FloatingActionsMenuConfig, FloatingActionsMenuItem, Flex, FlexGap, Stack, StackConfig};
/// use leptos::prelude::*;
/// use orbital_base_components::FloatingActionsMenuDirection;
/// view! {
///     <div data-testid="floating-actions-menu-directions" style="position: relative; min-height: 320px; overflow: visible; padding: 24px;">
///         <Stack config=StackConfig::horizontal(FlexGap::Large)>
///             <div style="position: relative; min-width: 180px; min-height: 260px; overflow: visible; display: flex; align-items: flex-end; justify-content: flex-end;">
///                 <FloatingActionsMenu config=FloatingActionsMenuConfig {
///                     aria_label: "Up".to_string(),
///                     direction: Signal::from(FloatingActionsMenuDirection::Up),
///                     right: Signal::from(None),
///                     bottom: Signal::from(None),
///                     viewport_fixed: Signal::from(false),
///                     ..Default::default()
///                 }>
///                     <FloatingActionsMenuItem tooltip="Up".to_string() icon=icondata::AiArrowUpOutlined />
///                 </FloatingActionsMenu>
///             </div>
///             <div style="position: relative; min-width: 180px; min-height: 260px; overflow: visible; display: flex; align-items: flex-end; justify-content: flex-end;">
///                 <FloatingActionsMenu config=FloatingActionsMenuConfig {
///                     aria_label: "Right".to_string(),
///                     direction: Signal::from(FloatingActionsMenuDirection::Right),
///                     right: Signal::from(None),
///                     bottom: Signal::from(None),
///                     viewport_fixed: Signal::from(false),
///                     ..Default::default()
///                 }>
///                     <FloatingActionsMenuItem tooltip="Right".to_string() icon=icondata::AiArrowRightOutlined />
///                 </FloatingActionsMenu>
///             </div>
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Persistent tooltips (left)
/// Labels stay visible to the left of each action while the menu is open. Set `persistent_tooltips` and `persistent_tooltip_side: Left`.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingActionsMenu, FloatingActionsMenuConfig, FloatingActionsMenuItem};
/// use leptos::prelude::*;
/// use orbital_base_components::FloatingActionsMenuTooltipSide;
/// view! {
///     <div data-testid="floating-actions-menu-tooltips-left" style="position: relative; width: 100%; min-height: 280px; display: flex; align-items: flex-end; justify-content: flex-end; overflow: visible; padding: var(--orb-space-inline-lg);">
///         <FloatingActionsMenu
///             config=FloatingActionsMenuConfig {
///                 aria_label: "Share".to_string(),
///                 persistent_tooltips: Signal::from(true),
///                 persistent_tooltip_side: Signal::from(FloatingActionsMenuTooltipSide::Left),
///                 right: Signal::from(None),
///                 bottom: Signal::from(None),
///                 viewport_fixed: Signal::from(false),
///                 ..Default::default()
///             }
///         >
///             <FloatingActionsMenuItem tooltip="Email".to_string() icon=icondata::AiMailOutlined testid="fam-email-left" />
///         </FloatingActionsMenu>
///     </div>
/// }
/// ```
///
/// ## Persistent tooltips (right)
/// Labels stay visible to the right of each action while the menu is open. Set `persistent_tooltips` and `persistent_tooltip_side: Right`.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingActionsMenu, FloatingActionsMenuConfig, FloatingActionsMenuItem};
/// use leptos::prelude::*;
/// use orbital_base_components::FloatingActionsMenuTooltipSide;
/// view! {
///     <div data-testid="floating-actions-menu-tooltips-right" style="position: relative; width: 100%; min-height: 280px; display: flex; align-items: flex-end; justify-content: flex-end; overflow: visible; padding: var(--orb-space-inline-lg);">
///         <FloatingActionsMenu
///             config=FloatingActionsMenuConfig {
///                 aria_label: "Share".to_string(),
///                 persistent_tooltips: Signal::from(true),
///                 persistent_tooltip_side: Signal::from(FloatingActionsMenuTooltipSide::Right),
///                 right: Signal::from(None),
///                 bottom: Signal::from(None),
///                 viewport_fixed: Signal::from(false),
///                 ..Default::default()
///             }
///         >
///             <FloatingActionsMenuItem tooltip="Email".to_string() icon=icondata::AiMailOutlined testid="fam-email-right" />
///         </FloatingActionsMenu>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "floating-actions-menu",
    preview_label = "Floating Actions Menu",
    preview_icon = icondata::AiPlusCircleOutlined,
)]
#[component]
pub fn FloatingActionsMenu(
    /// Direction, labeling, tooltip, and fixed-position options.
    config: FloatingActionsMenuConfig,
    /// Optional controlled open binding.
    #[prop(optional, into)]
    open: OpenBind,
    /// Optional CSS class on the menu root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Icon shown on the closed trigger.
    #[prop(optional, into)]
    trigger_icon: MaybeProp<IconData>,
    /// Icon shown on the open trigger.
    #[prop(optional, into)]
    open_icon: MaybeProp<IconData>,
    /// Secondary action items.
    children: Children,
) -> impl IntoView {
    view! {
        <FloatingActionsMenuRoot
            config=config
            open=open
            class=class
            trigger_icon=trigger_icon
            open_icon=open_icon
        >
            {children()}
        </FloatingActionsMenuRoot>
    }
}
