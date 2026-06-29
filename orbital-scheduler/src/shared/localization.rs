//! Shared scheduler locale strings (SC-15, SC-28).

use leptos::prelude::*;

use crate::{SchedulerView, TimelinePreset};

/// User-facing scheduler chrome strings (toolbar, overlays, view names).
#[derive(Clone, Debug, PartialEq)]
pub struct SchedulerLocaleText {
    pub today: String,
    pub previous: String,
    pub next: String,
    pub day: String,
    pub week: String,
    pub month: String,
    pub agenda: String,
    pub resources: String,
    pub loading_events: String,
    pub preset_day: String,
    pub preset_business_day: String,
    pub preset_week: String,
    pub preset_business_week: String,
    pub preferences: String,
}

impl SchedulerLocaleText {
    pub fn english() -> Self {
        Self {
            today: "Today".into(),
            previous: "Previous".into(),
            next: "Next".into(),
            day: "Day".into(),
            week: "Week".into(),
            month: "Month".into(),
            agenda: "Agenda".into(),
            resources: "Resources".into(),
            loading_events: "Loading events…".into(),
            preset_day: "Day".into(),
            preset_business_day: "Business day".into(),
            preset_week: "Week".into(),
            preset_business_week: "Business week".into(),
            preferences: "Preferences".into(),
        }
    }

    pub fn french() -> Self {
        Self {
            today: "Aujourd'hui".into(),
            previous: "Précédent".into(),
            next: "Suivant".into(),
            day: "Jour".into(),
            week: "Semaine".into(),
            month: "Mois".into(),
            agenda: "Agenda".into(),
            resources: "Ressources".into(),
            loading_events: "Chargement des événements…".into(),
            preset_day: "Jour".into(),
            preset_business_day: "Journée ouvrée".into(),
            preset_week: "Semaine".into(),
            preset_business_week: "Semaine ouvrée".into(),
            preferences: "Préférences".into(),
        }
    }

    pub fn view_label(&self, view: SchedulerView) -> &str {
        match view {
            SchedulerView::Day => &self.day,
            SchedulerView::Week => &self.week,
            SchedulerView::Month => &self.month,
            SchedulerView::Agenda => &self.agenda,
        }
    }

    pub fn preset_label(&self, preset: TimelinePreset) -> &str {
        match preset {
            TimelinePreset::Day => &self.preset_day,
            TimelinePreset::BusinessDay => &self.preset_business_day,
            TimelinePreset::Week => &self.preset_week,
            TimelinePreset::BusinessWeek => &self.preset_business_week,
        }
    }
}

/// Resolve locale text from optional prop or English defaults.
pub fn resolve_scheduler_locale_text(
    locale_text: Option<SchedulerLocaleText>,
) -> SchedulerLocaleText {
    locale_text.unwrap_or_else(SchedulerLocaleText::english)
}

/// Signal-backed locale for reactive toolbar updates in previews.
pub fn locale_text_signal(
    initial: SchedulerLocaleText,
) -> (
    RwSignal<SchedulerLocaleText>,
    ReadSignal<SchedulerLocaleText>,
) {
    let signal = RwSignal::new(initial);
    (signal, signal.read_only())
}
