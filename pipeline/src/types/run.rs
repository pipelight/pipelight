use super::{Command, Event, Parallel, Pipeline, Step, StepOrParallel};
use exec::types::{Status, StrOutput};
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
            (*ptr).status(&Status::Running);
            (*ptr).duration = Some(duration);
            (*ptr).log();
        }

        unsafe {
            for step in &mut (*ptr).steps {
                step.run(ptr);
            }
        }
        // Set pipeline status to last Step status
        unsafe {
            let last_step = (*ptr).steps.last().unwrap();
            if last_step.get_status().is_some() {
                (*ptr).status(&last_step.get_status().clone().unwrap())
            } else {
                (*ptr).status(&Status::Failed)
            }
        }
        unsafe {
            (*ptr).log();
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
        self.status(&Status::Running);
        for command in &mut self.commands {
            command.run(ptr);
        }
        let optional_output = &self.commands.last().unwrap().output;
        if optional_output.is_some() {
            self.status(&optional_output.clone().unwrap().status)
        } else {
            self.status(&Status::Failed)
        }
        unsafe {
            (*ptr).log();
        }
    }
}

impl Parallel {
    fn run(&mut self, ptr: *mut Pipeline) {
        self.status(&Status::Running);

        // Pass wrapped pointer to thread
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
            self.status(&Status::Failed);
        } else {
            if steps_res.contains(&Status::Aborted) {
                self.status(&Status::Aborted);
            } else {
                self.status(&Status::Succeeded);
            }
        }
        unsafe {
            (*ptr).log();
        }
    }
}

impl Command {
    fn run(&mut self, ptr: *mut Pipeline) {
        // Duration
        let start = Instant::now();
        let mut duration = start.elapsed();
        // pipeline.duration.unwrap() + duration;

        let output_res = Exec::new().simple(&self.stdin);
        match output_res {
            Ok(output) => {
                self.output = Some(output);
                Ok(())
            }
            Err(e) => {
                let mut output = StrOutput::new();
                output.status = Status::Failed;
                self.output = Some(output);
                Err(e)
            }
        };
        duration = start.elapsed();

        // pipeline.duration.unwrap() + duration;
        unsafe {
            (*ptr).log();
        }
    }
}
