//! [`DateCalendar`] — picker layout wrapper over core [`Calendar`].

use leptos::prelude::*;
use orbital_base_components::{FieldInjection, Rule};
use orbital_core_components::{
    Calendar, CalendarAppearance, CalendarBind, CalendarChromeLabels, PickerShortcutsBar,
};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{
    layout_root_classes, picker_style_sheet, use_datetime_locale_strings,
    PickerLayoutActionBarSlot, PickerLayoutToolbarSlot,
};

use super::field_types::{DateCalendarAppearance, DateCalendarBind};

/// Standalone calendar panel for date selection, bound to [`OrbitalDateTime`].
///
/// DateCalendar wraps the core [`Calendar`](orbital_core_components::Calendar) grid in the
/// plugin picker layout shell (`orb-picker-layout`). Use it when building date-picker plugin
/// surfaces; prefer core `Calendar` directly for inline grids in application code.
///
/// # When to use
///
/// - Date-pickers plugin consumers expecting the DateCalendar name
/// - Popover or dialog panels that need picker chrome around the month grid
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`DateCalendarBind`].
/// 2. Set `appearance.min_date` / `max_date` to constrain selectable days.
/// 3. Provide `appearance.shortcuts` for preset chips (Today, Yesterday, etc.).
/// 4. Wrap preview examples in a native element with `data-testid`.
///
/// # Lifecycle
///
/// - **Value:** commits immediately when a day or shortcut is selected.
/// - **Open state:** inline only — no popover; the grid is always visible.
/// - **Validation:** `bind.rules` run on value change (`Change` trigger) when nested in [`Field`](orbital_core_components::Field).
///
/// # Timezone
///
/// `appearance.timezone` controls how calendar days map to [`OrbitalDateTime`] start-of-day.
/// The bound value carries its own `OrbitalDateTime::timezone()`; keep them aligned for consistent display.
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind with [`OrbitalDateTime`] at start-of-day in your timezone
/// * Wrap previews with `data-testid` on a native element
///
/// ## Don'ts
///
/// * Do not reimplement month grid logic — extend core `Calendar` instead
/// * Do not put `data-testid` on the component itself
///
/// # Examples
///
/// ## Default calendar
/// Inline month grid inside the picker layout shell with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-calendar-preview">
///         <PickerPreviewKnobs />
///         <DateCalendar bind=value />
///         <div data-testid="date-calendar-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Clicking a day updates the bound [`OrbitalDateTime`].
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DC-02">
///         <DateCalendar bind=value />
///         <div data-testid="DC-02-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## UTC timezone
/// Calendar days resolve against UTC start-of-day.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DC-03">
///         <DateCalendar bind=value appearance=DateCalendarAppearance { timezone: Signal::from(DatetimeTimezone::Utc), ..Default::default() } />
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Min/max with Field validation
/// Shortcut selection outside the allowed range surfaces a Field error.
/// <!-- preview -->
/// ```rust
/// use orbital_core_components::Field;
/// use crate::preview::PickerPreviewExample;
/// use orbital_base_components::{DatePickerRule, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let min = Signal::from(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid min"),
/// ));
/// let max = Signal::from(Some(
///     OrbitalDateTime::try_from_unix_seconds(1767225600_i64, DatetimeTimezone::Utc)
///         .expect("valid max"),
/// ));
/// view! {
///     <PickerPreviewExample data_testid="DC-04">
///         <Field label="Event date" name="event_date">
///             <DateCalendar
///                 bind=DateCalendarBind { value: value.into(), rules: vec![DatePickerRule::min_date(min), DatePickerRule::max_date(max)], ..Default::default() }
///                 appearance=DateCalendarAppearance { min_date: min, max_date: max, ..Default::default() }
///             />
///         </Field>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-calendar",
    preview_label = "Date Calendar",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DateCalendar(
    /// Selected day binding forwarded to core [`Calendar`].
    #[prop(optional, into)]
    bind: DateCalendarBind,
    /// Timezone, min/max range, shortcuts, and disabled state.
    #[prop(optional, into)]
    appearance: DateCalendarAppearance,
    /// Optional CSS class merged onto the layout root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional toolbar slot above shortcuts.
    #[prop(optional)]
    picker_layout_toolbar_slot: Option<PickerLayoutToolbarSlot>,
    /// Optional action bar slot below the calendar grid.
    #[prop(optional)]
    picker_layout_action_bar_slot: Option<PickerLayoutActionBarSlot>,
) -> impl IntoView {
    let DateCalendarBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let DateCalendarAppearance {
        timezone,
        min_date,
        max_date,
        shortcuts,
        disabled,
        day,
    } = appearance;

    let toolbar_children = StoredValue::new(picker_layout_toolbar_slot.map(|slot| slot.children));
    let action_bar_children =
        StoredValue::new(picker_layout_action_bar_slot.map(|slot| slot.children));

    let theme_options = use_theme_options();
    let (resolved_id, resolved_name) = FieldInjection::use_id_and_name(id, name);
    let _ = resolved_id;
    let _ = resolved_name;

    let value_stored = StoredValue::new(value);
    let validate = Rule::validate(rules, value_stored, resolved_name);

    Effect::new(move |_| {
        let _ = value_stored.get_value().get();
        validate.run(None);
    });

    let on_shortcut = Callback::new(move |dt: orbital_base_components::OrbitalDateTime| {
        value_stored.with_value(|v| v.set(Some(dt.start_of_day())));
        validate.run(None);
    });

    let chrome_labels = move || {
        let strings = use_datetime_locale_strings();
        CalendarChromeLabels {
            weekday_short: strings.weekday_header_labels(),
            today: strings.today_label,
            previous_month: strings.previous_month_label,
            next_month: strings.next_month_label,
        }
    };

    let root_class = move || {
        let mut parts = vec![layout_root_classes(theme_options.get().density)];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            {move || {
                if let Some(children) = toolbar_children.get_value() {
                    let children = children.clone();
                    children().into_any()
                } else {
                    ().into_any()
                }
            }}
            {move || {
                if shortcuts.get().is_empty() {
                    ().into_any()
                } else {
                    view! {
                        <PickerShortcutsBar
                            shortcuts=shortcuts
                            disabled=disabled
                            on_select=on_shortcut
                        />
                    }
                    .into_any()
                }
            }}
            {move || {
                view! {
                    <Calendar
                        bind=CalendarBind { value: value_stored.get_value() }
                        appearance=CalendarAppearance {
                            timezone: timezone.get(),
                            min_date: min_date.get(),
                            max_date: max_date.get(),
                        }
                        day=day.clone()
                        chrome_labels=chrome_labels()
                    />
                }
            }}
            {move || {
                if let Some(children) = action_bar_children.get_value() {
                    let children = children.clone();
                    children().into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
