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
// static mut pipeline: Box<Pipeline> = unsafe { Box::new(PIPELINE) };

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
    }
}

impl StepOrParallel {
    fn run(&mut self, ptr: *mut Pipeline) {
        match self {
            StepOrParallel::Step(res) => res.run(ptr),
            StepOrParallel::Parallel(res) => res.run(),
        }
    }
}
impl Parallel {
    fn run(&self) {
        // let ptr = Arc::new(pipeline_ptr);
        // let ptr = Arc::clone(&ptr);
        // self.parallel.par_iter_mut().for_each(|step| {
        //     step.run(ptr);
        // })

        // Set parallel status
        // let last_step = pipeline_ptr.as_mut().unwrap().steps.last().unwrap();
        //
        // if last_step.status.is_some() {
        //     pipeline.status(&last_step.status.clone().unwrap())
        // } else {
        //     pipeline.status(&Status::Failed)
        // }
        // pipeline.log();
    }
}
impl Step {
    fn run(&mut self, ptr: *mut Pipeline) {
        for command in &mut self.commands {
            command.run(ptr);
        }
        let optional_output = &self.commands.last().unwrap().output;
        if optional_output.is_some() {
            self.status(&optional_output.clone().unwrap().status)
        } else {
            self.status(&Status::Failed)
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
                unsafe {
                    (*ptr).status(&Status::Failed);
                }
                Err(e)
            }
        };
        // *p.steps[step_index].command[cmd_index] = self;
        duration = start.elapsed();

        // pipeline.duration.unwrap() + duration;
        unsafe {
            (*ptr).log();
        }
    }
}
