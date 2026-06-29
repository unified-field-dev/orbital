use leptos::{
    logging::debug_warn,
    prelude::{
        ArcReadSignal, ArcRwSignal, ArcWriteSignal, Get, GetUntracked, RwSignal, Storage,
        SyncStorage, Update,
    },
};

/// Imperative handle to a child component instance (e.g. focus a button).
#[derive(Clone, Copy)]
pub struct ComponentRef<T, S = SyncStorage>(RwSignal<Option<T>, S>);

impl<T> Default for ComponentRef<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self(RwSignal::new(None))
    }
}

impl<T> ComponentRef<T>
where
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, S> ComponentRef<T, S>
where
    T: Clone + 'static,
    S: Storage<ArcRwSignal<Option<T>>> + Storage<ArcReadSignal<Option<T>>>,
{
    pub fn get(&self) -> Option<T> {
        self.0.get()
    }

    pub fn try_get(&self) -> Option<T> {
        self.0.try_get().flatten()
    }

    pub fn get_untracked(&self) -> Option<T> {
        self.0.get_untracked()
    }

    pub fn try_get_untracked(&self) -> Option<T> {
        self.0.try_get_untracked().flatten()
    }
}

impl<T, S> ComponentRef<T, S>
where
    T: 'static,
    S: Storage<ArcRwSignal<Option<T>>> + Storage<ArcWriteSignal<Option<T>>>,
{
    pub fn load(&self, comp: T) {
        self.0.update(|current| {
            if current.is_some() {
                debug_warn!(
                    "You are setting a ComponentRef that has already been filled. \
                     It's possible this is intentional."
                );
            }
            *current = Some(comp);
        });
    }
}
