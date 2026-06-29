use super::spotlight::{use_spotlight_rect, SpotlightBackdropHole, SpotlightBackdropPanels};
use super::types::BackdropMode;
use leptos::prelude::*;

/// Headless full-viewport scrim — structure and click handling only.
#[component]
pub fn BaseBackdrop(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, default = BackdropMode::Full)] mode: BackdropMode,
    #[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
) -> AnyView {
    match mode {
        BackdropMode::Full => render_full_backdrop(class, on_click, children).into_any(),
        BackdropMode::Spotlight { anchor_id, padding } => {
            render_spotlight_backdrop(class, anchor_id, padding, on_click, children).into_any()
        }
    }
}

fn render_full_backdrop(
    class: MaybeProp<String>,
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    children: Option<Children>,
) -> impl IntoView {
    let click_handler = on_click;
    let on_click = move |event: leptos::ev::MouseEvent| {
        if let Some(handler) = click_handler.as_ref() {
            handler.run(event);
        }
    };

    let interactive = click_handler.is_some();

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-backdrop".to_string()];
                if !interactive {
                    parts.push("orbital-backdrop--passive".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            aria-hidden="true"
            on:click=on_click
        >
            {children.map(|c| c())}
        </div>
    }
}

fn render_spotlight_backdrop(
    class: MaybeProp<String>,
    anchor_id: Signal<Option<String>>,
    padding: u32,
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    children: Option<Children>,
) -> impl IntoView {
    let (rect, sync, ensure_listeners) = use_spotlight_rect(anchor_id, padding);
    let interactive = on_click.is_some();

    #[cfg(feature = "ssr")]
    {
        let _ = (&sync, &ensure_listeners);
    }

    #[cfg(not(feature = "ssr"))]
    {
        #[allow(clippy::clone_on_copy)]
        let sync = sync.clone();
        #[allow(clippy::clone_on_copy)]
        let ensure_listeners = ensure_listeners.clone();
        Effect::new(move |_| {
            let _ = anchor_id.get();
            sync();
            ensure_listeners();
            request_animation_frame({
                let sync = sync.clone();
                move || sync()
            });
        });
    }

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-backdrop-spotlight".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            aria-hidden="true"
        >
            {if interactive {
                let handler = on_click.expect("interactive spotlight backdrop requires on_click");
                view! { <SpotlightBackdropPanels rect=rect on_click=handler /> }.into_any()
            } else {
                view! { <SpotlightBackdropHole rect=rect /> }.into_any()
            }}
            {children.map(|c| c())}
        </div>
    }
}
