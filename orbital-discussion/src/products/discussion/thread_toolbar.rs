use leptos::prelude::*;
use orbital_core_components::Select;

use crate::{use_discussion, use_discussion_locale, DiscussionViewMode};

fn view_mode_to_value(mode: DiscussionViewMode) -> &'static str {
    match mode {
        DiscussionViewMode::Tree => "tree",
        DiscussionViewMode::Flat => "flat",
        DiscussionViewMode::Compact => "compact",
    }
}

fn view_mode_from_value(value: &str) -> DiscussionViewMode {
    match value {
        "flat" => DiscussionViewMode::Flat,
        "compact" => DiscussionViewMode::Compact,
        _ => DiscussionViewMode::Tree,
    }
}

/// Default thread toolbar with view mode picker.
#[component]
pub fn DiscussionDefaultThreadToolbar() -> impl IntoView {
    let ctx = use_discussion();
    let locale = use_discussion_locale();
    let select_value = RwSignal::new(view_mode_to_value(ctx.view_mode.get_untracked()).to_string());

    Effect::new(move |_| {
        let value = view_mode_to_value(ctx.view_mode.get());
        if select_value.get_untracked() != value {
            select_value.set(value.to_string());
        }
    });

    Effect::new(move |_| {
        ctx.set_view_mode
            .set(view_mode_from_value(&select_value.get()));
    });

    let picker_label = Memo::new(move |_| locale.get().view_picker_label.clone());
    let tree_label = Memo::new(move |_| locale.get().view_mode_tree_label.clone());
    let flat_label = Memo::new(move |_| locale.get().view_mode_flat_label.clone());
    let compact_label = Memo::new(move |_| locale.get().view_mode_compact_label.clone());

    view! {
        <div class="orbital-discussion__toolbar">
            <label class="orbital-discussion__toolbar-label" for="discussion-view-mode-select">
                {move || picker_label.get()}
            </label>
            <div data-testid="discussion-view-mode-select">
                <Select bind=select_value>
                    <option value="tree">{move || tree_label.get()}</option>
                    <option value="flat">{move || flat_label.get()}</option>
                    <option value="compact">{move || compact_label.get()}</option>
                </Select>
            </div>
        </div>
    }
}
