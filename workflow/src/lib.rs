// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_must_use)]

pub mod globals;
// Error struct
pub mod error;
// Methods
mod config;
mod logs;
mod pipeline;
mod step;
mod trigger;

pub mod traits;
pub mod types;

// Re-export
pub use exec::Statuable;
pub use traits::Getters;
pub use types::*;
