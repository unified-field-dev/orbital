use leptos::prelude::*;
use uuid::Uuid;

use super::field_validation::FieldValidationState;

#[derive(Clone)]
pub struct FieldInjection {
    pub(crate) id: StoredValue<String>,
    pub(crate) name: MaybeProp<String>,
    pub(crate) label: MaybeProp<String>,
    pub(crate) validation_state: RwSignal<Option<FieldValidationState>>,
}

impl FieldInjection {
    pub fn new(
        id: StoredValue<String>,
        name: MaybeProp<String>,
        label: MaybeProp<String>,
        validation_state: RwSignal<Option<FieldValidationState>>,
    ) -> Self {
        Self {
            id,
            name,
            label,
            validation_state,
        }
    }

    pub fn use_context() -> Option<Self> {
        use_context()
    }

    pub fn id(&self) -> Option<String> {
        if self.label.with(|l| l.is_some()) {
            Some(self.id.get_value())
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.get()
    }

    pub fn use_id_and_name(
        id: MaybeProp<String>,
        name: MaybeProp<String>,
    ) -> (Signal<Option<String>>, Signal<Option<String>>) {
        let field_injection = Self::use_context();
        let id = Signal::derive(move || {
            if let Some(id) = id.get() {
                return Some(id);
            }
            field_injection.as_ref()?.id()
        });

        let field_injection = Self::use_context();
        let name = Signal::derive(move || {
            if let Some(name) = name.get() {
                return Some(name);
            }
            field_injection.as_ref()?.name()
        });

        (id, name)
    }

    pub fn update_validation_state(&self, state: Result<(), FieldValidationState>) {
        let state = state.err();
        self.validation_state.try_maybe_update(|validation_state| {
            if validation_state == &state {
                (false, ())
            } else {
                *validation_state = state;
                (true, ())
            }
        });
    }
}

pub fn new_field_id() -> String {
    Uuid::new_v4().to_string()
}
