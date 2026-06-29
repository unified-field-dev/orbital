use std::path::{Path, PathBuf};
use std::time::Duration;

use axum::Router;
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use orbital_preview_app::preview::collect_preview_slugs_for_export;
use orbital_preview_app::preview_site_base;
use tower_http::services::ServeDir;

// `current_thread` keeps Leptos sandboxed-arena Owner cleanup on the same thread
// that created SendWrapper-backed effects. Multi-thread Tokio workers panic with:
// "Dropped SendWrapper<T> variable from a thread different to the one it has been created with."
#[tokio::main(flavor = "current_thread")]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("logger");

    // CSSTransition uses `spawn_local` during SSR. Tokio's `spawn_local` requires a
    // LocalSet on every worker thread; the futures thread-local LocalPool does not.
    // Register it before leptos_axum's init_tokio (which we ignore if AlreadySet).
    any_spawner::Executor::init_futures_executor().expect("futures executor");

    let conf = get_configuration(Some("Cargo.toml"))
        .or_else(|_| get_configuration(None))
        .expect("leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(orbital_preview_app::App);

    // Root-mounted `/*slug` would otherwise SSR-wrap `/pkg/*`, `/fonts/*`, and `/preview-assets/*`.
    let site_root = PathBuf::from(leptos_options.site_root.as_ref());
    let pkg_dir = site_root.join(leptos_options.site_pkg_dir.as_ref());
    let fonts_dir = site_root.join("fonts");
    let preview_assets_dir = site_root.join("orbital").join("preview-assets");

    let app = build_preview_router(
        leptos_options.clone(),
        routes,
        pkg_dir,
        fonts_dir,
        preview_assets_dir,
    );

    if std::env::var("ORBITAL_EXPORT_STATIC").is_ok() {
        export_static_html(&leptos_options, app, addr).await;
        log::info!(
            "Static export complete → {}",
            leptos_options.site_root.as_ref()
        );
        return;
    }

    log::info!("Orbital preview server listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind preview server");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve preview server");
}

fn build_preview_router(
    leptos_options: leptos::config::LeptosOptions,
    routes: Vec<leptos_axum::AxumRouteListing>,
    pkg_dir: PathBuf,
    fonts_dir: PathBuf,
    preview_assets_dir: PathBuf,
) -> Router {
    let leptos_options_for_routes = leptos_options.clone();
    let leptos_options_state = leptos_options.clone();

    let leptos_router = Router::new()
        .leptos_routes(&leptos_options, routes, move || {
            orbital_preview_app::shell(leptos_options_for_routes.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(
            orbital_preview_app::shell,
        ))
        .with_state(leptos_options_state);

    let base = preview_site_base();
    if base.is_empty() {
        Router::new()
            .nest_service("/pkg", ServeDir::new(pkg_dir))
            .nest_service("/fonts", ServeDir::new(fonts_dir))
            .nest_service("/preview-assets", ServeDir::new(preview_assets_dir))
            .merge(leptos_router)
    } else {
        let pkg_path = format!("{base}/pkg");
        let fonts_path = format!("{base}/fonts");
        let preview_assets_path = format!("{base}/preview-assets");
        Router::new()
            .nest_service(&pkg_path, ServeDir::new(pkg_dir))
            .nest_service(&fonts_path, ServeDir::new(fonts_dir))
            .nest_service(&preview_assets_path, ServeDir::new(preview_assets_dir))
            .merge(leptos_router)
    }
}

/// Crawl the live SSR server and write `{slug}/index.html` pages for GitHub Pages.
async fn export_static_html(
    leptos_options: &leptos::config::LeptosOptions,
    app: Router,
    addr: std::net::SocketAddr,
) {
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind preview server for static export");
    let server = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .expect("serve preview server for static export");
    });

    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = reqwest::Client::new();
    let base = preview_site_base();
    let site_root = PathBuf::from(leptos_options.site_root.as_ref());

    let mut slugs: Vec<String> = vec![String::new()];
    slugs.extend(collect_preview_slugs_for_export());

    for slug in slugs {
        let request_path = if slug.is_empty() {
            if base.is_empty() {
                "/".to_string()
            } else {
                base.to_string()
            }
        } else if base.is_empty() {
            format!("/{slug}")
        } else {
            format!("{base}/{slug}")
        };

        let url = format!("http://{addr}{request_path}");
        log::info!("Exporting {url}");

        let response = client
            .get(&url)
            .send()
            .await
            .unwrap_or_else(|err| panic!("failed to fetch {url}: {err}"));
        if !response.status().is_success() {
            panic!(
                "failed to export {url}: HTTP {}",
                response.status().as_u16()
            );
        }

        let html = response
            .text()
            .await
            .unwrap_or_else(|err| panic!("failed to read body for {url}: {err}"));

        write_crawled_page(&site_root, &slug, &html)
            .await
            .unwrap_or_else(|err| panic!("failed to write export for {url}: {err}"));
    }

    server.abort();
}

async fn write_crawled_page(site_root: &Path, slug: &str, html: &str) -> std::io::Result<()> {
    if slug.is_empty() {
        tokio::fs::write(site_root.join("index.html"), html).await
    } else {
        let dir = site_root.join(slug);
        tokio::fs::create_dir_all(&dir).await?;
        tokio::fs::write(dir.join("index.html"), html).await
    }
}
