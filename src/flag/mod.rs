pub mod flag;  pub use flag::AtomicFlag;

use crate::prelude::*;

/// A static data alias
pub type Flag = Lazy<AtomicFlag>;

/// Initializes a static data
#[macro_export]
macro_rules! flag {
    ($v:expr) => {{
        ::atomic_state::Lazy::new(|| ::atomic_state::AtomicFlag::new($v))
    }}
}
