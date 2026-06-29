//! Localized picker chrome strings resolved from a BCP-47 locale tag.

/// User-facing strings for date/time picker chrome (calendar headers, weekdays, meridiem).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatetimeLocaleStrings {
    /// Short weekday labels starting Sunday (`Sun`..=`Sat` in en-US).
    pub weekday_short: [String; 7],
    /// Short month labels (`Jan`..=`Dec` in en-US).
    pub month_short: [String; 12],
    /// First day of the week (`0` = Sunday, `1` = Monday).
    pub first_day_of_week: u8,
    pub today_label: String,
    pub previous_month_label: String,
    pub next_month_label: String,
    pub am_label: String,
    pub pm_label: String,
}

impl DatetimeLocaleStrings {
    /// Default English (US) picker strings.
    pub fn english() -> Self {
        Self {
            weekday_short: [
                "Sun".into(),
                "Mon".into(),
                "Tue".into(),
                "Wed".into(),
                "Thu".into(),
                "Fri".into(),
                "Sat".into(),
            ],
            month_short: [
                "Jan".into(),
                "Feb".into(),
                "Mar".into(),
                "Apr".into(),
                "May".into(),
                "Jun".into(),
                "Jul".into(),
                "Aug".into(),
                "Sep".into(),
                "Oct".into(),
                "Nov".into(),
                "Dec".into(),
            ],
            first_day_of_week: 0,
            today_label: "Today".into(),
            previous_month_label: "Previous".into(),
            next_month_label: "Next".into(),
            am_label: "AM".into(),
            pm_label: "PM".into(),
        }
    }

    /// French locale preset for localization previews.
    pub fn french() -> Self {
        Self {
            weekday_short: [
                "dim.".into(),
                "lun.".into(),
                "mar.".into(),
                "mer.".into(),
                "jeu.".into(),
                "ven.".into(),
                "sam.".into(),
            ],
            month_short: [
                "janv.".into(),
                "févr.".into(),
                "mars".into(),
                "avr.".into(),
                "mai".into(),
                "juin".into(),
                "juil.".into(),
                "août".into(),
                "sept.".into(),
                "oct.".into(),
                "nov.".into(),
                "déc.".into(),
            ],
            first_day_of_week: 1,
            today_label: "Aujourd'hui".into(),
            previous_month_label: "Précédent".into(),
            next_month_label: "Suivant".into(),
            am_label: "AM".into(),
            pm_label: "PM".into(),
        }
    }

    /// Resolve strings for a BCP-47 locale tag (falls back to English).
    pub fn for_tag(tag: &str) -> Self {
        let normalized = tag.to_ascii_lowercase();
        if normalized.starts_with("fr") {
            Self::french()
        } else {
            Self::english()
        }
    }

    /// Weekday header labels rotated so index `0` is [`first_day_of_week`](Self::first_day_of_week).
    pub fn weekday_header_labels(&self) -> [String; 7] {
        let start = self.first_day_of_week as usize % 7;
        std::array::from_fn(|i| self.weekday_short[(start + i) % 7].clone())
    }
}
