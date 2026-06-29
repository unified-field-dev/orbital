//! User-facing locale strings for discussion chrome.

use chrono::{DateTime, Utc};
use leptos::prelude::*;

use super::DiscussionViewMode;

/// User-facing strings for discussion thread chrome (composer, navigation, view modes).
#[derive(Clone, Debug, PartialEq)]
pub struct DiscussionLocale {
    /// Composer textarea placeholder.
    pub composer_placeholder: String,
    /// Accessible name for the composer textarea (screen readers).
    pub composer_aria_label: String,
    /// Focus navigation go-back bar label.
    pub go_back_label: String,
    /// View mode picker field label.
    pub view_picker_label: String,
    /// Tree view mode option label.
    pub view_mode_tree_label: String,
    /// Flat view mode option label.
    pub view_mode_flat_label: String,
    /// Compact view mode option label.
    pub view_mode_compact_label: String,
    /// Relative-time unit suffixes and templates.
    pub relative_time: DiscussionRelativeTimeLocale,
    /// Show-more drill-in button template (`{count}` replaced with the reply count).
    pub show_more_replies_template: String,
}

/// Relative time formatting strings for reply header timestamps.
#[derive(Clone, Debug, PartialEq)]
pub struct DiscussionRelativeTimeLocale {
    pub just_now: String,
    pub minutes_ago: String,
    pub hours_ago: String,
    pub days_ago: String,
    pub weeks_ago: String,
    pub months_ago: String,
    pub years_ago: String,
}

impl DiscussionLocale {
    /// Default English (US) locale strings.
    pub fn english() -> Self {
        Self {
            composer_placeholder: "Write a reply…".into(),
            composer_aria_label: "Reply message".into(),
            go_back_label: "Go back to parent thread".into(),
            view_picker_label: "View".into(),
            view_mode_tree_label: "Tree".into(),
            view_mode_flat_label: "Flat".into(),
            view_mode_compact_label: "Compact".into(),
            show_more_replies_template: "Show {count} more replies".into(),
            relative_time: DiscussionRelativeTimeLocale {
                just_now: "just now".into(),
                minutes_ago: "{n}m ago".into(),
                hours_ago: "{n}h ago".into(),
                days_ago: "{n}d ago".into(),
                weeks_ago: "{n}w ago".into(),
                months_ago: "{n}mo ago".into(),
                years_ago: "{n}y ago".into(),
            },
        }
    }

    /// French locale preset for localization previews and tests.
    pub fn french() -> Self {
        Self {
            composer_placeholder: "Écrire une réponse…".into(),
            composer_aria_label: "Message de réponse".into(),
            go_back_label: "Retour au fil parent".into(),
            view_picker_label: "Affichage".into(),
            view_mode_tree_label: "Arbre".into(),
            view_mode_flat_label: "Plat".into(),
            view_mode_compact_label: "Compact".into(),
            show_more_replies_template: "Afficher {count} réponses de plus".into(),
            relative_time: DiscussionRelativeTimeLocale {
                just_now: "à l'instant".into(),
                minutes_ago: "il y a {n} min".into(),
                hours_ago: "il y a {n} h".into(),
                days_ago: "il y a {n} j".into(),
                weeks_ago: "il y a {n} sem".into(),
                months_ago: "il y a {n} mois".into(),
                years_ago: "il y a {n} an".into(),
            },
        }
    }

    /// Label for a view mode picker option.
    pub fn view_mode_label(&self, mode: DiscussionViewMode) -> &str {
        match mode {
            DiscussionViewMode::Tree => &self.view_mode_tree_label,
            DiscussionViewMode::Flat => &self.view_mode_flat_label,
            DiscussionViewMode::Compact => &self.view_mode_compact_label,
        }
    }

    /// Show-more drill-in button label for `count` hidden or nested replies.
    pub fn show_more_replies(&self, count: u32) -> String {
        self.show_more_replies_template
            .replace("{count}", &count.to_string())
    }

    /// Format a UTC timestamp as a relative time string.
    pub fn format_relative_time(&self, at: DateTime<Utc>, now: DateTime<Utc>) -> String {
        let rt = &self.relative_time;
        let duration = now.signed_duration_since(at);
        let seconds = duration.num_seconds();

        if seconds < 60 {
            return rt.just_now.clone();
        }

        let minutes = duration.num_minutes();
        if minutes < 60 {
            return rt.minutes_ago.replace("{n}", &minutes.to_string());
        }

        let hours = duration.num_hours();
        if hours < 24 {
            return rt.hours_ago.replace("{n}", &hours.to_string());
        }

        let days = duration.num_days();
        if days < 7 {
            return rt.days_ago.replace("{n}", &days.to_string());
        }

        let weeks = days / 7;
        if weeks < 5 {
            return rt.weeks_ago.replace("{n}", &weeks.to_string());
        }

        let months = days / 30;
        if months < 12 {
            return rt.months_ago.replace("{n}", &months.to_string());
        }

        let years = days / 365;
        rt.years_ago.replace("{n}", &years.to_string())
    }
}

/// Resolve locale from an optional partial override, falling back to English defaults.
pub fn resolve_discussion_locale(locale: Option<DiscussionLocale>) -> DiscussionLocale {
    locale.unwrap_or_else(DiscussionLocale::english)
}

/// Signal-backed locale for reactive toolbar and preview language toggles.
pub fn locale_signal(
    initial: DiscussionLocale,
) -> (RwSignal<DiscussionLocale>, ReadSignal<DiscussionLocale>) {
    let signal = RwSignal::new(initial);
    (signal, signal.read_only())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ts(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
    }

    #[test]
    fn english_relative_time_just_now() {
        let locale = DiscussionLocale::english();
        let now = ts(2026, 6, 19, 12, 0);
        assert_eq!(locale.format_relative_time(now, now), "just now");
    }

    #[test]
    fn english_relative_time_hours_ago() {
        let locale = DiscussionLocale::english();
        let now = ts(2026, 6, 19, 14, 0);
        let at = ts(2026, 6, 19, 12, 0);
        assert_eq!(locale.format_relative_time(at, now), "2h ago");
    }

    #[test]
    fn english_show_more_replies() {
        let locale = DiscussionLocale::english();
        assert_eq!(locale.show_more_replies(4), "Show 4 more replies");
    }

    #[test]
    fn french_composer_placeholder_distinct() {
        let en = DiscussionLocale::english();
        let fr = DiscussionLocale::french();
        assert_ne!(en.composer_placeholder, fr.composer_placeholder);
        assert_eq!(fr.composer_placeholder, "Écrire une réponse…");
    }

    #[test]
    fn french_show_more_replies() {
        let locale = DiscussionLocale::french();
        assert_eq!(locale.show_more_replies(3), "Afficher 3 réponses de plus");
    }
}
