//! Scheduler chrome context — preferences and locale for nested views.

use leptos::prelude::*;

use super::localization::SchedulerLocaleText;
use super::preferences::{
    default_scheduler_preferences, scheduler_preferences_from_signals, SchedulerPreferences,
};

/// Preferences and locale strings provided by scheduler products.
#[derive(Clone, Copy)]
pub struct SchedulerChromeContext {
    pub preferences: SchedulerPreferences,
    pub locale_text: RwSignal<SchedulerLocaleText>,
}

impl SchedulerChromeContext {
    pub fn provide(self) {
        provide_context(self);
    }

    pub fn week_layout(&self) -> super::preferences::WeekLayoutPrefs {
        self.preferences.week_layout()
    }

    pub fn try_week_layout(&self) -> Option<super::preferences::WeekLayoutPrefs> {
        self.preferences.try_week_layout()
    }

    pub fn ampm_clock(&self) -> bool {
        self.preferences.ampm_clock()
    }

    pub fn try_ampm_clock(&self) -> Option<bool> {
        self.preferences.try_ampm_clock()
    }
}

/// Read chrome context when inside a scheduler product.
pub fn use_scheduler_chrome() -> Option<SchedulerChromeContext> {
    use_context::<SchedulerChromeContext>()
}

/// Resolve preferences and locale for a scheduler product, inheriting parent preview context.
pub fn resolve_scheduler_chrome(
    preferences: Option<SchedulerPreferences>,
    locale_text: Option<SchedulerLocaleText>,
    locale_store: StoredValue<Option<RwSignal<SchedulerLocaleText>>>,
) -> SchedulerChromeContext {
    let parent = use_context::<SchedulerChromeContext>();

    let resolved_prefs = if let Some(prefs) = preferences {
        prefs
    } else if let Some(parent) = parent {
        if let Some(snap) = parent.preferences.try_snapshot() {
            scheduler_preferences_from_signals(
                RwSignal::new(snap.show_weekends),
                RwSignal::new(snap.ampm),
                RwSignal::new(snap.week_starts_on),
            )
        } else {
            default_scheduler_preferences()
        }
    } else {
        default_scheduler_preferences()
    };

    let locale_rw = if let Some(text) = locale_text {
        let signal = locale_store.get_value().unwrap_or_else(|| {
            let signal = RwSignal::new(text.clone());
            locale_store.set_value(Some(signal));
            signal
        });
        signal.set(text);
        signal
    } else if let Some(parent) = parent {
        // Parent scope owns the locale signal (e.g. localization preview); reuse for live updates.
        parent.locale_text
    } else {
        locale_store.get_value().unwrap_or_else(|| {
            let signal = RwSignal::new(SchedulerLocaleText::english());
            locale_store.set_value(Some(signal));
            signal
        })
    };

    SchedulerChromeContext {
        preferences: resolved_prefs,
        locale_text: locale_rw,
    }
}

/// Week start for the active preferences (Sunday or Monday).
pub fn week_start_offset(preferences: SchedulerPreferences) -> u8 {
    preferences.week_starts_on.get_untracked()
}
