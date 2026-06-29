use leptos::prelude::*;

/// Configuration for [`Upload`](crate::Upload).
#[derive(Clone, Default)]
pub struct UploadConfig {
    /// `accept` attribute filter (for example `"image/*"`).
    pub accept: Signal<String>,
    /// Allow selecting multiple files.
    pub multiple: Signal<bool>,
    /// Input element id for form association.
    pub id: MaybeProp<String>,
    /// Form field name submitted with the file.
    pub name: MaybeProp<String>,
}

impl UploadConfig {
    pub fn accept(accept: impl Into<String>) -> Self {
        Self {
            accept: Signal::stored(accept.into()),
            ..Default::default()
        }
    }
}
