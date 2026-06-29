use chrono::{Datelike, Months, NaiveDate, Utc};
use leptos::prelude::*;
use orbital_base_components::{
    build_month_grid, format_unix, is_day_disabled, DatetimeFormat, DatetimeTimezone, GridDayKind,
    OrbitalDateTime,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::button::{Button, ButtonAppearance};
use crate::forms::button_group::ButtonGroup;
use crate::forms::calendar::bind::CalendarBind;
use crate::forms::calendar::day::{
    default_calendar_day, CalendarDayProps, CalendarDayRenderer, CalendarWeekdayHeader,
};

use super::styles::calendar_styles;

/// Inline month grid for picking a single day, bound to [`OrbitalDateTime`].
///
/// Calendar is an inline month grid for picking a single day. Bind an `Option<OrbitalDateTime>` (start-of-day in your chosen [`DatetimeTimezone`](orbital_base_components::DatetimeTimezone)), navigate months with the header controls, or jump to today with one click. Use it directly on filters and settings panels. For a text field with an anchored calendar panel, use [`DatePicker`](crate::DatePicker) instead. For the date-pickers plugin surface, import `DateCalendar` — it wraps this same grid.
///
/// # Customization
///
/// Pass an optional `day` renderer to replace the default day cell while keeping grid keyboard navigation.
///
/// # When to use
///
/// - Inline date selection on filters and settings panels
/// - Month browsing with explicit day picking where no text field is needed
/// - [`OrbitalDateTime`](orbital_base_components::OrbitalDateTime) workflows that still need a visual day grid
///
/// # Calendar vs DatePicker
///
/// | Need | Component |
/// |------|-----------|
/// | Inline month grid on the page | `Calendar` |
/// | Text field + anchored calendar panel | `DatePicker` |
///
/// # Usage
///
/// 1. Bind `value` with [`CalendarBind`] using `OptionBind<OrbitalDateTime>`.
/// 2. Set `timezone` on [`CalendarAppearance`] to control start-of-day conversion.
/// 3. Read the bound [`OrbitalDateTime`] in app code; convert at API boundaries via `ToUnixSeconds` / `ToIso8601`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`Calendar`](crate::Calendar) directly in core for inline month grids
/// * Configure `timezone` explicitly when users span regions
///
/// ## Don'ts
///
/// * Do not use for popover field entry — prefer [`DatePicker`](crate::DatePicker)
/// * Do not confuse with `DateCalendar` in `orbital-date-pickers` — that crate wraps this `Calendar` for plugin APIs
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default calendar
/// Local timezone calendar with month navigation and click-to-select behavior.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="calendar-preview">
///         <Calendar bind=value />
///     </div>
/// }
/// ```
///
/// ## Preselected day
/// Preseed unix value highlights its calendar day on first render.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1718409600_i64, DatetimeTimezone::Local)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="CA-02">
///         <Calendar bind=value />
///     </div>
/// }
/// ```
///
/// ## Selection value readout
/// Clicking a day updates the bound unix value (rendered below for verification).
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, OrbitalDateTime, format_datetime};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="CA-03">
///         <Calendar bind=value />
///         <div data-testid="CA-03-VALUE">
///             {move || value.get().map(|v| format_datetime(v, DatetimeFormat::IsoDate)).unwrap_or_else(|| "none".to_string())}
///         </div>
///     </div>
/// }
/// ```
///
/// ## UTC mode
/// Uses UTC for value conversion and date formatting.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="CA-04">
///         <Calendar bind=value appearance=DatetimeTimezone::Utc />
///     </div>
/// }
/// ```
///
/// ## Fixed offset mode
/// Fixed offset timezone (-05:00) for region-specific date semantics.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="CA-05">
///         <Calendar bind=value appearance=DatetimeTimezone::FixedOffset(-5 * 3600) />
///     </div>
/// }
/// ```
///
/// ## Month navigation
/// Previous and next buttons navigate visible months while preserving selected value.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="CA-06">
///         <Calendar bind=value />
///     </div>
/// }
/// ```
///
/// ## Today action
/// Today button jumps to the current month and selects today at start-of-day.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="CA-07">
///         <Calendar bind=value />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "calendar",
    preview_label = "Calendar",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn Calendar(
    /// Value binding for selected [`OrbitalDateTime`].
    #[prop(optional, into)]
    bind: CalendarBind,
    /// Visual and conversion appearance options.
    #[prop(optional, into)]
    appearance: CalendarAppearance,
    /// Optional custom day cell renderer.
    #[prop(default = None)]
    day: Option<CalendarDayRenderer>,
    /// Extra CSS class names merged on the root calendar element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional localized header and weekday labels.
    #[prop(optional)]
    chrome_labels: Option<CalendarChromeLabels>,
) -> impl IntoView {
    inject_style("orbital-calendar", calendar_styles());

    let CalendarBind { value } = bind;
    let CalendarAppearance {
        timezone,
        min_date,
        max_date,
    } = appearance;
    let value = StoredValue::new(value);
    let timezone = StoredValue::new(timezone);
    let min_date = StoredValue::new(min_date);
    let max_date = StoredValue::new(max_date);
    let day_renderer = StoredValue::new(day);
    let chrome_labels = StoredValue::new(chrome_labels.unwrap_or_default());

    let initial_date = value
        .get_value()
        .get_untracked()
        .and_then(|dt| dt.wall_date())
        .unwrap_or_else(|| today_for_timezone(timezone.get_value()));
    let show_date = RwSignal::new(initial_date);
    let focused_date = RwSignal::new(initial_date);

    Effect::new(move |_| {
        if let Some(selected) = value.get_value().get() {
            if let Some(selected_date) = selected.wall_date() {
                show_date.update(|current| {
                    if current.year() != selected_date.year()
                        || current.month() != selected_date.month()
                    {
                        *current = selected_date;
                    }
                });
                focused_date.set(selected_date);
            }
        }
    });

    let month_grid = Memo::new(move |_| {
        let current = show_date.get();
        build_month_grid(current.year(), current.month())
    });

    let previous_month = move |_| {
        show_date.update(|date| {
            if let Some(prev) = date.checked_sub_months(Months::new(1)) {
                *date = prev;
            }
        });
    };

    let next_month = move |_| {
        show_date.update(|date| {
            if let Some(next) = date.checked_add_months(Months::new(1)) {
                *date = next;
            }
        });
    };

    let jump_to_today = move |_| {
        let tz = timezone.get_value();
        let now = today_for_timezone(tz);
        show_date.set(now);
        focused_date.set(now);
        value.with_value(|v| v.set(Some(OrbitalDateTime::utc_now(tz).start_of_day())));
    };

    let select_date = move |date: NaiveDate,
                            tz: DatetimeTimezone,
                            min: Option<OrbitalDateTime>,
                            max: Option<OrbitalDateTime>| {
        if is_day_disabled(date, min, max) {
            return;
        }
        show_date.set(date);
        focused_date.set(date);
        if let Some(dt) = orbital_from_date(date, tz) {
            value.with_value(|v| v.set(Some(dt)));
        }
    };

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-calendar".to_string()];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        }>
            <div class="orbital-calendar__header">
                <span class="orbital-calendar__header-title" data-testid="calendar-header-title">
                    {move || show_date.with(|date| date.format("%B %Y").to_string())}
                </span>
                <ButtonGroup>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(previous_month)
                    >
                        {move || chrome_labels.get_value().previous_month.clone()}
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(jump_to_today)
                    >
                        {move || chrome_labels.get_value().today.clone()}
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(next_month)
                    >
                        {move || chrome_labels.get_value().next_month.clone()}
                    </Button>
                </ButtonGroup>
            </div>

            <CalendarWeekdayHeader labels=chrome_labels.get_value().weekday_short.clone() />

            <div
                class="orbital-calendar__dates"
                role="grid"
                aria-label="Calendar days"
                tabindex="-1"
                on:keydown=move |ev| {
                    let tz = timezone.get_value();
                    let min = min_date.get_value();
                    let max = max_date.get_value();
                    handle_calendar_grid_keydown(
                        ev,
                        focused_date,
                        show_date,
                        move |date| select_date(date, tz, min, max),
                    );
                }
            >
                {move || {
                    let tz = timezone.get_value();
                    let min = min_date.get_value();
                    let max = max_date.get_value();
                    let today = today_for_timezone(tz);
                    let selected = value.get_value().get();
                    let focus = focused_date.get();
                    let renderer = day_renderer.get_value();
                    month_grid
                        .get()
                        .into_iter()
                        .map(|day| {
                            let selected_date = selected.and_then(|dt| dt.wall_date());
                            let is_selected =
                                selected_date.map(|d| d == day.date).unwrap_or(false);
                            let is_today = day.date == today;
                            let is_other_month = day.kind != GridDayKind::Current;
                            let is_disabled = is_day_disabled(day.date, min, max);
                            let day_label = format_unix(
                                day.unix_secs,
                                DatetimeFormat::IsoDate,
                                DatetimeTimezone::Utc,
                            );
                            let tabindex = if is_disabled {
                                "-1"
                            } else if focus == day.date {
                                "0"
                            } else {
                                "-1"
                            };
                            let date = day.date;
                            let on_select = Callback::new(move |_| {
                                select_date(date, tz, min, max);
                            });
                            let on_focus = Callback::new(move |_| focused_date.set(date));
                            let props = CalendarDayProps {
                                date,
                                day_number: day.date.day(),
                                weekday_label: None,
                                selected: is_selected,
                                today: is_today,
                                other_month: is_other_month,
                                disabled: is_disabled,
                                aria_label: day_label,
                                tabindex,
                                on_select,
                                on_focus,
                            };
                            if let Some(render) = renderer.clone() {
                                render(props).into_any()
                            } else {
                                default_calendar_day(props).into_any()
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

/// Localized calendar chrome copy (optional — English defaults when omitted).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarChromeLabels {
    pub weekday_short: [String; 7],
    pub today: String,
    pub previous_month: String,
    pub next_month: String,
}

impl Default for CalendarChromeLabels {
    fn default() -> Self {
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
            today: "Today".into(),
            previous_month: "Previous".into(),
            next_month: "Next".into(),
        }
    }
}

/// Appearance contract for [`Calendar`].
#[derive(Clone, Copy, Debug)]
pub struct CalendarAppearance {
    pub timezone: DatetimeTimezone,
    pub min_date: Option<OrbitalDateTime>,
    pub max_date: Option<OrbitalDateTime>,
}

impl Default for CalendarAppearance {
    fn default() -> Self {
        Self {
            timezone: DatetimeTimezone::Local,
            min_date: None,
            max_date: None,
        }
    }
}

impl CalendarAppearance {
    pub fn new(timezone: DatetimeTimezone) -> Self {
        Self {
            timezone,
            min_date: None,
            max_date: None,
        }
    }
}

impl From<DatetimeTimezone> for CalendarAppearance {
    fn from(timezone: DatetimeTimezone) -> Self {
        Self::new(timezone)
    }
}

fn handle_calendar_grid_keydown(
    ev: leptos::ev::KeyboardEvent,
    focused_date: RwSignal<NaiveDate>,
    show_date: RwSignal<NaiveDate>,
    select_date: impl Fn(NaiveDate) + Copy + 'static,
) {
    let key = ev.key();
    let current = focused_date.get_untracked();
    let next = match key.as_str() {
        "ArrowLeft" => current.pred_opt(),
        "ArrowRight" => current.succ_opt(),
        "ArrowUp" => current.checked_sub_days(chrono::Days::new(7)),
        "ArrowDown" => current.checked_add_days(chrono::Days::new(7)),
        "Home" => Some(start_of_week(current)),
        "End" => Some(end_of_week(current)),
        "PageUp" => current.checked_sub_months(Months::new(1)),
        "PageDown" => current.checked_add_months(Months::new(1)),
        "Enter" | " " => {
            ev.prevent_default();
            select_date(current);
            return;
        }
        _ => None,
    };
    if let Some(next_date) = next {
        ev.prevent_default();
        focused_date.set(next_date);
        if next_date.year() != show_date.get_untracked().year()
            || next_date.month() != show_date.get_untracked().month()
        {
            show_date.set(next_date);
        }
    }
}

fn start_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_sunday();
    date.checked_sub_days(chrono::Days::new(weekday as u64))
        .unwrap_or(date)
}

fn end_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_sunday();
    date.checked_add_days(chrono::Days::new(6 - weekday as u64))
        .unwrap_or(date)
}

fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    use chrono::{FixedOffset, Local};
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}
