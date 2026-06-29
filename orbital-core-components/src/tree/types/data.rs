use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Data adapter for [`RichTree`](super::rich_tree::RichTree).
#[derive(Clone)]
pub struct RichTreeData<T> {
    pub items: Arc<Vec<T>>,
    pub get_id: Arc<dyn Fn(&T) -> String + Send + Sync>,
    pub get_label: Arc<dyn Fn(&T) -> String + Send + Sync>,
    pub get_children: Arc<dyn Fn(&T) -> Vec<T> + Send + Sync>,
    pub is_disabled: Arc<dyn Fn(&T) -> bool + Send + Sync>,
    pub is_editable: Arc<dyn Fn(&T) -> bool + Send + Sync>,
    pub lazy_fetch:
        Option<Arc<dyn Fn(String) -> Pin<Box<dyn Future<Output = Vec<T>> + Send>> + Send + Sync>>,
    pub virtualize: bool,
}

impl<T: Clone + Send + Sync + 'static> RichTreeData<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items: Arc::new(items),
            get_id: Arc::new(|_: &T| String::new()),
            get_label: Arc::new(|_: &T| String::new()),
            get_children: Arc::new(|_: &T| Vec::new()),
            is_disabled: Arc::new(|_: &T| false),
            is_editable: Arc::new(|_: &T| false),
            lazy_fetch: None,
            virtualize: false,
        }
    }

    pub fn get_id<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> String + Send + Sync + 'static,
    {
        self.get_id = Arc::new(f);
        self
    }

    pub fn get_label<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> String + Send + Sync + 'static,
    {
        self.get_label = Arc::new(f);
        self
    }

    pub fn get_children<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> Vec<T> + Send + Sync + 'static,
    {
        self.get_children = Arc::new(f);
        self
    }

    pub fn is_disabled<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.is_disabled = Arc::new(f);
        self
    }

    pub fn is_editable<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.is_editable = Arc::new(f);
        self
    }

    pub fn with_lazy_fetch<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Vec<T>> + Send + 'static,
    {
        self.lazy_fetch = Some(Arc::new(move |id| Box::pin(f(id))));
        self
    }

    pub fn with_virtualize(mut self, virtualize: bool) -> Self {
        self.virtualize = virtualize;
        self
    }
}
