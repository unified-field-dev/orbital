use leptos::prelude::*;

/// Default `"N of M"` step indicator when a tour step has no custom footer.
pub fn default_tour_footer(
    active_index: ReadSignal<usize>,
    step_count: ReadSignal<usize>,
) -> impl IntoView {
    view! {
        <div class="orbital-spotlight__footer" data-testid="spotlight-footer">
            {move || format!("{} of {}", active_index.get() + 1, step_count.get())}
        </div>
    }
}
