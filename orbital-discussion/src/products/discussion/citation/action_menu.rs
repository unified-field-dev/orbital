use leptos::prelude::*;

use crate::{
    use_discussion, CitationRenderContext, DiscussionCitation, DiscussionMenuItem, DiscussionReply,
};

use super::super::shared::DiscussionOverflowMenu;

/// Citation overflow menu with built-in open/copy actions and optional host extras.
#[component]
pub fn DiscussionCitationActionMenu(
    reply: DiscussionReply,
    citation: DiscussionCitation,
    index: usize,
    has_url: bool,
) -> impl IntoView {
    let ctx = use_discussion();
    let row_id = citation.id.clone();
    let menu_testid = format!("discussion-citation-menu-{row_id}");
    let render_ctx = CitationRenderContext::new(reply, citation.clone(), index);

    let custom_menu = ctx.renderers.with_value(|renderers| {
        renderers
            .citation_menu
            .as_ref()
            .map(|view| view(render_ctx.clone()))
    });

    if let Some(custom_view) = custom_menu {
        return view! {
            <div class="orbital-discussion__citation-menu" data-testid=menu_testid>
                {custom_view}
            </div>
        }
        .into_any();
    }

    let mut menu_items = Vec::new();
    if has_url {
        menu_items.push(DiscussionMenuItem {
            id: "open_link".into(),
            label: "Open link".into(),
            disabled: None,
        });
        menu_items.push(DiscussionMenuItem {
            id: "copy_url".into(),
            label: "Copy URL".into(),
            disabled: None,
        });
    }

    let extras = ctx.renderers.with_value(|renderers| {
        renderers
            .citation_menu_extras
            .as_ref()
            .map(|view| view(render_ctx))
            .unwrap_or_default()
    });
    menu_items.extend(extras);

    if menu_items.is_empty() {
        return view! { <span></span> }.into_any();
    }

    let citation_for_select = citation;
    let on_select = Callback::new(move |action: String| {
        ctx.events.with_value(|events| {
            events.notify_citation_action(citation_for_select.clone(), action);
        });
    });

    view! {
        <div class="orbital-discussion__citation-menu" data-testid=menu_testid>
            <DiscussionOverflowMenu
                items=menu_items
                on_select=on_select
                action_testid_prefix="discussion-citation-action".to_string()
                aria_label="Citation actions".to_string()
            />
        </div>
    }
    .into_any()
}
