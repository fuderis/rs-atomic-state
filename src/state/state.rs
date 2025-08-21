use crate::prelude::*;
use super::*;

/// The static data wrapper
#[derive(Clone)]
pub struct AtomicState<T: Clone> {
    pub(super) inner: Arc<Mutex<T>>,
    pub(super) fast: Arc<RwLock<T>>,
}

impl<T: Clone + 'static> AtomicState<T> {
    /// Creates a new static data
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value.clone())),
            fast: Arc::new(RwLock::new(value)),
        }
    }
    
    /// Lock a data mutex
    pub async fn lock(&self) -> AtomicStateGuard<T> {
        AtomicStateGuard {
            guard: self.inner.clone().lock_owned().await,
            fast: self.fast.clone(),
        }
    }

    /// Get a last data
    pub fn get(&self) -> T {
        self.fast.read().expect(ERR_MSG).clone()
    }

    /// Set a new data
    pub async fn set(&self, value: T) {
        *self.inner.lock().await = value.clone();
        self.set_fast(value);
    }

    /// Set fast value
    fn set_fast(&self, value: T) {
        *self.fast.write().expect(ERR_MSG) = value;
    }

    /// Read/write data directly
    pub async fn map(&self, f: impl FnOnce(&mut T)) {
        f(&mut *self.inner.lock().await);
        self.update().await;
    }

    /// Update fast value
    pub async fn update(&self) {
        self.set_fast(self.inner.lock().await.clone());
    }
}

impl<T: Clone + ::std::fmt::Debug + 'static> ::std::fmt::Debug for AtomicState<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

impl<T: Clone + ::std::fmt::Display + 'static> ::std::fmt::Display for AtomicState<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.get())
    }
}
