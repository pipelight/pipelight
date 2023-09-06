mod methods;
pub mod run;
mod traits;
pub mod types;

// Re-export
pub use types::traits::Getters;
pub use types::{
    Command, Config, Duration, Event, Fallback, Logs, Mode, Node, Parallel, Pipeline, Step,
    StepOrParallel, Trigger,
};
