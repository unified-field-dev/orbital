use leptos::prelude::*;
use orbital_base_components::BaseRatingItem;

use super::injection::RatingInjection;

#[component]
pub fn RatingItem(value: u8) -> impl IntoView {
    let rating = RatingInjection::expect_context();

    let displayed_value = Memo::new(move |_| {
        rating
            .hovered_value
            .get()
            .unwrap_or_else(|| rating.value.get().unwrap_or_default())
    });

    view! {
        <BaseRatingItem
            value=value
            displayed_value=displayed_value
            step=rating.step
            size=rating.size
            color=rating.color
            interactive=Signal::from(rating.interactive)
            name=Signal::derive(move || rating.name.get())
        />
    }
}
