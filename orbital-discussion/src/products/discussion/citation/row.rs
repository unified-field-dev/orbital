use leptos::prelude::*;
use orbital_core_components::Link;

use crate::{use_discussion, CitationRenderContext, DiscussionCitation, DiscussionReply};

use super::action_menu::DiscussionCitationActionMenu;

/// Single citation row with title, url, excerpt, affordance, and overflow menu.
#[component]
pub fn DiscussionCitationRow(
    reply: DiscussionReply,
    citation: DiscussionCitation,
    index: usize,
) -> impl IntoView {
    let ctx = use_discussion();
    let row_id = citation.id.clone();
    let row_testid = format!("discussion-citation-row-{}", row_id);
    let affordance_testid = format!("discussion-citation-affordance-{}", row_id);
    let title = citation.title.clone();
    let url = citation.url.clone();
    let excerpt = citation.excerpt.clone();
    let has_url = url.is_some();

    let render_ctx = CitationRenderContext::new(reply.clone(), citation.clone(), index);

    let affordance_view = ctx.renderers.with_value(|renderers| {
        renderers
            .citation_affordance_view
            .as_ref()
            .and_then(|view| view(render_ctx))
    });

    view! {
        <div
            class="orbital-discussion__citation-row"
            id=row_testid.clone()
            data-testid=row_testid
        >
            <div class="orbital-discussion__citation-row-main">
                <span class="orbital-discussion__citation-index">{format!("[{index}]")}</span>
                <div class="orbital-discussion__citation-content">
                    <div class="orbital-discussion__citation-title-row">
                        <span class="orbital-discussion__citation-title">{title.clone()}</span>
                        {affordance_view.map(|view| view! {
                            <div
                                class="orbital-discussion__citation-affordance"
                                data-testid=affordance_testid
                            >
                                {view}
                            </div>
                        })}
                        <DiscussionCitationActionMenu
                            reply=reply.clone()
                            citation=citation.clone()
                            index=index
                            has_url=has_url
                        />
                    </div>
                    {url.map(|href| view! {
                        <Link href=href.clone() class="orbital-discussion__citation-url".to_string()>
                            {href}
                        </Link>
                    })}
                    {excerpt.map(|text| view! {
                        <p class="orbital-discussion__citation-excerpt">{text}</p>
                    })}
                </div>
            </div>
        </div>
    }
}
