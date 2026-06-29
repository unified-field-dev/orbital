#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferListItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl TransferListItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
