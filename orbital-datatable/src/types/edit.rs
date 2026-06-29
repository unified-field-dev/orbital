use std::collections::HashMap;

use leptos::prelude::*;
use orbital_data::{DataRecord, DataValue};

use super::DataTableColumnDef;

/// Scope for inline editing: one cell at a time or all editable cells in a row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditMode {
    #[default]
    Cell,
    Row,
}

/// Draft state for a single editable field while a session is active.
#[derive(Clone)]
pub struct EditFieldDraft {
    pub field: String,
    pub original: DataValue,
    pub draft: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
}

/// Active inline edit session for a row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditSession {
    Idle,
    Editing {
        row_id: String,
        mode: EditMode,
        /// Active field in cell mode (all editable fields in row mode).
        active_field: Option<String>,
    },
    Committing {
        row_id: String,
    },
}

/// Runtime edit session storage (draft signals live in the side map).
#[derive(Clone, Copy)]
pub struct EditSessionStore {
    pub session: RwSignal<EditSession>,
    pub drafts: StoredValue<HashMap<String, EditFieldDraft>>,
}

impl Default for EditSessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EditSessionStore {
    pub fn new() -> Self {
        Self {
            session: RwSignal::new(EditSession::Idle),
            drafts: StoredValue::new(HashMap::new()),
        }
    }

    pub fn is_editing_row(&self, row_id: &str) -> bool {
        matches!(
            self.session.get(),
            EditSession::Editing { row_id: id, .. } | EditSession::Committing { row_id: id }
                if id == row_id
        )
    }

    pub fn is_editing_field(&self, row_id: &str, field: &str) -> bool {
        match self.session.get() {
            EditSession::Editing {
                row_id: id,
                mode,
                active_field,
            } if id == row_id => match mode {
                EditMode::Row => self.drafts.with_value(|d| d.contains_key(field)),
                EditMode::Cell => active_field.as_deref() == Some(field),
            },
            EditSession::Committing { row_id: id } if id == row_id => {
                self.drafts.with_value(|d| d.contains_key(field))
            }
            _ => false,
        }
    }

    pub fn clear(&self) {
        self.drafts.set_value(HashMap::new());
        self.session.set(EditSession::Idle);
    }
}

/// Props passed to custom column [`DataTableColumnDef::edit_view`] renderers.
pub struct EditCellProps {
    pub row: DataRecord,
    pub column: DataTableColumnDef,
    pub draft: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub on_commit: Callback<(), ()>,
    pub on_cancel: Callback<(), ()>,
}
