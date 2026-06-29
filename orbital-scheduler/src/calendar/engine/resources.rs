//! Resource row flattening and per-cell event filtering.

use chrono::NaiveDate;

use orbital_base_components::DatetimeTimezone;

use super::event_layout::event_overlaps_day;
use crate::{PlannedEvent, ScheduleResource};

/// A schedulable resource row in the week/day grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceRow {
    pub id: String,
    pub title: String,
    pub depth: usize,
}

/// Sentinel id for events without a resource assignment.
pub const UNASSIGNED_RESOURCE_ID: &str = "__unassigned__";

fn flatten_inner(resources: &[ScheduleResource], depth: usize, out: &mut Vec<ResourceRow>) {
    for resource in resources {
        if resource.children.is_empty() {
            out.push(ResourceRow {
                id: resource.id.clone(),
                title: resource.title.clone(),
                depth,
            });
        } else {
            flatten_inner(&resource.children, depth + 1, out);
        }
    }
}

/// DFS flatten: leaf nodes become schedulable rows; parent-only nodes are skipped.
pub fn flatten_resource_rows(resources: &[ScheduleResource]) -> Vec<ResourceRow> {
    let mut rows = Vec::new();
    flatten_inner(resources, 0, &mut rows);
    rows
}

/// Build resource rows for the grid, including an unassigned row when needed.
pub fn resource_rows_for_grid(resources: &[ScheduleResource]) -> Vec<ResourceRow> {
    if resources.is_empty() {
        return Vec::new();
    }
    let mut rows = flatten_resource_rows(resources);
    rows.push(ResourceRow {
        id: UNASSIGNED_RESOURCE_ID.to_string(),
        title: "Unassigned".to_string(),
        depth: 0,
    });
    rows
}

/// Events matching a grid cell (day + optional resource row).
pub fn events_for_cell<'a>(
    events: &'a [PlannedEvent],
    day: NaiveDate,
    resource_id: Option<&str>,
    display_tz: DatetimeTimezone,
    has_resources: bool,
) -> Vec<&'a PlannedEvent> {
    events
        .iter()
        .filter(|event| {
            if !event_overlaps_day(event, day, display_tz) {
                return false;
            }
            if !has_resources {
                return true;
            }
            match resource_id {
                Some(UNASSIGNED_RESOURCE_ID) => event.resource_id.is_none(),
                Some(id) => event.resource_id.as_deref() == Some(id),
                None => true,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{
        DatetimeTimezone, OrbitalDateTime, ToUnixSeconds, TryFromUnixSeconds,
    };

    fn anchor() -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Utc)
            .expect("valid")
            .start_of_day()
    }

    fn event(id: &str, resource_id: Option<&str>, day_offset: i64) -> PlannedEvent {
        let day = anchor();
        let base = OrbitalDateTime::try_from_unix_seconds(
            day.to_unix_seconds() + day_offset * 86_400,
            DatetimeTimezone::Utc,
        )
        .expect("valid");
        let start = base.apply_hms(9, 0, 0).expect("start");
        let end = base.apply_hms(10, 0, 0).expect("end");
        let mut event = PlannedEvent::new(id, id, start, end);
        event.resource_id = resource_id.map(str::to_string);
        event
    }

    #[test]
    fn flatten_nested_resources() {
        let resources = vec![ScheduleResource {
            id: "building".into(),
            title: "Main Building".into(),
            children: vec![
                ScheduleResource::new("room-a", "Room A"),
                ScheduleResource::new("room-b", "Room B"),
            ],
        }];
        let rows = flatten_resource_rows(&resources);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, "room-a");
        assert_eq!(rows[0].depth, 1);
        assert_eq!(rows[1].depth, 1);
    }

    #[test]
    fn filter_by_resource_and_day() {
        let day = anchor().wall_date().unwrap();
        let events = vec![
            event("a", Some("room-a"), 0),
            event("b", Some("room-b"), 0),
            event("c", None, 0),
        ];
        let room_a = events_for_cell(&events, day, Some("room-a"), DatetimeTimezone::Utc, true);
        assert_eq!(room_a.len(), 1);
        assert_eq!(room_a[0].id, "a");

        let unassigned = events_for_cell(
            &events,
            day,
            Some(UNASSIGNED_RESOURCE_ID),
            DatetimeTimezone::Utc,
            true,
        );
        assert_eq!(unassigned.len(), 1);
        assert_eq!(unassigned[0].id, "c");
    }

    #[test]
    fn no_resources_shows_all_events() {
        let day = anchor().wall_date().unwrap();
        let events = vec![event("a", Some("room-a"), 0), event("b", None, 0)];
        let all = events_for_cell(&events, day, None, DatetimeTimezone::Utc, false);
        assert_eq!(all.len(), 2);
    }
}
