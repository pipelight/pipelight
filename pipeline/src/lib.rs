#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]

mod methods;
mod run;
mod traits;
mod types;

// Re-export
pub use crate::traits::Getters;
// pub use crate::traits::Statuable;
pub use crate::types::{
    Command, Config, Event, Fallback, Mode, Node, Parallel, Pipeline, Step, StepOrParallel,
    Trigger, TriggerBranch, TriggerTag,
};
