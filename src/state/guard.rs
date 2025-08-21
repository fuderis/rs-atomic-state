use crate::prelude::*;
use super::ERR_MSG;

/// The state mutex guard
pub struct AtomicStateGuard<T: Clone> {
    pub(super) guard: OwnedMutexGuard<T>,
    pub(super) fast: Arc<RwLock<T>>,
}

impl<T: Clone> ::std::ops::Drop for AtomicStateGuard<T> {
    fn drop(self: &mut Self) {
        *self.fast.write().expect(ERR_MSG) = (*self.guard).clone();
    }
}

impl<T: Clone> ::std::ops::Deref for AtomicStateGuard<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T: Clone> ::std::ops::DerefMut for AtomicStateGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}

impl<T: Clone + ::std::fmt::Debug> ::std::fmt::Debug for AtomicStateGuard<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.guard)
    }
}

impl<T: Clone + ::std::fmt::Display> ::std::fmt::Display for AtomicStateGuard<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.guard)
    }
}
