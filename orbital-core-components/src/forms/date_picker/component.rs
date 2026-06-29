use leptos::prelude::*;
use orbital_base_components::{
    format_datetime, parse_datetime, BaseDatePicker, DatePickerRuleTrigger, FieldInjection,
    Handler, InputType, MonthButtonRenderProps, MonthButtonRenderer, OrbitalDateTime, Rule,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::date_picker_styles;
use super::types::{DatePickerAppearance, DatePickerBind};
use crate::forms::calendar::CalendarMonthButtonProps;
use crate::forms::datetime_bridge::use_unix_bridge;
use crate::{Icon, Input, InputAppearance, InputBind, InputEvents, InputSuffix};

/// Labeled date field with an anchored calendar panel, bound to [`OrbitalDateTime`].
///
/// DatePicker is a labeled-ready date field: users type a locale-formatted date or open the calendar panel to pick a day. The bound value is [`OrbitalDateTime`] at start-of-day in your chosen [`DatetimeTimezone`](orbital_base_components::DatetimeTimezone). Convert at API boundaries via `ToUnixSeconds` or `ToIso8601`. Display format is controlled via [`DatetimeFormat`](orbital_base_components::DatetimeFormat). Pair with [`TimePicker`](crate::TimePicker) when you need time-of-day. For an inline month grid without a text field, use [`Calendar`](crate::Calendar) instead.
///
/// # When to use
///
/// - Single-date form fields (start date, birth date, due date)
/// - Filter bars where a calendar affordance beats free-text parsing alone
/// - Flows that need locale-specific display format while storing a canonical timestamp
///
/// # DatePicker vs Calendar
///
/// | Need | Component |
/// |------|-----------|
/// | Text field + anchored calendar panel | `DatePicker` |
/// | Inline month grid on the page | `Calendar` |
///
/// # Usage
///
/// 1. Create an `Option<OrbitalDateTime>` signal and pass it via [`DatePickerBind`].
/// 2. Set `appearance.format` and `appearance.timezone` when display differs from defaults.
/// 3. Wrap in [`Field`](crate::Field) with `DatePickerRule::required` when a date is mandatory.
/// 4. Wrap preview examples in a native element with `data-testid` for E2E selectors.
///
/// # Lifecycle
///
/// - **Open:** the calendar panel opens when the trigger is clicked; it closes after a day or shortcut is selected.
/// - **Value (panel):** selecting a day or shortcut commits immediately and closes the panel.
/// - **Value (typed):** the text input commits on blur after parsing.
/// - **Validation:** `Blur` runs after typed input; `Change` runs after panel or shortcut selection.
///
/// # Timezone
///
/// `appearance.timezone` controls how typed input is parsed and how the panel maps days to
/// [`OrbitalDateTime`] start-of-day. The bound value retains its own `OrbitalDateTime::timezone()`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind `value` with [`DatePickerBind`] for two-way sync
/// * Use [`Field`](crate::Field) + `DatePickerRule::required` when a date is mandatory
/// * Pick `DatetimeFormat` and `DatetimeTimezone` to match user locale expectations
///
/// ## Don'ts
///
/// * Do not use for time-only selection — prefer [`TimePicker`](crate::TimePicker)
/// * Do not use for inline grids without a text field — prefer [`Calendar`](crate::Calendar)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default
/// Empty date field with calendar suffix button.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! { <div data-testid="date-picker-preview"><DatePicker bind=value /></div> }
/// ```
///
/// ## Preselected unix value
/// Existing unix seconds populate the text input on mount.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! { <div data-testid="DP-02"><DatePicker bind=value /></div> }
/// ```
///
/// ## Bind readout
/// Panel selection updates the bound unix signal and readout text.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="DP-03">
///         <DatePicker bind=value />
///         <div data-testid="DP-03-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </div>
/// }
/// ```
///
/// ## ISO format
/// Year-month-day display for locales that prefer ISO ordering.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="DP-04">
///         <DatePicker bind=value appearance=DatePickerAppearance { format: Signal::from(DatetimeFormat::IsoDate), ..Default::default() } />
///     </div>
/// }
/// ```
///
/// ## UTC timezone
/// Formatting uses UTC regardless of browser local offset.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="DP-05">
///         <DatePicker bind=value appearance=DatePickerAppearance { timezone: Signal::from(DatetimeTimezone::Utc), ..Default::default() } />
///     </div>
/// }
/// ```
///
/// ## Fixed offset timezone
/// Display anchored to a fixed offset from UTC (e.g. EST).
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="DP-06">
///         <DatePicker bind=value appearance=DatePickerAppearance { timezone: Signal::from(DatetimeTimezone::FixedOffset(-5 * 3600)), ..Default::default() } />
///     </div>
/// }
/// ```
///
/// ## Required in Field
/// Required rule shows validation messaging on blur when empty.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// use orbital_base_components::DatePickerRule;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="DP-07">
///         <Field label="Start date" name="start_date" required=true>
///             <DatePicker bind=DatePickerBind { value: value.into(), rules: vec![DatePickerRule::required(required)], ..Default::default() } />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Input and calendar button cannot be interacted with.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="DP-08">
///         <DatePicker bind=value appearance=DatePickerAppearance { disabled: Signal::from(true), ..Default::default() } />
///     </div>
/// }
/// ```
///
/// ## Format and timezone readout
/// Default US format with UTC timezone reflected in the bound readout.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, DatetimeTimezone, format_datetime, ToUnixSeconds, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid date"),
/// ));
/// view! {
///     <div data-testid="DP-09">
///         <DatePicker bind=value appearance=DatePickerAppearance { timezone: Signal::from(DatetimeTimezone::Utc), ..Default::default() } />
///         <div data-testid="DP-09-TEXT">{move || value.get().map(|v| format_datetime(v, DatetimeFormat::default())).unwrap_or_else(|| "none".to_string())}</div>
///     </div>
/// }
/// ```
///
/// ## Min/max validation
/// Field shows an error when the typed date falls outside the allowed range.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
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
///     <div data-testid="DP-10">
///         <Field label="Event date" name="event_date">
///             <DatePicker
///                 bind=DatePickerBind { value: value.into(), rules: vec![DatePickerRule::min_date(min), DatePickerRule::max_date(max)], ..Default::default() }
///                 appearance=DatePickerAppearance { min_date: min, max_date: max, ..Default::default() }
///             />
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-picker",
    preview_label = "Date Picker",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DatePicker(
    /// Value binding, field identity, and validation rules.
    #[prop(optional, into)]
    bind: DatePickerBind,
    /// Display format, timezone, placeholder, disabled/readonly, and panel placement.
    #[prop(optional, into)]
    appearance: DatePickerAppearance,
    /// Extra CSS class names merged onto the root wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-date-picker", date_picker_styles());

    let DatePickerBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let DatePickerAppearance {
        format,
        timezone,
        placeholder,
        disabled,
        readonly,
        placement,
        min_date,
        max_date,
        shortcuts,
        month_button,
    } = appearance;

    let (resolved_id, resolved_name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, resolved_name);
    let unix_value = use_unix_bridge(value.get_value(), timezone);

    let text_value = RwSignal::new(String::new());
    let sync_display = Callback::new(move |_| {
        let display = value
            .get_value()
            .get_untracked()
            .map(|dt| format_datetime(dt, format.get_untracked()))
            .unwrap_or_default();
        text_value.set(display);
    });

    let sync_display_effect = sync_display;
    Effect::new(move |_| {
        let _ = value.get_value().get();
        let _ = format.get();
        let _ = timezone.get();
        sync_display_effect.run(());
    });

    let validate_blur = validate;
    let on_blur = move |_| {
        if disabled.get_untracked() || readonly.get_untracked() {
            validate_blur.run(Some(DatePickerRuleTrigger::Blur));
            return;
        }

        let trimmed = text_value.get_untracked().trim().to_string();
        if trimmed.is_empty() {
            value.with_value(|v| v.set(None));
            validate_blur.run(Some(DatePickerRuleTrigger::Blur));
            return;
        }

        if let Some(parsed) =
            parse_datetime(&trimmed, format.get_untracked(), timezone.get_untracked())
        {
            let normalized = parsed.start_of_day();
            value.with_value(|v| v.set(Some(normalized)));
            text_value.set(format_datetime(normalized, format.get_untracked()));
        } else {
            sync_display.run(());
        }

        validate_blur.run(Some(DatePickerRuleTrigger::Blur));
    };

    let validate_change = validate;
    let on_select = Callback::new(move |_: i64| {
        validate_change.run(Some(DatePickerRuleTrigger::Change));
    });

    let validate_shortcut = validate;
    let sync_display_shortcut = sync_display;
    let on_shortcut = Callback::new(move |_dt: OrbitalDateTime| {
        sync_display_shortcut.run(());
        validate_shortcut.run(Some(DatePickerRuleTrigger::Change));
    });

    let input_bind = InputBind {
        value: text_value.into(),
        id: resolved_id.into(),
        name: resolved_name.into(),
        ..Default::default()
    };
    let input_appearance = InputAppearance {
        input_type: Signal::from(InputType::Text),
        placeholder,
        disabled,
        readonly,
        ..Default::default()
    };
    let input_events = InputEvents {
        on_blur: Some(Handler::with(on_blur)),
        ..Default::default()
    };
    let panel_disabled = Signal::derive(move || disabled.get() || readonly.get());
    let suffix_disabled = panel_disabled;
    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-date-picker".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let base_month_button = month_button.map(|renderer| {
        std::sync::Arc::new(move |props: MonthButtonRenderProps| {
            renderer(CalendarMonthButtonProps {
                month: props.month,
                label: props.label,
                selected: props.selected,
                on_select: props.on_select,
            })
        }) as MonthButtonRenderer
    });

    view! {
        <BaseDatePicker
            class=wrapper_class
            panel_class="orbital-date-picker__panel"
            value=unix_value
            timezone=timezone
            min_date=min_date
            max_date=max_date
            shortcuts=shortcuts
            disabled=panel_disabled
            placement=placement
            on_select=on_select
            on_shortcut=on_shortcut
            month_button=base_month_button
        >
            <div class="orbital-date-picker__trigger">
                <Input bind=input_bind appearance=input_appearance events=input_events>
                    <InputSuffix slot>
                        <button
                            type="button"
                            class="orbital-date-picker__suffix-button"
                            aria-label="Open calendar"
                            tabindex="-1"
                            disabled=move || suffix_disabled.get()
                            on:mousedown=|e| e.prevent_default()
                        >
                            <Icon icon=icondata::AiCalendarOutlined />
                        </button>
                    </InputSuffix>
                </Input>
            </div>
        </BaseDatePicker>
    }
}
