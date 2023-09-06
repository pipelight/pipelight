use crate::workflow::types::{Command, Parallel, Pipeline, Step, StepOrParallel};
use exec::{Statuable, Status};

impl Statuable for Command {
    fn get_status(&self) -> Option<Status> {
        self.process.get_status()
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.process.set_status(status);
    }
}
impl Statuable for Step {
    fn get_status(&self) -> Option<Status> {
        self.status.to_owned()
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
        self.status.to_owned()
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
impl Statuable for Pipeline {
    fn get_status(&self) -> Option<Status> {
        self.status.to_owned()
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
