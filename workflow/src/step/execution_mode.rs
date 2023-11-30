// Structs
use crate::types::{Mode, Step, StepOrParallel};

impl StepOrParallel {
    pub fn get_mode(&self) -> Option<Mode> {
        match self {
            StepOrParallel::Step(res) => res.get_mode(),
            StepOrParallel::Parallel(_) => None,
        }
    }
}
impl Step {
    pub fn get_mode(&self) -> Option<Mode> {
        if let Some(options) = &self.options {
            options.mode.clone()
        } else {
            None
        }
    }
}
