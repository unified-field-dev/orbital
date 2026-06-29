use orbital_data::DataValue;

/// One reversible cell edit pushed onto the undo stack.
#[derive(Clone, Debug, PartialEq)]
pub struct EditHistoryEntry {
    pub row_id: String,
    pub field: String,
    pub before: DataValue,
    pub after: DataValue,
}

/// Undo/redo stacks for inline edits (gated by [`super::DataTableFeatures::UNDO_REDO`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EditHistory {
    pub undo: Vec<EditHistoryEntry>,
    pub redo: Vec<EditHistoryEntry>,
}

impl EditHistory {
    pub fn push(&mut self, entry: EditHistoryEntry) {
        self.undo.push(entry);
        self.redo.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}
