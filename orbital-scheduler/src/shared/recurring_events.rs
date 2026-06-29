//! Recurring events preview and RRULE expansion documentation (SC-04).

use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, Text, ThemeDensityStepper};
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{SchedulerCalendar, SchedulerFeatures, SchedulerView};

#[cfg(feature = "preview")]
use crate::preview::fixtures::sample_recurring_standup;

/// Show recurring series on [`SchedulerCalendar`] by expanding RFC 5545-style rules on master events.
///
/// # When to use
///
/// - Standups, shifts, or maintenance windows that repeat daily or weekly
/// - Showing multiple synthetic instances inside the visible calendar window
///
/// # Usage
///
/// 1. Set `recurrence_rule` on a master [`PlannedEvent`] using the supported subset
///    (`FREQ=DAILY`, `FREQ=WEEKLY` with optional `INTERVAL`, `BYDAY`, `COUNT`, `UNTIL`).
/// 2. Enable [`SchedulerFeatures::RECURRING_EVENTS`] on [`SchedulerCalendar`] or [`SchedulerTimeline`].
///
/// Expanded instances receive synthetic ids (`{master_id}::{unix}`) for display.
/// Editing or dragging an instance updates the master event (series edit).
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep one master row per series in your backing store — let expansion happen at render time.
///
/// ## Don'ts
///
/// - Do not assume full RRULE parity — document the supported subset in your product copy.
///
/// # Examples
///
/// ## Weekly standup
/// Mon/Wed/Fri standup expands to three chips in the anchor week.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{SchedulerCalendar, SchedulerFeatures, SchedulerView};
/// use crate::preview::fixtures::sample_recurring_standup;
/// let visible_date = RwSignal::new(
///     orbital_base_components::OrbitalDateTime::from_naive_date(
///         chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
///         DatetimeTimezone::Utc,
///     ).expect("valid anchor"),
/// );
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(vec![sample_recurring_standup()]);
/// let resources = RwSignal::new(Vec::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-recurring-events-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::RECURRING_EVENTS
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-recurring-events",
    preview_label = "Scheduler Recurring Events",
    preview_icon = icondata::AiReloadOutlined,
)]
#[component]
pub fn SchedulerRecurringEvents(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(
            orbital_base_components::OrbitalDateTime::from_naive_date(
                chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                DatetimeTimezone::Utc,
            )
            .expect("valid anchor"),
        );
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(vec![sample_recurring_standup()]);
        let resources = RwSignal::new(Vec::<crate::ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        let root_class = move || class.get().filter(|c| !c.is_empty()).unwrap_or_default();

        view! {
            <div data-testid="scheduler-recurring-events-preview" class=root_class>
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <Text>"Weekly standup with FREQ=WEEKLY;BYDAY=MO,WE,FR"</Text>
                    <SchedulerCalendar
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        view=view
                        features=SchedulerFeatures::RECURRING_EVENTS
                    />
                </Flex>
                {children.map(|c| c())}
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        let _ = (&class, &children);
        ().into_any()
    }
}
