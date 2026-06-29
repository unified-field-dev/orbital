use std::collections::HashSet;

use leptos::prelude::*;
use orbital_core_components::{Accordion, AccordionHeader, AccordionItem};

use crate::{DiscussionReasoningPart, DiscussionReasoningStatus};

/// Collapsible thinking / reasoning trace block.
#[component]
pub fn DiscussionReasoningPartView(part: DiscussionReasoningPart) -> impl IntoView {
    let open_items = RwSignal::new(match part.status {
        DiscussionReasoningStatus::Streaming => HashSet::from(["reasoning".to_string()]),
        DiscussionReasoningStatus::Done => HashSet::new(),
    });
    let text = part.text.clone();

    view! {
        <div
            class="orbital-discussion__reasoning-part"
            data-testid="discussion-reasoning-part"
        >
            <Accordion open_items=open_items collapsible=true>
                <AccordionItem value="reasoning">
                    <AccordionHeader slot>"Thinking…"</AccordionHeader>
                    <p class="orbital-discussion__reasoning-text">{text}</p>
                </AccordionItem>
            </Accordion>
        </div>
    }
}
