use leptos::prelude::*;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use crate::components::{
    AppBar, AppBarLeading, ContentContainer, EmptyState, EMPTYSTATE_SAD_DOG_ILLUSTRATION,
};

/// Centered 404 page with a minimal app bar surface.
#[component_doc]
#[component]
pub fn NotFoundPage() -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Page {
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            background: var(--orb-color-surface-subtle);
        }

        .Center {
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 32px 16px;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.page>
            <AppBar>
                <AppBarLeading slot>
                    <span>"Not Found"</span>
                </AppBarLeading>
            </AppBar>
            <ContentContainer max_width="720px">
                <div class=class_names.center>
                    <EmptyState
                        message="Page not found"
                        description="The page you requested does not exist or may have moved."
                        illustration_src=EMPTYSTATE_SAD_DOG_ILLUSTRATION
                        illustration_alt="Sad dog illustration"
                    />
                </div>
            </ContentContainer>
        </div>
    }
}
