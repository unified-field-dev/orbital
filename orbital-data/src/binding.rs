/// Declarative binding from [`crate::Dataset`] fields to chart geometry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChartFieldBinding {
    /// Category or time axis field key.
    pub x_field: Option<String>,
    /// One or more value series field keys.
    pub y_fields: Vec<String>,
    /// Pivot long→wide by grouping on this field (e.g. region column).
    pub series_by_field: Option<String>,
    /// Field key driving per-item color.
    pub color_field: Option<String>,
    /// Field key for data labels.
    pub label_field: Option<String>,
    /// Scatter z-axis / bubble size field key.
    pub size_field: Option<String>,
    /// Scatter point identity field key (falls back to record id when unset).
    pub id_field: Option<String>,
}

impl ChartFieldBinding {
    /// Shorthand for a single category axis and one or more value fields.
    pub fn new(x_field: impl Into<String>, y_fields: Vec<String>) -> Self {
        Self {
            x_field: Some(x_field.into()),
            y_fields,
            ..Default::default()
        }
    }
}
