pub mod flag;  pub use flag::AtomicFlag;

use crate::prelude::*;

/// A static flag alias
pub type Flag = Lazy<AtomicFlag>;

/// Initializes a static flag
#[macro_export]
macro_rules! flag {
    ($v:expr) => {{
        ::atomic_state::Lazy::new(|| ::atomic_state::AtomicFlag::new($v))
    }}
}
