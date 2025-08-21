pub mod guard;   pub use guard::AtomicStateGuard;
pub mod state;   pub use state::AtomicState;

use crate::prelude::*;

pub(super) const ERR_MSG: &str = "The State data has been poisoned!";

/// A static data alias
pub type State<T> = Lazy<AtomicState<T>>;

/// Initializes a static data
#[macro_export]
macro_rules! state {
    ($v:expr) => {{
        ::atomic_state::Lazy::new(|| ::atomic_state::AtomicState::new($v))
    }}
}
