use crate::prelude::*;

/// The static boolean flag
#[derive(Clone)]
pub struct AtomicFlag {
    state: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl AtomicFlag {
    /// Creates a new static flag
    pub fn new(initial: bool) -> Self {
        AtomicFlag {
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

impl ::std::fmt::Debug for AtomicFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", &self.get())
    }
}

impl ::std::fmt::Display for AtomicFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.get())
    }
}

impl ::std::cmp::Eq for AtomicFlag {}

impl ::std::cmp::PartialEq for AtomicFlag {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl ::std::cmp::PartialEq<bool> for AtomicFlag {
    fn eq(&self, other: &bool) -> bool {
        &self.get() == other
    }
}

impl ::std::convert::From<bool> for AtomicFlag {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl ::std::convert::Into<bool> for AtomicFlag {
    fn into(self) -> bool {
        self.get()
    }
}
