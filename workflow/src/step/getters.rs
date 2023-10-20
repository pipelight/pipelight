// Structs
use crate::types::{Fallback, Mode, Parallel, Step, StepOrParallel};
use exec::Process;
// Error Handling
use miette::Result;

impl Step {
    pub fn get_procs(&self) -> Result<Vec<Process>> {
        let mut procs: Vec<Process> = vec![];
        for command in self.commands.clone() {
            procs.push(command.process);
        }
        Ok(procs)
    }
}
impl Parallel {
    pub fn get_procs(&self) -> Result<Vec<Process>> {
        let mut procs: Vec<Process> = vec![];
        for step in self.steps.clone() {
            procs.extend(step.get_procs()?);
        }
        Ok(procs)
    }
}
impl StepOrParallel {
    pub fn get_procs(&self) -> Result<Vec<Process>> {
        let mut procs: Vec<Process> = vec![];
        match self {
            StepOrParallel::Step(step) => {
                procs.extend(step.get_procs()?);
            }
            StepOrParallel::Parallel(parallel) => {
                procs.extend(parallel.get_procs()?);
            }
        }
        Ok(procs)
    }
}
impl Fallback {
    pub fn get_procs(&self) -> Result<Vec<Process>> {
        let mut procs: Vec<Process> = vec![];
        let fallbacks: Vec<Option<Vec<StepOrParallel>>> = vec![
            self.on_started.clone(),
            self.on_failure.clone(),
            self.on_success.clone(),
            self.on_abortion.clone(),
        ];
        for vec_step_or_parallel in fallbacks {
            if let Some(vec_step_or_parallel) = vec_step_or_parallel {
                for step_or_parallel in vec_step_or_parallel {
                    procs.extend(step_or_parallel.get_procs()?);
                }
            }
        }
        Ok(procs)
    }
}
