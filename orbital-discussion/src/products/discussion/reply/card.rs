use leptos::prelude::*;
use orbital_core_components::{Card, CardContent, MaterialElevation, MaterialVariant};

use crate::{
    resolve_reply_surface, DiscussionAppearance, DiscussionFeatures, DiscussionReply,
    DiscussionReplyGraph, DiscussionReplySurface,
};

use super::super::citation::DiscussionCitationList;
use super::{
    body::DiscussionReplyBody, footer::DiscussionReplyFooter, header::DiscussionReplyHeader,
};

/// Card shell composing header, body, footer for one reply.
#[component]
pub fn DiscussionReplyCard(
    reply: DiscussionReply,
    graph: Memo<DiscussionReplyGraph>,
    features: DiscussionFeatures,
    surface: DiscussionReplySurface,
    is_collapsed: bool,
    #[prop(default = DiscussionAppearance::Surface)] appearance: DiscussionAppearance,
) -> impl IntoView {
    let surface_class = format!(
        "orbital-discussion__reply-card orbital-discussion__reply-card--{}",
        surface.class_suffix()
    );
    let card_variant = if appearance == DiscussionAppearance::Plain {
        MaterialVariant::Outlined
    } else {
        MaterialVariant::Solid
    };
    let elevation = if appearance == DiscussionAppearance::Plain {
        MaterialElevation::Flat
    } else {
        MaterialElevation::Resting
    };

    let reply_for_header = reply.clone();
    let citations = reply.citations.clone();
    let show_citations = features.contains(DiscussionFeatures::CITATIONS) && !citations.is_empty();
    let reply_for_body = reply.clone();
    let reply_for_citations = reply.clone();
    let reply_for_footer = reply;

    view! {
        <Card
            class=surface_class
            variant=card_variant
            elevation=elevation
        >
            <CardContent class="orbital-discussion__reply-card-content".to_string()>
                <DiscussionReplyHeader
                    reply=reply_for_header
                    graph=graph
                    is_collapsed=is_collapsed
                />
                {(!is_collapsed).then(|| view! {
                    <>
                        <DiscussionReplyBody reply=reply_for_body features=features />
                        {show_citations.then(|| view! {
                            <DiscussionCitationList reply=reply_for_citations citations=citations />
                        })}
                        <DiscussionReplyFooter reply=reply_for_footer />
                    </>
                })}
            </CardContent>
        </Card>
    }
}

/// Resolve surface tint from reply data and optional viewer id signal.
pub fn reply_surface_for(
    reply: &DiscussionReply,
    current_user_id: Signal<Option<String>>,
) -> DiscussionReplySurface {
    let user_id = current_user_id.get();
    resolve_reply_surface(reply, user_id.as_deref())
}
