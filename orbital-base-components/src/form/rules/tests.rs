use leptos::prelude::*;

use super::date_picker::DatePickerRule;
use super::input::InputRule;
use super::select::SelectRule;
use super::textarea::TextareaRule;
use crate::form::field_validation::FieldValidationState;
use crate::form::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};

fn with_owner<F: FnOnce()>(f: F) {
    Owner::new().with(f);
}

fn run_required<F>(required: bool, value: &str, rule_fn: F) -> Result<(), FieldValidationState>
where
    F: FnOnce(Signal<bool>) -> InputRule,
{
    let required = Signal::from(required);
    let name = Signal::from(None::<String>);
    let rule = rule_fn(required);
    (rule.validator)(&value.to_string(), name)
}

#[test]
fn input_required_empty_when_required() {
    with_owner(|| {
        let result = run_required(true, "", InputRule::required);
        assert!(matches!(result, Err(FieldValidationState::Error(_))));
    });
}

#[test]
fn input_required_ok_when_filled() {
    with_owner(|| {
        let result = run_required(true, "hello", InputRule::required);
        assert!(result.is_ok());
    });
}

#[test]
fn input_required_ok_when_not_required() {
    with_owner(|| {
        let result = run_required(false, "", InputRule::required);
        assert!(result.is_ok());
    });
}

#[test]
fn select_required_empty_when_required() {
    with_owner(|| {
        let required = Signal::from(true);
        let name = Signal::from(None::<String>);
        let rule = SelectRule::required(required);
        let result = (rule.validator)(&String::new(), name);
        assert!(matches!(result, Err(FieldValidationState::Error(_))));
    });
}

fn orbital_date(y: i32, m: u32, d: u32) -> OrbitalDateTime {
    OrbitalDateTime::try_from_unix_seconds(
        chrono::NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp(),
        DatetimeTimezone::Utc,
    )
    .expect("valid")
}

fn run_date_rule<F>(value: Option<OrbitalDateTime>, rule_fn: F) -> Result<(), FieldValidationState>
where
    F: FnOnce() -> DatePickerRule,
{
    let name = Signal::from(None::<String>);
    let rule = rule_fn();
    (rule.validator)(&value, name)
}

#[test]
fn date_picker_min_date_rejects_before_bound() {
    with_owner(|| {
        let min = Signal::from(Some(orbital_date(2025, 6, 1)));
        let result = run_date_rule(Some(orbital_date(2025, 5, 15)), || {
            DatePickerRule::min_date(min)
        });
        assert!(matches!(result, Err(FieldValidationState::Error(_))));
    });
}

#[test]
fn date_picker_min_date_accepts_in_range() {
    with_owner(|| {
        let min = Signal::from(Some(orbital_date(2025, 6, 1)));
        let result = run_date_rule(Some(orbital_date(2025, 6, 15)), || {
            DatePickerRule::min_date(min)
        });
        assert!(result.is_ok());
    });
}

#[test]
fn date_picker_max_date_rejects_after_bound() {
    with_owner(|| {
        let max = Signal::from(Some(orbital_date(2025, 6, 30)));
        let result = run_date_rule(Some(orbital_date(2025, 7, 1)), || {
            DatePickerRule::max_date(max)
        });
        assert!(matches!(result, Err(FieldValidationState::Error(_))));
    });
}

#[test]
fn date_picker_min_max_skip_none_value() {
    with_owner(|| {
        let min = Signal::from(Some(orbital_date(2025, 6, 1)));
        let result = run_date_rule(None, || DatePickerRule::min_date(min));
        assert!(result.is_ok());
    });
}

#[test]
fn textarea_required_with_custom_message() {
    with_owner(|| {
        let required = Signal::from(true);
        let message = Signal::from("Custom".to_string());
        let name = Signal::from(None::<String>);
        let rule = TextareaRule::required_with_message(required, message);
        let result = (rule.validator)(&String::new(), name);
        assert!(matches!(
            result,
            Err(FieldValidationState::Error(msg)) if msg == "Custom"
        ));
    });
}
