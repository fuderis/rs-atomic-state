use crate::prelude::*;
use super::*;

/// The atomic state
#[derive(Clone)]
pub struct AtomState<T: Clone> {
    lock: Arc<RwLock<Arc<T>>>,
    swap: Arc<ArcSwapAny<Arc<T>>>,
}

impl<T: Clone> AtomState<T> {
    /// Creates a new state
    pub fn new(value: T) -> Self {
        let arc_val = Arc::new(value);
        
        Self {
            lock: Arc::new(RwLock::new(arc_val.clone())),
            swap: Arc::new(ArcSwapAny::from(arc_val)),
        }
    }

    /// Returns a locked state guard
    pub fn lock(&self) -> AtomStateGuard<'_, T> {
        let data = (*self.get()).clone();
        
        AtomStateGuard {
            lock: self.lock.write().expect(ERR_MSG),
            swap: self.swap.clone(),
            data,
        }
    }

    /// Returns a state value
    pub fn get(&self) -> Arc<T> {
        self.swap.load_full()
    }

    /// Sets a new value to state
    pub fn set(&self, value: T) {
        *self.lock() = value;
    }

    /// Writes data directly
    pub fn map(&self, f: impl FnOnce(&mut T)) {
        let mut lock = self.lock();
        let mut data = (*lock).clone();

        f(&mut data);
        *lock = data;
    }
}
