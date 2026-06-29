//! Floating drag ghost rendered at the scheduler product root.

use leptos::prelude::*;

use crate::shared::interaction::{
    scheduler_drag_active, scheduler_drag_ghost, SchedulerInteractionContext,
};

/// Follower label shown while an event is being dragged or resized.
#[component]
pub fn SchedulerEventDragGhost() -> impl IntoView {
    let ctx = use_context::<SchedulerInteractionContext>();

    view! {
        {move || {
            let Some(ctx) = ctx.clone() else {
                return ().into_any();
            };
            let _ = ctx.drag_repaint.get();
            if !scheduler_drag_active(&ctx) {
                return ().into_any();
            }
            let Some(ghost) = scheduler_drag_ghost(&ctx) else {
                return ().into_any();
            };
            let style = format!(
                "left: {:.0}px; top: {:.0}px; width: {:.0}px; height: {:.0}px;",
                ghost.x, ghost.y, ghost.width_px, ghost.height_px
            );
            view! {
                <div
                    class="orb-scheduler-event-drag-ghost"
                    style=style
                    data-testid="scheduler-event-drag-ghost"
                >
                    {ghost.title}
                </div>
            }
            .into_any()
        }}
    }
}
