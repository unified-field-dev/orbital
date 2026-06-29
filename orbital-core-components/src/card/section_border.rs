use leptos::prelude::*;

use super::styles::card_section_border_styles;

/// Horizontal separator between [`Card`] slot regions.
///
/// Insert between stacked [`CardContent`], [`CardPreview`], [`CardFooter`], or other card slots
/// when a flush internal rule is needed. Distinct from [`Divider`](crate::Divider), which targets
/// page sections and toolbar clusters.
#[component]
pub fn CardSectionBorder() -> impl IntoView {
    let style_sheet = card_section_border_styles();

    view! {
        <style>{style_sheet}</style>
        <div
            class="orbital-card-section-border"
            role="separator"
            aria-orientation="horizontal"
        />
    }
}
