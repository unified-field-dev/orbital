use chrono::NaiveDate;
use leptos::prelude::*;
use orbital_core_components::Divider;

/// Chronological date break between reply rows in Flat/Compact modes.
#[component]
pub fn DiscussionDateDivider(date: NaiveDate) -> impl IntoView {
    let label = date.format("%B %d, %Y").to_string();
    let label_attr = label.clone();

    view! {
        <div class="orbital-discussion__date-divider" role="separator" data-date-divider=label_attr>
            <Divider class="orbital-discussion__date-divider-line".to_string() />
            <span class="orbital-discussion__date-divider-label">{label}</span>
            <Divider class="orbital-discussion__date-divider-line".to_string() />
        </div>
    }
}
