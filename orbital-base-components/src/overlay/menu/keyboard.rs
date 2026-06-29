use leptos::{ev, prelude::*};

use crate::overlay::OverlayDismiss;

/// Keyboard navigation wrapper for menu panels (Escape dismiss; item roving tabindex in core).
#[component]
pub fn MenuKeyboardRegion(children: Children) -> impl IntoView {
    let on_keydown = move |event: ev::KeyboardEvent| {
        if event.key() == "Escape" {
            if let Some(dismiss) = use_context::<OverlayDismiss>() {
                dismiss.close.run(());
            }
        }
    };

    view! {
        <div class="orbital-menu-keyboard-region" tabindex="-1" on:keydown=on_keydown>
            {children()}
        </div>
    }
}
