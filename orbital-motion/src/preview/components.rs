//! Preview-only UI helpers (avoids depending on orbital-core-components).

use leptos::prelude::*;
use orbital_style::inject_style;

/// Minimal preview button matching catalog interaction patterns.
#[component]
pub fn PreviewButton(
    #[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-motion-preview-button", preview_button_styles());

    view! {
        <button type="button" class="orbital-motion-preview-button" on:click=move |ev| {
            if let Some(cb) = on_click {
                cb.run(ev);
            }
        }>
            {children()}
        </button>
    }
}

pub fn preview_button_styles() -> &'static str {
    r#"
.orbital-motion-preview-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 32px;
    padding: 5px var(--orb-space-inline-md);
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-md);
    cursor: pointer;
    transition: background 120ms var(--orb-motion-ease-standard), border-color 120ms var(--orb-motion-ease-standard);
}
.orbital-motion-preview-button:hover {
    background: var(--orb-color-surface-canvas-hover);
    border-color: var(--orb-color-border-default-hover);
}
.orbital-motion-preview-button:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: 2px;
}
"#
}
