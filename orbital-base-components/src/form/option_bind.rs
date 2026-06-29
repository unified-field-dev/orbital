use leptos::prelude::*;
use reactive_stores::Field;

/// Two-way optional value handle for form controls.
#[derive(Clone)]
pub enum OptionBind<T: 'static> {
    Signal(RwSignal<Option<T>>),
    Field(Field<Option<T>>),
}

impl<T: Send + Sync + 'static> OptionBind<T> {
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Signal(signal) => signal.get(),
            Self::Field(field) => field.get(),
        }
    }

    pub fn get_untracked(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Signal(signal) => signal.get_untracked(),
            Self::Field(field) => field.get_untracked(),
        }
    }

    pub fn set(&self, value: Option<T>) {
        match self {
            Self::Signal(signal) => signal.set(value),
            Self::Field(field) => field.set(value),
        }
    }

    pub fn update(&self, f: impl FnOnce(&mut Option<T>))
    where
        T: Clone,
    {
        match self {
            Self::Signal(signal) => signal.update(f),
            Self::Field(field) => field.update(f),
        }
    }
}

impl<T> From<RwSignal<Option<T>>> for OptionBind<T> {
    fn from(signal: RwSignal<Option<T>>) -> Self {
        Self::Signal(signal)
    }
}

impl<T> From<Field<Option<T>>> for OptionBind<T> {
    fn from(field: Field<Option<T>>) -> Self {
        Self::Field(field)
    }
}

impl<T: Send + Sync + 'static> From<Option<T>> for OptionBind<T> {
    fn from(value: Option<T>) -> Self {
        Self::Signal(RwSignal::new(value))
    }
}

impl<T: Send + Sync + 'static> From<T> for OptionBind<T> {
    fn from(value: T) -> Self {
        Self::Signal(RwSignal::new(Some(value)))
    }
}

impl<T: Send + Sync + 'static> Default for OptionBind<T> {
    fn default() -> Self {
        Self::Signal(RwSignal::new(None))
    }
}
