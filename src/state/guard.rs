use crate::prelude::*;

/// The state mutex guard
pub struct AtomStateGuard<'a, T: Clone> {
    pub(super) lock: RwLockWriteGuard<'a, Arc<T>>,
    pub(super) swap: Arc<ArcSwapAny<Arc<T>>>,
    pub(super) data: T,
}

impl<'a, T: Clone> ::std::ops::Drop for AtomStateGuard<'a, T> {
    fn drop(self: &mut Self) {
        let data = Arc::new(self.data.clone());
        
        *self.lock = data.clone();
        self.swap.store(data);
    }
}

impl<'a, T: Clone> ::std::ops::Deref for AtomStateGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T: Clone> ::std::ops::DerefMut for AtomStateGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<'a, T: Clone + ::std::fmt::Debug> ::std::fmt::Debug for AtomStateGuard<'a, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.data)
    }
}

impl<'a, T: Clone + ::std::fmt::Display> ::std::fmt::Display for AtomStateGuard<'a, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.data)
    }
}
