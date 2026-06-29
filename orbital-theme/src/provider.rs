use leptos::{context::Provider, prelude::*};
use orbital_style::inject_dynamic_style;

use crate::context::{scoped_css, ThemeInjection};
use crate::fonts::inject_font_faces;
use crate::Direction;
use crate::Theme;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Per-app-tree theme scope ids (stable across SSR and hydration, unlike global `next_id()`).
#[derive(Clone, Default)]
struct ThemeScopeCounter(Arc<AtomicUsize>);

fn alloc_theme_scope_id() -> String {
    let counter = use_context::<ThemeScopeCounter>().unwrap_or_else(|| {
        let counter = ThemeScopeCounter(Arc::new(AtomicUsize::new(0)));
        provide_context(counter.clone());
        counter
    });
    counter.0.fetch_add(1, Ordering::Relaxed).to_string()
}

/// Injects Orbital CSS variables for components.
#[component]
pub fn OrbitalThemeProvider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] theme: Option<RwSignal<Theme>>,
    #[prop(optional, into)] dir: Option<RwSignal<Direction>>,
    children: Children,
) -> impl IntoView {
    let theme = theme.unwrap_or_else(|| RwSignal::new(Theme::light()));
    let theme_id = alloc_theme_scope_id();
    let id = StoredValue::new(theme_id.clone());
    let style_mount_id = format!("orbital-theme-{}", id.get_value());

    inject_font_faces();

    inject_dynamic_style(style_mount_id.clone(), move || {
        let mut css_vars = String::new();
        theme.with(|t| t.write_css_vars(&mut css_vars));
        scoped_css(&id.get_value(), &css_vars)
    });

    #[cfg(not(feature = "ssr"))]
    {
        let cleanup_id = style_mount_id;
        Owner::on_cleanup(move || {
            if let Ok(Some(style)) =
                document().query_selector(&format!("head style#orbital-style-{cleanup_id}"))
            {
                style.remove();
            }
        });
    }

    let injection = ThemeInjection::new(theme, dir, theme_id);

    let root_class = Signal::derive(move || {
        let mut parts = vec!["orbital-theme-provider".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <Provider value=injection>
            <div
                class=root_class
                data-orbital-theme-id=id.get_value()
                dir=move || dir.map(|d| d.get().as_str())
                style="height: 100%; font-family: var(--orb-type-family-sans);"
            >
                {children()}
            </div>
        </Provider>
    }
}
