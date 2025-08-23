use crate::prelude::*;

/// The atomic flag
#[derive(Clone)]
pub struct AtomFlag {
    state: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl AtomFlag {
    /// Creates a new flag
    pub fn new(initial: bool) -> Self {
        AtomFlag {
            state: Arc::new(AtomicBool::new(initial)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Check state for 'true'
    pub fn is_true(&self) -> bool {
        self.get()
    }

    /// Check state for 'false'
    pub fn is_false(&self) -> bool {
        !self.get()
    }

    /// Get actual state
    pub fn get(&self) -> bool {
        self.state.load(Ordering::SeqCst)
    }

    /// Set a new state
    pub fn set(&self, value: bool) {
        self.state.store(value, Ordering::SeqCst);
        self.notify.notify_waiters();
    }

    /// Wait state change
    pub async fn wait(&self, value: bool) {
        loop {
            if self.get() == value {
                break;
            }

            self.notify.notified().await;
        }
    }
    
    /// Wait & swap state
    pub async fn swap(&self, value: bool) {
        self.wait(!value).await;
        self.set(value);
    }
}

impl ::std::fmt::Debug for AtomFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.get())
    }
}

impl ::std::fmt::Display for AtomFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.get())
    }
}

impl ::std::cmp::Eq for AtomFlag {}

impl ::std::cmp::PartialEq for AtomFlag {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl ::std::cmp::PartialEq<bool> for AtomFlag {
    fn eq(&self, other: &bool) -> bool {
        &self.get() == other
    }
}

impl ::std::convert::From<bool> for AtomFlag {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl ::std::convert::Into<bool> for AtomFlag {
    fn into(self) -> bool {
        self.get()
    }
}
