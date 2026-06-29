use leptos::prelude::*;
use orbital_base_components::BaseLoadingBarProvider;

/// Mounts a teleported top loading bar and provides [`LoadingBarInjection`] to descendants.
///
/// Wrap your app shell once. Descendants call `LoadingBarInjection::expect_context()` to imperatively drive `start`, `finish`, and `error`. See [`super::loading_bar::LoadingBar`] for full usage and examples.
#[component]
pub fn LoadingBarProvider(
    /// Subtree that can access [`LoadingBarInjection`] via context.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseLoadingBarProvider>
            {children()}
        </BaseLoadingBarProvider>
    }
}
