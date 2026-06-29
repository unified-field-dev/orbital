//! Static sample events and resources for scheduler previews.

use orbital_base_components::{
    DatetimeTimezone, OrbitalDateTime, ToUnixSeconds, TryFromUnixSeconds,
};

use crate::{PlannedEvent, ScheduleResource};

const ANCHOR_UNIX: i64 = 1_735_689_600; // 2025-01-01 UTC anchor

fn tz() -> DatetimeTimezone {
    DatetimeTimezone::Local
}

fn day_anchor() -> OrbitalDateTime {
    OrbitalDateTime::try_from_unix_seconds(ANCHOR_UNIX, tz())
        .expect("valid anchor")
        .start_of_day()
}

fn event_at(
    id: &str,
    title: &str,
    day_offset: i64,
    start_hms: (u32, u32, u32),
    end_hms: (u32, u32, u32),
    resource_id: Option<&str>,
) -> PlannedEvent {
    let day = day_anchor();
    let day_secs = 86_400 * day_offset;
    let base = OrbitalDateTime::try_from_unix_seconds(day.to_unix_seconds() + day_secs, tz())
        .expect("valid day");
    let start = base
        .apply_hms(start_hms.0, start_hms.1, start_hms.2)
        .expect("valid start");
    let end = base
        .apply_hms(end_hms.0, end_hms.1, end_hms.2)
        .expect("valid end");
    let mut event = PlannedEvent::new(id, title, start, end);
    event.resource_id = resource_id.map(str::to_string);
    event
}

/// Sample planned events for quickstart and catalog previews.
pub fn sample_planned_events() -> Vec<PlannedEvent> {
    let mut events = vec![
        event_at(
            "evt-1",
            "Design review",
            0,
            (9, 0, 0),
            (10, 0, 0),
            Some("room-a"),
        ),
        event_at(
            "evt-2",
            "Sprint planning",
            1,
            (13, 0, 0),
            (14, 30, 0),
            Some("room-b"),
        ),
        event_at("evt-3", "Release window", 2, (16, 0, 0), (17, 0, 0), None),
        event_at(
            "evt-4",
            "Team sync",
            3,
            (10, 30, 0),
            (11, 0, 0),
            Some("room-a"),
        ),
    ];
    events[0].color = Some("#2563eb".into());
    events[1].color = Some("#059669".into());
    events[2].color = Some("#d97706".into());
    events[3].color = Some("#7c3aed".into());
    events
}

/// Sample events for timeline previews, including a multi-day bar.
pub fn sample_timeline_events() -> Vec<PlannedEvent> {
    let mut events = vec![
        event_at(
            "evt-1",
            "Design review",
            0,
            (9, 0, 0),
            (10, 0, 0),
            Some("room-a"),
        ),
        event_at(
            "evt-2",
            "Sprint planning",
            1,
            (13, 0, 0),
            (14, 30, 0),
            Some("room-b"),
        ),
        event_at(
            "evt-4",
            "Team sync",
            3,
            (10, 30, 0),
            (11, 0, 0),
            Some("room-a"),
        ),
    ];
    let mut span = event_at(
        "evt-span",
        "Release window",
        0,
        (0, 0, 0),
        (23, 59, 59),
        Some("room-b"),
    );
    let span_end = OrbitalDateTime::try_from_unix_seconds(
        day_anchor().to_unix_seconds() + 86_400 * 2 + 86399,
        tz(),
    )
    .expect("valid end");
    span.end = span_end;
    span.color = Some("#d97706".into());
    events.push(span);
    events[0].color = Some("#2563eb".into());
    events[1].color = Some("#059669".into());
    events[2].color = Some("#7c3aed".into());
    events
}

/// Generate many leaf resources for virtualization previews.
pub fn large_schedule_resources(count: usize) -> Vec<ScheduleResource> {
    (0..count)
        .map(|i| ScheduleResource::new(format!("resource-{i}"), format!("Resource {i}")))
        .collect()
}

/// Nested schedule resources for resource column previews.
pub fn sample_nested_schedule_resources() -> Vec<ScheduleResource> {
    vec![ScheduleResource {
        id: "building".into(),
        title: "Main Building".into(),
        children: vec![
            ScheduleResource::new("room-a", "Conference Room A"),
            ScheduleResource::new("room-b", "Conference Room B"),
        ],
    }]
}

/// Sample schedule resources for quickstart previews.
pub fn sample_schedule_resources() -> Vec<ScheduleResource> {
    vec![
        ScheduleResource::new("room-a", "Conference Room A"),
        ScheduleResource::new("room-b", "Conference Room B"),
    ]
}

/// Fixed-offset Eastern timezone for timezone preview demos.
pub fn eastern_timezone() -> DatetimeTimezone {
    DatetimeTimezone::FixedOffset(-5 * 3600)
}

/// Single event at 9:00 AM Eastern on 2025-01-01 (SC-03).
pub fn sample_timezone_demo_event() -> PlannedEvent {
    let tz = eastern_timezone();
    let day = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let start = OrbitalDateTime::from_naive_date(day, tz)
        .expect("valid")
        .apply_hms(9, 0, 0)
        .expect("valid start");
    let end = start.apply_hms(10, 0, 0).expect("valid end");
    let mut event = PlannedEvent::new("tz-demo", "Cross-timezone review", start, end);
    event.color = Some("#2563eb".into());
    event
}

/// Weekly standup master for recurrence preview (SC-04).
pub fn sample_recurring_standup() -> PlannedEvent {
    let tz = DatetimeTimezone::Utc;
    let monday = chrono::NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
    let start = OrbitalDateTime::from_naive_date(monday, tz)
        .expect("valid")
        .apply_hms(9, 0, 0)
        .expect("valid start");
    let end = start.apply_hms(9, 30, 0).expect("valid end");
    let mut event = PlannedEvent::new("weekly-standup", "Weekly standup", start, end);
    event.recurrence_rule = Some("FREQ=WEEKLY;BYDAY=MO,WE,FR".into());
    event.color = Some("#059669".into());
    event
}
