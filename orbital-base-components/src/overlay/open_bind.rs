use leptos::prelude::*;

/// Two-way open-state handle for modal overlays.
#[derive(Clone, Copy)]
pub enum OpenBind {
    Signal(RwSignal<bool>),
    ReadWrite(ReadSignal<bool>, WriteSignal<bool>),
}

impl OpenBind {
    pub fn get(&self) -> bool {
        match self {
            Self::Signal(signal) => signal.get(),
            Self::ReadWrite(read, _) => read.get(),
        }
    }

    pub fn get_untracked(&self) -> bool {
        match self {
            Self::Signal(signal) => signal.get_untracked(),
            Self::ReadWrite(read, _) => read.get_untracked(),
        }
    }

    pub fn set(&self, value: bool) {
        match self {
            Self::Signal(signal) => signal.set(value),
            Self::ReadWrite(_, write) => write.set(value),
        }
    }

    pub fn signal(&self) -> Signal<bool> {
        match self {
            Self::Signal(signal) => signal.read_only().into(),
            Self::ReadWrite(read, _) => {
                let read = *read;
                Signal::derive(move || read.get())
            }
        }
    }
}

impl From<RwSignal<bool>> for OpenBind {
    fn from(signal: RwSignal<bool>) -> Self {
        Self::Signal(signal)
    }
}

impl From<(ReadSignal<bool>, WriteSignal<bool>)> for OpenBind {
    fn from((read, write): (ReadSignal<bool>, WriteSignal<bool>)) -> Self {
        Self::ReadWrite(read, write)
    }
}

impl From<bool> for OpenBind {
    fn from(value: bool) -> Self {
        Self::Signal(RwSignal::new(value))
    }
}

impl Default for OpenBind {
    fn default() -> Self {
        Self::Signal(RwSignal::new(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rw_signal_round_trip() {
        Owner::new().with(|| {
            let signal = RwSignal::new(false);
            let bind = OpenBind::from(signal);
            bind.set(true);
            assert!(bind.get_untracked());
            bind.set(false);
            assert!(!bind.get_untracked());
        });
    }
}
