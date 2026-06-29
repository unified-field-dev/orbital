#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    #[cfg(feature = "hydrate")]
    leptos::mount::hydrate_body(orbital_preview_app::App);
}
