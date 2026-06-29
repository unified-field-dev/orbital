use leptos::prelude::*;
use turf::inline_style_sheet_values;

use crate::components::{
    AppBar, AppBarMaterial, MaterialCorners, MaterialElevation, MaterialVariant,
};

/// Optional header region for [`PreviewShell`].
#[slot]
pub struct PreviewAppBar {
    pub children: Children,
}

/// Minimal shell for the component preview catalog (`:3010`).
#[component]
pub fn PreviewShell(
    #[prop(optional)] preview_app_bar: Option<PreviewAppBar>,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Shell {
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            background: var(--orb-color-surface-subtle);
        }

        .AppBarRegion {
            flex-shrink: 0;
        }

        .Content {
            flex: 1;
            min-height: 0;
            overflow: auto;
            background: var(--orb-color-surface-canvas);
            margin: 12px;
            border-radius: var(--orb-radius-xl);
            box-shadow: var(--orb-elev-raised-sm);
            padding: 24px;
            box-sizing: border-box;
        }
    };

    let app_bar_view = preview_app_bar.map(|slot| {
        view! {
            <div class=class_names.app_bar_region>
                {(slot.children)()}
            </div>
        }
    });

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.shell data-testid="preview-shell">
            {app_bar_view}
            <main class=class_names.content>
                {children()}
            </main>
        </div>
    }
}

/// Empty app bar placeholder for preview pages that register the shell region.
#[component]
pub fn PreviewAppBarPlaceholder() -> impl IntoView {
    view! {
        <AppBar>
            <AppBarMaterial
                variant=MaterialVariant::Shell
                elevation=MaterialElevation::Flat
                corners=MaterialCorners::Square
                slot
            />
        </AppBar>
    }
}
