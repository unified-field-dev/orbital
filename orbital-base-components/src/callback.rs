use std::{ops::Deref, sync::Arc};

/// Cloneable `Send + Sync` handler for component props and overlay callbacks.
#[derive(Clone)]
pub struct Handler<A = (), R = ()>(Arc<dyn Fn(A) -> R + Send + Sync + 'static>);

impl Handler<(), ()> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Arc::new(move |_| f()))
    }
}

impl<A, R> Handler<A, R> {
    pub fn with<F>(f: F) -> Self
    where
        F: Fn(A) -> R + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }

    pub fn run(&self, arg: A) -> R {
        (self.0)(arg)
    }
}

impl<A> Handler<A, ()> {
    pub fn on<F>(f: F) -> Self
    where
        F: Fn(A) + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl<A, R> Deref for Handler<A, R> {
    type Target = Arc<dyn Fn(A) -> R + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A> From<F> for Handler<A, ()>
where
    F: Fn(A) + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::on(value)
    }
}
