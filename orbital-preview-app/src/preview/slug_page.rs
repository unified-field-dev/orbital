use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_params_map};
use leptos_router::params::ParamsMap;
use orbital::components::{Body1, Title3};

use super::collect_preview_registrations;

/// Resolve the preview slug from the splat param or the root-mounted pathname.
pub(crate) fn preview_slug_from_route(params: &ParamsMap, pathname: &str) -> String {
    if let Some(slug) = params.get("slug") {
        let slug = normalize_preview_slug(&slug);
        if !slug.is_empty() {
            return slug;
        }
    }

    let path = pathname.trim();
    if path.is_empty() || path == "/" {
        return String::new();
    }

    normalize_preview_slug(path.trim_start_matches('/'))
}

pub(crate) fn normalize_preview_slug(raw: &str) -> String {
    raw.trim()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_string()
}

/// Map legacy sub-component slugs (e.g. `combobox-option`) to a registered preview page.
pub(crate) fn resolve_preview_slug(slug: &str, registered: &[&str]) -> String {
    if registered.contains(&slug) {
        return slug.to_string();
    }

    for candidate in subcomponent_slug_aliases(slug) {
        if registered.contains(&candidate.as_str()) {
            return candidate;
        }
    }

    slug.to_string()
}

fn subcomponent_slug_aliases(slug: &str) -> Vec<String> {
    let mut aliases = Vec::new();

    for suffix in ["-option-group", "-option", "-prefix", "-suffix"] {
        if let Some(base) = slug.strip_suffix(suffix) {
            if !base.is_empty() {
                aliases.push(base.to_string());
            }
        }
    }

    if slug == "spin-button" {
        aliases.push("numeric-stepper".to_string());
    }

    aliases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_subcomponent_slug_to_parent_preview() {
        let registered = &["combobox", "auto-complete", "select"];
        assert_eq!(
            resolve_preview_slug("combobox-option", registered),
            "combobox"
        );
        assert_eq!(
            resolve_preview_slug("combobox-option-group", registered),
            "combobox"
        );
        assert_eq!(resolve_preview_slug("combobox", registered), "combobox");
        assert_eq!(
            resolve_preview_slug("unknown-widget", registered),
            "unknown-widget"
        );
    }

    #[test]
    fn resolves_legacy_spin_button_slug() {
        let registered = &["numeric-stepper", "slider"];
        assert_eq!(
            resolve_preview_slug("spin-button", registered),
            "numeric-stepper"
        );
    }
}

#[component]
pub fn PreviewSlugPage() -> impl IntoView {
    let params = use_params_map();
    let location = use_location();

    let slug = Memo::new(move |_| {
        let pathname = location.pathname.get();
        preview_slug_from_route(&params.get(), &pathname)
    });

    move || {
        let current = slug.get();
        let registrations = collect_preview_registrations();
        let registered_slugs: Vec<&str> = registrations.iter().map(|item| item.slug).collect();
        let resolved = resolve_preview_slug(&current, &registered_slugs);
        let registration = registrations.into_iter().find(|item| item.slug == resolved);

        match registration {
            Some(item) => (item.render)().into_any(),
            None => view! {
                <div data-testid="preview-not-found">
                    <Title3>"Preview not found"</Title3>
                    <Body1>{format!("No preview registered for slug: {current}")}</Body1>
                </div>
            }
            .into_any(),
        }
    }
}
