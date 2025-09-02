#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod prelude;

pub mod flag;    pub use flag::AtomFlag;
pub mod state;   pub use state::{ AtomState, AtomStateGuard };

pub use once_cell::{ self, sync::Lazy };
pub use arc_swap::{ self, ArcSwap };

/// Initializes a static variable by 'once_cell::Lazy'
#[macro_export]
macro_rules! lazy {
    ($e:expr) => {{
        ::atomic_state::Lazy::new(|| $e)
    }}
}

/// Initializes a static 'AtomFlag' by 'once_cell::Lazy'
#[macro_export]
macro_rules! lazy_flag {
    ($e:expr) => {{
        ::atomic_state::Lazy::new(|| AtomFlag::new($e))
    }}
}

/// Initializes a static 'AtomState' by 'once_cell::Lazy'
#[macro_export]
macro_rules! lazy_state {
    ($e:expr) => {{
        ::atomic_state::Lazy::new(|| AtomState::new($e))
    }}
}
