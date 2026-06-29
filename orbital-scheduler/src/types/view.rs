/// Calendar view modes for [`SchedulerCalendar`](crate::SchedulerCalendar).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulerView {
    Day,
    Week,
    Month,
    Agenda,
}

impl SchedulerView {
    /// User-facing label for toolbar and select options.
    pub fn label(self) -> &'static str {
        match self {
            Self::Day => "Day",
            Self::Week => "Week",
            Self::Month => "Month",
            Self::Agenda => "Agenda",
        }
    }

    /// Stable select / wire value.
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Agenda => "agenda",
        }
    }

    /// Parse a select wire value.
    pub fn from_wire_value(value: &str) -> Option<Self> {
        match value {
            "day" => Some(Self::Day),
            "week" => Some(Self::Week),
            "month" => Some(Self::Month),
            "agenda" => Some(Self::Agenda),
            _ => None,
        }
    }

    /// All views in display order.
    pub fn all() -> &'static [Self] {
        &[Self::Day, Self::Week, Self::Month, Self::Agenda]
    }
}

/// Timeline zoom presets for [`SchedulerTimeline`](crate::SchedulerTimeline).
///
/// Presets are capped at one week of wall time — wider spans are hard to read on a timeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelinePreset {
    /// One day at hourly resolution.
    Day,
    /// One business day (08:00–18:00) at hourly resolution.
    BusinessDay,
    /// One week at daily resolution.
    Week,
    /// One business week (Mon–Fri) at daily resolution.
    BusinessWeek,
}

impl TimelinePreset {
    /// User-facing label for toolbar and select options.
    pub fn label(self) -> &'static str {
        match self {
            Self::Day => "Day",
            Self::BusinessDay => "Business day",
            Self::Week => "Week",
            Self::BusinessWeek => "Business week",
        }
    }

    /// Stable select / wire value.
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::BusinessDay => "business_day",
            Self::Week => "week",
            Self::BusinessWeek => "business_week",
        }
    }

    /// Parse a select wire value.
    pub fn from_wire_value(value: &str) -> Option<Self> {
        match value {
            "day" => Some(Self::Day),
            "business_day" => Some(Self::BusinessDay),
            "week" => Some(Self::Week),
            "business_week" => Some(Self::BusinessWeek),
            // Legacy wire values from removed presets.
            "hour" => Some(Self::Day),
            "month" | "year" => Some(Self::Week),
            _ => None,
        }
    }

    /// All presets in display order.
    pub fn all() -> &'static [Self] {
        &[Self::Day, Self::BusinessDay, Self::Week, Self::BusinessWeek]
    }
}
