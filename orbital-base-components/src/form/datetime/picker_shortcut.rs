use super::OrbitalDateTime;

/// Shortcut preset shown on calendar/picker shortcut bars.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PickerShortcut {
    pub label: String,
    pub value: OrbitalDateTime,
}

impl PickerShortcut {
    pub fn new(label: impl Into<String>, value: OrbitalDateTime) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}
