#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod prelude;

pub mod flag;    pub use flag::*;
pub mod state;   pub use state::*;

pub use once_cell::{ self, sync::Lazy };
