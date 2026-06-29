use leptos::prelude::*;
use orbital_core_components::{Tag, TagAppearance, TagSize};

use crate::{DiscussionStepPart, DiscussionStepStatus};

fn step_status_label(status: DiscussionStepStatus) -> &'static str {
    match status {
        DiscussionStepStatus::Active => "In progress",
        DiscussionStepStatus::Complete => "Complete",
        DiscussionStepStatus::Error => "Error",
    }
}

fn step_status_appearance(status: DiscussionStepStatus) -> TagAppearance {
    match status {
        DiscussionStepStatus::Active => TagAppearance::Brand,
        DiscussionStepStatus::Complete => TagAppearance::Filled,
        DiscussionStepStatus::Error => TagAppearance::Outline,
    }
}

/// Step delimiter row with label and status chip.
#[component]
pub fn DiscussionStepPartView(part: DiscussionStepPart) -> impl IntoView {
    let step_number = part.step_number;
    let label = part.label.clone();
    let status = part.status;
    let status_attr = status.as_str().to_string();

    view! {
        <div
            class="orbital-discussion__step-part"
            data-testid="discussion-step-part"
            data-step-status=status_attr
        >
            <span class="orbital-discussion__step-label">
                {format!("Step {step_number}: {label}")}
            </span>
            <Tag
                class="orbital-discussion__step-status".to_string()
                appearance=Signal::derive(move || step_status_appearance(status))
                size=Signal::derive(|| TagSize::ExtraSmall)
            >
                {step_status_label(status)}
            </Tag>
        </div>
    }
}
