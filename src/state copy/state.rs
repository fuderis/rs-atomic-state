use crate::prelude::*;
use super::*;

/// The atomic state
#[derive(Clone)]
pub struct AtomState<T: Clone> {
    write: Arc<Mutex<Arc<T>>>,
    read: Arc<RwLock<Arc<T>>>,
}

impl<T: Clone> AtomState<T> {
    /// Creates a new state
    pub fn new(value: T) -> Self {
        let value = Arc::new(value);
        
        Self {
            write: Arc::new(Mutex::new(value.clone())),
            read: Arc::new(RwLock::new(value)),
        }
    }

    /// Returns a state value
    pub fn get(&self) -> Arc<T> {
        self.read.read().expect(ERR_MSG).clone()
    }

    /// Sets a new value to state
    pub fn set(&self, value: T) {
        let value = Arc::new(value);
        
        *self.write.lock().expect(ERR_MSG) = value.clone();
        *self.read.write().expect(ERR_MSG) = value;
    }

    /// Writes data directly
    pub fn map(&self, f: impl FnOnce(&mut T)) {
        let lock = self.lock();
        let mut data = lock.clone();

        f(&mut data);
    }

    /// Returns a state locked guard
    pub fn lock(&self) -> AtomStateGuard<'_, T> {
        let lock = self.write.lock().expect(ERR_MSG);
        let data = (**lock).clone();
        
        AtomStateGuard {
            write: self.write.lock().expect(ERR_MSG),
            read: self.read.clone(),
            data,
        }
    }
}
