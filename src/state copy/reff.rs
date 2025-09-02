use crate::prelude::*;

/// The state mutex read
pub struct AtomStateRef<'a, T: Clone> {
    pub(super) read: RwLockReadGuard<'a, T>,
}

/* impl<'a, T: Clone> ::std::ops::Drop for AtomStateRef<'a, T> {
    fn drop(self: &mut Self) {
    }
} */

impl<'a, T: Clone> ::std::ops::Deref for AtomStateRef<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.read
    }
}

impl<'a, T: Clone + ::std::fmt::Debug> ::std::fmt::Debug for AtomStateRef<'a, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.read)
    }
}

impl<'a, T: Clone + ::std::fmt::Display> ::std::fmt::Display for AtomStateRef<'a, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.read)
    }
}
