use orbital_theme::Density;
use std::sync::OnceLock;
use turf::inline_style_sheet_values;

pub struct PickerClassNames {
    pub layout: String,
    pub date_field: String,
    pub time_field: String,
    pub datetime_field: String,
    pub segment: String,
    pub separator: String,
}

static PICKER_STYLES: OnceLock<(&'static str, PickerClassNames)> = OnceLock::new();

fn picker_styles() -> (&'static str, PickerClassNames) {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orb-picker-layout {
            display: flex;
            flex-direction: column;
            width: fit-content;
        }

        .orb-date-range-field,
        .orb-time-range-field,
        .orb-datetime-range-field {
            display: inline-flex;
            align-items: center;
            gap: var(--orb-space-inline-sm);
            flex-wrap: wrap;
        }

        .orb-picker-range-field__separator {
            color: var(--orb-color-text-secondary);
            user-select: none;
            flex-shrink: 0;
        }

        .orb-picker-datetime-field__row {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            align-items: flex-start;
            gap: var(--orb-space-inline-md);
            width: fit-content;
            max-width: 100%;
        }

        .orb-picker-datetime-picker__row {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            align-items: flex-start;
            gap: var(--orb-space-inline-md);
            width: fit-content;
            max-width: 100%;
        }

        .orb-picker-datetime-picker__row > .orbital-date-picker,
        .orb-picker-datetime-picker__row > .orbital-time-picker {
            width: auto;
        }

        .orb-picker-datetime-picker__row > .orbital-date-picker {
            flex: 1 1 280px;
            min-width: 0;
        }

        .orb-picker-datetime-picker__row > .orbital-time-picker {
            flex: 0 1 220px;
            min-width: 0;
        }

        .orb-picker-datetime-range-picker__row {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            align-items: flex-start;
            gap: var(--orb-space-inline-md);
            margin-top: var(--orb-space-block-md);
            width: 100%;
        }

        .orb-picker-datetime-range-picker__row > .orb-picker-layout {
            flex: 1 1 320px;
            min-width: 0;
        }

        .orb-picker-layout-density-compact .orbital-calendar-item {
            padding: 4px 6px;
        }

        .orb-picker-layout-density-compact .orbital-calendar {
            height: 560px;
        }

        .orb-picker-layout-density-spacious .orbital-calendar-item {
            padding: 12px 16px;
        }

        .orb-picker-layout-density-spacious .orbital-calendar {
            height: 840px;
        }

        .orb-picker-shortcuts {
            margin-bottom: var(--orb-space-block-sm);
            padding-bottom: var(--orb-space-block-sm);
            border-bottom: 1px solid var(--orb-color-border-subtle);
        }
    };

    let class_names = PickerClassNames {
        layout: "orb-picker-layout".to_string(),
        date_field: "orb-date-field".to_string(),
        time_field: "orb-time-field".to_string(),
        datetime_field: "orb-datetime-field".to_string(),
        segment: "orb-date-field__segment".to_string(),
        separator: "orb-date-field__separator".to_string(),
    };

    (style_sheet, class_names)
}

/// Injected picker stylesheet and stable class names.
pub fn picker_style_sheet() -> &'static str {
    PICKER_STYLES.get_or_init(picker_styles).0
}

/// Stable class names for picker shells and segmented fields.
pub fn picker_class_names() -> &'static PickerClassNames {
    &PICKER_STYLES.get_or_init(picker_styles).1
}

/// Density modifier class for picker layout roots.
pub fn picker_density_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orb-picker-layout--density-compact",
        Density::Default => "",
        Density::Spacious => "orb-picker-layout--density-spacious",
    }
}

/// Inner density class for descendant selectors on calendar/field nodes.
pub fn picker_density_inner_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orb-picker-layout-density-compact",
        Density::Default => "",
        Density::Spacious => "orb-picker-layout-density-spacious",
    }
}

/// Build root layout class list for [`DateTimePicker`].
pub fn datetime_picker_root_classes(density: Density) -> String {
    format!(
        "{} orb-picker-datetime-picker",
        layout_root_classes(density)
    )
}

/// Row class pairing date and time segmented fields side by side.
pub fn datetime_field_row_class() -> &'static str {
    "orb-picker-datetime-field__row"
}

/// Row class pairing date and time pickers side by side.
pub fn datetime_picker_row_class() -> &'static str {
    "orb-picker-datetime-picker__row"
}

/// Row class for start/end datetime pickers in range pickers.
pub fn datetime_range_picker_row_class() -> &'static str {
    "orb-picker-datetime-range-picker__row"
}

/// Build root layout class list for a picker shell.
pub fn layout_root_classes(density: Density) -> String {
    let names = picker_class_names();
    let mut parts = vec![names.layout.clone()];
    let modifier = picker_density_class(density);
    if !modifier.is_empty() {
        parts.push(modifier.to_string());
    }
    let inner = picker_density_inner_class(density);
    if !inner.is_empty() {
        parts.push(inner.to_string());
    }
    parts.join(" ")
}

/// Map theme density to core [`Input`](orbital_core_components::Input) size modifiers.
pub fn input_size_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orbital-input--small",
        Density::Default => "",
        Density::Spacious => "orbital-input--large",
    }
}

/// Build field root class list for combined date-time segmented fields.
pub fn datetime_field_root_classes(density: Density) -> String {
    let names = picker_class_names();
    let mut parts = vec!["orbital-input".to_string(), names.datetime_field.clone()];
    let size = input_size_class(density);
    if !size.is_empty() {
        parts.push(size.to_string());
    }
    parts.join(" ")
}

/// Segment class for datetime combined fields.
pub fn datetime_segment_class() -> &'static str {
    "orb-datetime-field__segment"
}

/// Separator class for datetime combined fields.
pub fn datetime_separator_class() -> &'static str {
    "orb-datetime-field__separator"
}

/// Build field root class list for segmented date/time fields.
pub fn field_root_classes(is_time: bool, density: Density) -> String {
    let names = picker_class_names();
    let field = if is_time {
        names.time_field.clone()
    } else {
        names.date_field.clone()
    };
    let mut parts = vec!["orbital-input".to_string(), field];
    let size = input_size_class(density);
    if !size.is_empty() {
        parts.push(size.to_string());
    }
    parts.join(" ")
}

/// Segment and separator classes for time fields (time-specific BEM suffix).
pub fn time_segment_class() -> &'static str {
    "orb-time-field__segment"
}

pub fn time_separator_class() -> &'static str {
    "orb-time-field__separator"
}

/// Build root class list for analog time clock surfaces.
pub fn time_clock_root_classes(density: Density) -> String {
    let mut parts = vec!["orb-picker-time-clock".to_string()];
    let modifier = picker_density_class(density);
    if !modifier.is_empty() {
        parts.push(modifier.to_string());
    }
    let inner = picker_density_inner_class(density);
    if !inner.is_empty() {
        parts.push(inner.to_string());
    }
    parts.join(" ")
}

/// Build root class list for range field wrappers.
pub fn range_field_root_classes(kind: &str, density: Density) -> String {
    let mut parts = vec![kind.to_string()];
    let inner = picker_density_inner_class(density);
    if !inner.is_empty() {
        parts.push(inner.to_string());
    }
    parts.join(" ")
}
