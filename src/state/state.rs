use std::sync::{ Arc, };
use tokio::sync::{ RwLock, RwLockReadGuard, RwLockWriteGuard };

/// The atomic state
#[derive(Clone)]
pub struct AtomState<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> AtomState<T> {
    /// Creates a new state
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    /// Returns a state locked guard
    pub async fn lock(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().await
    }

    /// Returns a state locked guard
    pub fn block_lock(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.blocking_write()
    }

    /// Returns a state value
    pub async fn get(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().await
    }

    /// Returns a state value (with thread blocking)
    pub fn block_get(&self) -> RwLockReadGuard<'_, T> {
        self.inner.blocking_read()
    }

    /// Sets a new value to state
    pub async fn set(&self, value: T) {
        let mut guard = self.inner.write().await;
        *guard = value;
    }

    /// Sets a new value to state (with thread blocking)
    pub fn block_set(&self, value: T) {
        let mut guard = self.inner.blocking_write();
        *guard = value;
    }
}
