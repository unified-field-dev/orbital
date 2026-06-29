//! Scheduler slot composition hub doc.

use std::sync::Arc;

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::{
    Badge, BadgeAppearance, Button, ButtonAppearance, Caption1, Flex, FlexAlign, FlexGap,
    ThemeDensityStepper, Toolbar,
};
use orbital_macros::component_doc;

use crate::{
    open_create_default, preview_anchor_date, use_scheduler_interaction, EventContentView,
    ScheduleResource, SchedulerCalendar, SchedulerEditingTools, SchedulerEventContent,
    SchedulerFeatures, SchedulerToolbar, SchedulerView,
};

/// Leptos slot regions for scheduler calendar and timeline composition.
///
/// # When to use
///
/// - Replacing the default toolbar, editing affordances, or lazy-load overlays.
/// - Customizing event chip inner content, resource labels, or agenda rows.
///
/// # Usage
///
/// Nest slot children on [`SchedulerCalendar`] or [`SchedulerTimeline`]:
/// [`SchedulerToolbar`], [`SchedulerEditingTools`], [`SchedulerEventContent`],
/// [`SchedulerResourceLabel`], [`SchedulerLoadingView`], and [`SchedulerErrorView`].
/// Wire side effects through [`SchedulerEvents`] and read interaction state with
/// [`use_scheduler_interaction`].
///
/// # Examples
///
/// ## Custom toolbar, event badges, and editing tools
/// Week view with slot-driven toolbar, chip badges, and a New event button.
/// <!-- preview -->
/// ```rust,ignore
/// use std::sync::Arc;
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{
///     Badge, BadgeAppearance, Button, ButtonAppearance, Caption1, Flex, FlexAlign, FlexGap,
///     ThemeDensityStepper, Toolbar,
/// };
/// use crate::{
///     EventContentView, EventRenderContext, EventRenderSurface, PlannedEvent, ScheduleResource,
///     SchedulerCalendar, SchedulerEditingTools, SchedulerEventContent, SchedulerFeatures,
///     SchedulerToolbar, SchedulerView, open_create_default, preview_anchor_date,
///     use_scheduler_interaction,
/// };
/// use crate::preview::fixtures::sample_planned_events;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(sample_planned_events());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let event_content: EventContentView = Arc::new(|ctx| {
///     Some(view! {
///         <Badge appearance=BadgeAppearance::Outline attr:data-testid="scheduler-slot-event-badge">
///             {ctx.event.title.clone()}
///         </Badge>
///     }.into_any())
/// });
/// view! {
///     <div data-testid="scheduler-slots-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::empty()
///                 event_creation=Signal::from(true)
///             >
///                 <SchedulerToolbar slot>
///                     <Toolbar attr:aria-label="Custom calendar chrome" attr:data-testid="scheduler-custom-toolbar">
///                         <Button
///                             appearance=ButtonAppearance::Subtle
///                             on:click=move |_| visible_date.set(preview_anchor_date())
///                         >
///                             "Reset week"
///                         </Button>
///                         <Caption1 class="orb-scheduler__toolbar-label">"Custom slot toolbar"</Caption1>
///                     </Toolbar>
///                 </SchedulerToolbar>
///                 <SchedulerEventContent slot render=event_content />
///                 <SchedulerEditingTools slot>
///                     <Button
///                         attr:data-testid="scheduler-event-dialog-open"
///                         on:click=move |_| {
///                             open_create_default(&use_scheduler_interaction(), visible_date.get_untracked());
///                         }
///                     >
///                         "New event"
///                     </Button>
///                 </SchedulerEditingTools>
///             </SchedulerCalendar>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-slots",
    preview_label = "Scheduler Slots",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn SchedulerSlotsDoc() -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(crate::preview::fixtures::sample_planned_events());
        let resources = RwSignal::new(Vec::<ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        let event_content: EventContentView = Arc::new(|ctx| {
            Some(
                view! {
                    <Badge
                        appearance=BadgeAppearance::Outline
                        attr:data-testid="scheduler-slot-event-badge"
                    >
                        {ctx.event.title.clone()}
                    </Badge>
                }
                .into_any(),
            )
        });

        view! {
            <div data-testid="scheduler-slots-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <SchedulerCalendar
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        view=view
                        features=SchedulerFeatures::empty()
                        event_creation=Signal::from(true)
                    >
                        <SchedulerToolbar slot>
                            <Toolbar
                                attr:aria-label="Custom calendar chrome"
                                attr:data-testid="scheduler-custom-toolbar"
                            >
                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    on:click=move |_| visible_date.set(preview_anchor_date())
                                >
                                    "Reset week"
                                </Button>
                                <Caption1 class="orb-scheduler__toolbar-label">
                                    "Custom slot toolbar"
                                </Caption1>
                            </Toolbar>
                        </SchedulerToolbar>
                        <SchedulerEventContent slot render=event_content />
                        <SchedulerEditingTools slot>
                            <Button
                                attr:data-testid="scheduler-event-dialog-open"
                                on:click=move |_| {
                                    open_create_default(
                                        &use_scheduler_interaction(),
                                        visible_date.get_untracked(),
                                    );
                                }
                            >
                                "New event"
                            </Button>
                        </SchedulerEditingTools>
                    </SchedulerCalendar>
                </Flex>
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    view! { () }.into_any()
}
