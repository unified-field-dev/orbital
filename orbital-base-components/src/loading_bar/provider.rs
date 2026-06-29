use leptos::prelude::*;

use crate::overlay::ThemedPortal;
use crate::ComponentRef;

use super::bar::{BaseLoadingBar, LoadingBarRef};

#[derive(Clone)]
pub struct LoadingBarInjection {
    loading_bar_ref: ComponentRef<LoadingBarRef>,
}

impl LoadingBarInjection {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }

    /// Callback function for loading bar to start loading.
    pub fn start(&self) {
        if let Some(loading_bar_ref) = self.loading_bar_ref.get_untracked() {
            loading_bar_ref.start();
        }
    }

    /// The callback function when the loading bar finishes loading.
    pub fn finish(&self) {
        if let Some(loading_bar_ref) = self.loading_bar_ref.get_untracked() {
            loading_bar_ref.finish();
        }
    }

    /// Callback function for loading bar error.
    pub fn error(&self) {
        if let Some(loading_bar_ref) = self.loading_bar_ref.get_untracked() {
            loading_bar_ref.error();
        }
    }
}

#[component]
pub fn BaseLoadingBarProvider(children: Children) -> impl IntoView {
    let loading_bar_ref = ComponentRef::<LoadingBarRef>::default();
    provide_context(LoadingBarInjection {
        loading_bar_ref: loading_bar_ref.clone(),
    });

    view! {
        {children()}
        <ThemedPortal immediate=true>
            <BaseLoadingBar comp_ref=loading_bar_ref />
        </ThemedPortal>
    }
}
