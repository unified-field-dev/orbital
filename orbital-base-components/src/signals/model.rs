use leptos::prelude::*;

/// Cloneable handle to a reactive value for two-way props.
#[derive(Clone)]
pub struct SignalModel<T>(RwSignal<T>);

impl<T> SignalModel<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(value: T) -> Self {
        Self(RwSignal::new(value))
    }

    pub fn from_rw(signal: RwSignal<T>) -> Self {
        Self(signal)
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.0.get()
    }

    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        self.0.get_untracked()
    }

    pub fn set(&self, value: T) {
        self.0.set(value);
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.0.update(f);
    }

    pub fn with<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.0.with(f)
    }

    pub fn with_untracked<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.0.with_untracked(f)
    }

    pub fn read_only(&self) -> ReadSignal<T> {
        self.0.read_only()
    }

    pub fn write_only(&self) -> WriteSignal<T> {
        self.0.write_only()
    }

    pub fn split(&self) -> (ReadSignal<T>, WriteSignal<T>) {
        self.0.split()
    }
}

impl<T> From<RwSignal<T>> for SignalModel<T>
where
    T: Send + Sync + 'static,
{
    fn from(signal: RwSignal<T>) -> Self {
        Self(signal)
    }
}

impl<T> From<SignalModel<T>> for RwSignal<T>
where
    T: Send + Sync + 'static,
{
    fn from(model: SignalModel<T>) -> Self {
        model.0
    }
}
