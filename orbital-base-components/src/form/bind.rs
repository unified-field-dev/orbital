use leptos::prelude::*;
use reactive_stores::Field;

/// Two-way value handle for **form controls only**.
///
/// Accepts store fields, standalone signals, or plain initial values (preview ergonomics).
#[derive(Clone)]
pub enum FormBind<T: 'static> {
    Signal(RwSignal<T>),
    Field(Field<T>),
}

impl<T: Send + Sync + 'static> FormBind<T> {
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Signal(signal) => signal.get(),
            Self::Field(field) => field.get(),
        }
    }

    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Signal(signal) => signal.get_untracked(),
            Self::Field(field) => field.get_untracked(),
        }
    }

    pub fn set(&self, value: T) {
        match self {
            Self::Signal(signal) => signal.set(value),
            Self::Field(field) => field.set(value),
        }
    }

    pub fn update(&self, f: impl FnOnce(&mut T))
    where
        T: Clone,
    {
        match self {
            Self::Signal(signal) => signal.update(f),
            Self::Field(field) => field.update(f),
        }
    }
}

impl<T> From<RwSignal<T>> for FormBind<T> {
    fn from(signal: RwSignal<T>) -> Self {
        Self::Signal(signal)
    }
}

impl<T> From<Field<T>> for FormBind<T> {
    fn from(field: Field<T>) -> Self {
        Self::Field(field)
    }
}

impl<T: Send + Sync + 'static> From<T> for FormBind<T> {
    fn from(value: T) -> Self {
        Self::Signal(RwSignal::new(value))
    }
}

impl<T: Default + Send + Sync + 'static> Default for FormBind<T> {
    fn default() -> Self {
        Self::Signal(RwSignal::new(T::default()))
    }
}
