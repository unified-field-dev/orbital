/// Public URL prefix for GitHub Pages project sites (e.g. `/orbital`).
///
/// Set at compile time via `LEPTOS_BASE_PATH=/orbital`. Local dev leaves this empty.
pub fn preview_site_base() -> &'static str {
    option_env!("LEPTOS_BASE_PATH").unwrap_or("")
}

pub fn preview_asset_path(relative: &str) -> String {
    let base = preview_site_base();
    let relative = relative.trim_start_matches('/');
    if base.is_empty() {
        format!("/{relative}")
    } else {
        format!("{base}/{relative}")
    }
}
