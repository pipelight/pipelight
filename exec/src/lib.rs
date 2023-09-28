// Rules
#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]

// Internal Imports
mod globals;
mod io;
mod processes;
mod traits;
mod types;

// Re-export
pub use traits::Statuable;
pub use types::{Io, Process, State, Status};
