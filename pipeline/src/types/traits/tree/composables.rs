use crate::types::traits::tree::characters::Characters;
use crate::types::traits::tree::Tree;
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use exec::types::{Statuable, Status};

impl Statuable for Command {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for Step {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for StepOrParallel {
    fn set_status(&mut self, status: Option<Status>) {
        match self {
            StepOrParallel::Step(res) => res.status = status,
            StepOrParallel::Parallel(res) => res.status = status,
        }
    }
    fn get_status(&self) -> Option<Status> {
        match self {
            StepOrParallel::Step(res) => res.status.clone(),
            StepOrParallel::Parallel(res) => res.status.clone(),
        }
    }
}
impl Statuable for Parallel {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for Pipeline {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
