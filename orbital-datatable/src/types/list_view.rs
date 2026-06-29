/// List view card layout configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListViewConfig {
    /// Primary field shown as card title.
    pub primary_field: String,
    /// Secondary fields shown below the title.
    pub secondary_fields: Vec<String>,
}

impl ListViewConfig {
    pub fn new(primary_field: impl Into<String>) -> Self {
        Self {
            primary_field: primary_field.into(),
            secondary_fields: Vec::new(),
        }
    }

    pub fn with_secondary_fields(mut self, fields: Vec<String>) -> Self {
        self.secondary_fields = fields;
        self
    }
}
