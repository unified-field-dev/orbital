use leptos::{html, prelude::*};
use orbital_theme::ThemeInjection;

use super::portal::Portal;

/// Portal wrapper that scopes portaled UI to the current Orbital theme.
#[component]
pub fn ThemedPortal(
    #[prop(default = false.into(), into)] immediate: Signal<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] mount: Option<web_sys::Element>,
    #[prop(default = None)] mount_ref: Option<NodeRef<html::Div>>,
    children: Children,
) -> impl IntoView {
    let theme = expect_context::<ThemeInjection>();
    let theme_id = StoredValue::new(theme.id());

    let themed_children = view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-theme-provider".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            data-orbital-theme-id=move || theme_id.get_value()
        >
            {children()}
        </div>
    };

    if let Some(mount_el) = mount {
        view! {
            <Portal immediate=immediate mount=mount_el>
                {themed_children}
            </Portal>
        }
    } else if mount_ref.is_some() {
        view! {
            <Portal immediate=immediate mount_ref=mount_ref>
                {themed_children}
            </Portal>
        }
    } else {
        view! {
            <Portal immediate=immediate>
                {themed_children}
            </Portal>
        }
    }
}
