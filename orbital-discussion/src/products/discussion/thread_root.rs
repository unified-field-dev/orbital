use leptos::prelude::*;
use orbital_core_components::{Material, MaterialElevation, MaterialVariant, ScrollArea};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::DiscussionSlots;

use super::styles::{density_modifier_class, discussion_styles};
use super::DiscussionDefaultThreadToolbar;

/// Scrollable shell for a discussion thread body.
#[component]
pub fn DiscussionThreadRoot(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] slots: DiscussionSlots,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-discussion", discussion_styles());

    let theme_options = use_theme_options();
    let density_class =
        Memo::new(move |_| density_modifier_class(theme_options.get().density).to_string());

    let root_class = move || {
        let mut classes = vec!["orbital-discussion".to_string()];
        let density = density_class.get();
        if !density.is_empty() {
            classes.push(density.to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    };

    view! {
        <div data-orbital-discussion="" class=root_class>
            {if let Some(toolbar) = slots.thread_toolbar {
                (toolbar.children)().into_any()
            } else {
                view! { <DiscussionDefaultThreadToolbar /> }.into_any()
            }}
            <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
                <ScrollArea
                    class="orbital-discussion__scroll"
                    style="display: block; width: 100%; height: 480px; box-sizing: border-box;"
                >
                    {children()}
                </ScrollArea>
            </Material>
            {slots.composer.map(|composer| view! {
                <div class="orbital-discussion__composer-slot">{(composer.children)()}</div>
            })}
        </div>
    }
}
