//! Bare debug preview page — default example only, no catalog shell.

use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_params_map};
use orbital::components::{Body1, Title3};
use orbital_base_components::PreviewRenderMode;
use orbital_theme::{OrbitalThemeProvider, Theme};

use super::collect_preview_registrations;
use super::slug_page::{normalize_preview_slug, preview_slug_from_route};

fn debug_slug_from_route(params: &leptos_router::params::ParamsMap, pathname: &str) -> String {
    if let Some(slug) = params.get("slug") {
        let slug = normalize_preview_slug(&slug);
        if !slug.is_empty() {
            return slug;
        }
    }

    normalize_preview_slug(
        pathname
            .strip_prefix("/debug/")
            .or_else(|| pathname.strip_prefix("/debug"))
            .unwrap_or(pathname),
    )
}

#[component]
pub fn DebugBarePreviewPage() -> impl IntoView {
    provide_context(PreviewRenderMode::BareDefault);

    let params = use_params_map();
    let location = use_location();
    let theme = RwSignal::new(Theme::dark());

    let slug = Memo::new(move |_| {
        let pathname = location.pathname.get();
        let from_params = debug_slug_from_route(&params.get(), &pathname);
        if !from_params.is_empty() {
            return from_params;
        }
        preview_slug_from_route(&params.get(), &pathname)
    });

    let skip_theme = Memo::new(move |_| location.query.get().get("notheme").is_some());

    view! {
        <Show
            when=move || !skip_theme.get()
            fallback=move || view! { <DebugBarePreviewBody slug=slug /> }
        >
            <OrbitalThemeProvider theme=theme>
                <DebugBarePreviewBody slug=slug />
            </OrbitalThemeProvider>
        </Show>
    }
}

#[component]
fn DebugBarePreviewBody(slug: Memo<String>) -> impl IntoView {
    move || {
        let current = slug.get();
        let registration = collect_preview_registrations()
            .into_iter()
            .find(|item| item.slug == current);

        match registration {
            Some(item) => view! {
                <main data-testid="debug-bare-root" style="min-height: 100vh; padding: 16px; box-sizing: border-box;">
                    {(item.render)()}
                </main>
            }
            .into_any(),
            None => view! {
                <main data-testid="debug-bare-root" style="min-height: 100vh; padding: 16px; box-sizing: border-box;">
                    <div data-testid="debug-bare-not-found">
                        <Title3>"Preview not found"</Title3>
                        <Body1>{format!("No preview registered for slug: {current}")}</Body1>
                    </div>
                </main>
            }
            .into_any(),
        }
    }
}
