pub mod state;   pub use state::AtomState;
pub mod guard;   pub use guard::AtomStateGuard;
pub mod reff;    pub use reff::AtomStateRef;

pub(super) const ERR_MSG: &str = "The data has been poisoned!";
