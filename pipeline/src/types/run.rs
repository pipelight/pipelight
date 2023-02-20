use super::{Command, Event, Parallel, Pipeline, Step, StepOrParallel};
use exec::types::{Statuable, Status, StrOutput};
use exec::Exec;
use std::clone::Clone;
use std::error::Error;
use std::thread;

// Global var
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
// Parallelism
use rayon::prelude::*;
// Duraion
use std::time::{Duration, Instant};

// Global var
static mut PIPELINE: Lazy<Pipeline> = Lazy::new(|| Pipeline::new());

impl Pipeline {
    /// Execute the pipeline
    pub fn run(&mut self) {
        // Globals
        let mut ptr: *mut Pipeline;
        unsafe {
            ptr = &mut *PIPELINE;
            *ptr = self.to_owned();
        }

        // Duration
        let start = Instant::now();
        let duration = start.elapsed();
        unsafe {
            (*ptr).duration = Some(duration);
        }

        // Guards
        unsafe {
            if (*ptr).is_running() {
                return;
            }
        }

        // Set Pid and Status and Duration
        unsafe {
            (*ptr).event = Some(Event::new());
            (*ptr).set_status(Some(Status::Running));
            (*ptr).duration = Some(duration);
            (*ptr).log();
        }

        unsafe {
            for step in &mut (*ptr).steps {
                step.run(ptr);
                if step.get_status() != Some(Status::Succeeded)
                    && (step.non_blocking().is_none() || step.non_blocking() == Some(false))
                {
                    break;
                }
            }
        }
        // Set pipeline status to last Step status
        unsafe {
            let last_step = (*ptr).steps.last().unwrap();
            if last_step.get_status().is_some() {
                (*ptr).set_status(Some(last_step.get_status().clone().unwrap()))
            } else {
                (*ptr).set_status(Some(Status::Failed))
            }
            (*ptr).log();
        }

        // Execute post-run steps
        unsafe {
            if (*ptr).status == Some(Status::Failed) && (*ptr).on_failure.is_some() {
                // let steps = (*ptr).on_failure.as_mut().unwrap();
                for step in (*ptr).on_failure.as_mut().unwrap() {
                    step.run(ptr);
                }
            }
            if (*ptr).status == Some(Status::Succeeded) && (*ptr).on_success.is_some() {
                // let steps = (*ptr).on_failure.as_mut().unwrap();
                for step in (*ptr).on_success.as_mut().unwrap() {
                    step.run(ptr);
                }
            }
            if (*ptr).status == Some(Status::Aborted) && (*ptr).on_success.is_some() {
                // let steps = (*ptr).on_failure.as_mut().unwrap();
                for step in (*ptr).on_abortion.as_mut().unwrap() {
                    step.run(ptr);
                }
            }
        }
    }
}

impl StepOrParallel {
    fn run(&mut self, ptr: *mut Pipeline) {
        match self {
            StepOrParallel::Step(res) => res.run(ptr),
            StepOrParallel::Parallel(res) => res.run(ptr),
        }
    }
}

impl Parallel {
    fn run(&mut self, ptr: *mut Pipeline) {
        self.set_status(Some(Status::Running));

        // Pass wrapped pointer to threads
        let ptr_wrapper = PtrWrapper(ptr);
        self.steps
            .par_iter_mut()
            .for_each(|e| e.unsafe_run(ptr_wrapper));

        // Set parallel global status
        let steps_res: Vec<Status> = self
            .steps
            .iter()
            .map(|e| e.clone().status.unwrap())
            .collect();

        if steps_res.contains(&Status::Failed) {
            self.set_status(Some(Status::Failed));
        } else {
            if steps_res.contains(&Status::Aborted) {
                self.set_status(Some(Status::Aborted));
            } else {
                self.set_status(Some(Status::Succeeded));
            }
        }
        unsafe {
            (*ptr).log();
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PtrWrapper(*mut Pipeline);
unsafe impl Sync for PtrWrapper {}
unsafe impl Send for PtrWrapper {}
impl Step {
    fn unsafe_run(&mut self, ptr: PtrWrapper) {
        let ptr = ptr.0.clone();
        self.run(ptr);
    }
    fn run(&mut self, ptr: *mut Pipeline) {
        self.set_status(Some(Status::Running));

        // Run commands
        for command in &mut self.commands {
            command.run(ptr);
            if command.status.is_none() {
                break;
            } else if command.status == Some(Status::Failed)
                || command.status == Some(Status::Aborted)
            {
                break;
            }
        }

        // Set global status after run
        let final_status = &self.commands.last().unwrap().status;
        if final_status.is_some() {
            self.status = final_status.clone();
        } else {
            self.set_status(Some(Status::Failed))
        }
        unsafe {
            (*ptr).log();
        }
        // Execute post-run steps
        if self.status == Some(Status::Failed) && self.on_failure.is_some() {
            for step in self.on_failure.as_mut().unwrap() {
                step.run(ptr);
            }
        }
        if self.status == Some(Status::Succeeded) && self.on_success.is_some() {
            for step in self.on_success.as_mut().unwrap() {
                step.run(ptr);
            }
        }
        if self.status == Some(Status::Aborted) && self.on_abortion.is_some() {
            for step in self.on_success.as_mut().unwrap() {
                step.run(ptr);
            }
        }
    }
}

impl Command {
    fn run(&mut self, ptr: *mut Pipeline) {
        // Duration
        let start = Instant::now();
        // pipeline.duration.unwrap() + duration;

        self.status = Some(Status::Running);
        unsafe {
            (*ptr).log();
        }

        let output_res = Exec::new().simple(&self.stdin);
        match output_res {
            Ok(output) => {
                self.output = Some(output.clone());
                self.status = output.clone().status;
                Ok(())
            }
            Err(e) => {
                let mut output = StrOutput::new();
                output.status = Some(Status::Aborted);
                self.output = Some(output);
                Err(e)
            }
        };

        self.duration = Some(start.elapsed());
        unsafe {
            (*ptr).log();
        }
    }
}
