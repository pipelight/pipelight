#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]

mod methods;
mod run;
mod traits;
mod types;
// tests
mod test;

// Re-export
pub use crate::traits::Getters;
pub use crate::types::{
    Command, Config, Event, Fallback, Logs, Mode, Node, Parallel, Pipeline, Step, StepOrParallel,
    Trigger,
};
pub use traits::display;
