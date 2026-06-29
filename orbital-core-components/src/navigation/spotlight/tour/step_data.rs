use std::sync::Arc;

use leptos::prelude::*;

use crate::PopoverPosition;

use super::super::anatomy::spotlight_anatomy;
use super::super::slots_to_views::anatomy_from_step;
use super::super::types::SpotlightTourStep;
use super::footer::default_tour_footer;

#[derive(Clone)]
pub struct StepMeta {
    pub anchor_id: String,
    pub position: PopoverPosition,
    pub has_footer: bool,
}

pub struct TourStepContent {
    pub meta: Vec<StepMeta>,
    panels: Arc<Vec<AnyView>>,
}

impl TourStepContent {
    pub fn from_steps(steps: Vec<SpotlightTourStep>, step_count: usize) -> Self {
        let mut meta = Vec::with_capacity(step_count);
        let mut panels = Vec::with_capacity(step_count);

        for step in steps {
            let extracted = anatomy_from_step(step);
            meta.push(StepMeta {
                anchor_id: extracted.anchor_id,
                position: extracted.position,
                has_footer: extracted.has_footer,
            });
            panels.push(spotlight_anatomy(extracted.views).into_any());
        }

        Self {
            meta,
            panels: Arc::new(panels),
        }
    }

    pub fn panel_for(
        &self,
        active_index: ReadSignal<usize>,
        step_count: usize,
        index: usize,
    ) -> AnyView {
        let panel = self
            .panels
            .get(index)
            .cloned()
            .unwrap_or_else(|| ().into_any());
        if self
            .meta
            .get(index)
            .is_some_and(|step| !step.has_footer)
        {
            return view! {
                <div class="orbital-spotlight">
                    {panel}
                    {default_tour_footer(active_index, step_count)}
                </div>
            }
            .into_any();
        }
        view! { <div class="orbital-spotlight">{panel}</div> }.into_any()
    }
}
