use chrono::{Datelike, Days, NaiveDate};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridDayKind {
    Current,
    Previous,
    Next,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridDay {
    pub date: NaiveDate,
    pub kind: GridDayKind,
    pub unix_secs: i64,
}

pub fn build_month_grid(show_year: i32, show_month: u32) -> Vec<GridDay> {
    let Some(first_of_month) = NaiveDate::from_ymd_opt(show_year, show_month, 1) else {
        return Vec::new();
    };

    let Some(next_month_first) = first_of_month.checked_add_months(chrono::Months::new(1)) else {
        return Vec::new();
    };

    let previous_count = first_of_month.weekday().num_days_from_sunday() as usize;
    let first_visible = first_of_month
        .checked_sub_days(Days::new(previous_count as u64))
        .unwrap_or(first_of_month);

    let days_in_month = (next_month_first - first_of_month).num_days() as usize;
    let mut days = Vec::with_capacity(42);

    for offset in 0..previous_count {
        let date = first_visible
            .checked_add_days(Days::new(offset as u64))
            .expect("valid previous visible day");
        days.push(to_grid_day(date, GridDayKind::Previous));
    }

    for offset in 0..days_in_month {
        let date = first_of_month
            .checked_add_days(Days::new(offset as u64))
            .expect("valid day in current month");
        days.push(to_grid_day(date, GridDayKind::Current));
    }

    let trailing_count = (7 - (days.len() % 7)) % 7;
    for offset in 0..trailing_count {
        let date = next_month_first
            .checked_add_days(Days::new(offset as u64))
            .expect("valid next month day");
        days.push(to_grid_day(date, GridDayKind::Next));
    }

    days
}

fn to_grid_day(date: NaiveDate, kind: GridDayKind) -> GridDay {
    GridDay {
        unix_secs: date
            .and_hms_opt(0, 0, 0)
            .expect("valid midnight")
            .and_utc()
            .timestamp(),
        date,
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::{build_month_grid, GridDayKind};
    use chrono::{NaiveDate, TimeZone, Utc};

    #[test]
    fn builds_full_week_grid_for_month() {
        let grid = build_month_grid(2024, 5);
        assert_eq!(grid.len(), 35);
        assert_eq!(grid[0].kind, GridDayKind::Previous);
        assert_eq!(
            grid[0].date,
            NaiveDate::from_ymd_opt(2024, 4, 28).expect("date")
        );
        assert_eq!(
            grid.last().expect("last").date,
            NaiveDate::from_ymd_opt(2024, 6, 1).expect("date")
        );
        assert_eq!(
            grid.iter()
                .filter(|day| day.kind == GridDayKind::Current)
                .count(),
            31
        );
    }

    #[test]
    fn leap_february_has_29_current_days() {
        let grid = build_month_grid(2024, 2);
        assert_eq!(
            grid.iter()
                .filter(|day| day.kind == GridDayKind::Current)
                .count(),
            29
        );
    }

    #[test]
    fn grid_unix_seconds_are_midnight_utc() {
        let grid = build_month_grid(2025, 1);
        let day = grid
            .iter()
            .find(|day| day.date == NaiveDate::from_ymd_opt(2025, 1, 1).expect("date"))
            .expect("grid has jan 1");
        let expected = Utc
            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
            .single()
            .expect("valid timestamp")
            .timestamp();
        assert_eq!(day.unix_secs, expected);
    }
}
