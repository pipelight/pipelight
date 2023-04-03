// Disable warnings
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
//
use super::{Command, Event, Parallel, Pipeline, Step, StepOrParallel};
use exec::types::{Statuable, Status, StrOutput};
use exec::Exec;
use std::clone::Clone;
use std::env;
use std::thread;
use std::thread::sleep;
use utils::git::Git;

use std::io::{BufRead, BufReader};
// Global var
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
// Parallelism
use rayon::prelude::*;
// Duraion
use std::time::{Duration, Instant};
// Error Handling
use miette::{IntoDiagnostic, Result};
// use std::error::Error;
use std::io;

// Global var
static mut PIPELINE: Lazy<Pipeline> = Lazy::new(|| Pipeline::new());

impl Pipeline {
    /// Execute the pipeline
    pub fn run(&mut self) {
        // Globals
        let mut ptr: *mut Pipeline;
        unsafe {
            ptr = &mut *PIPELINE;
            *ptr = self.clone().to_owned();
        }
        // Guards
        unsafe {
            if (*ptr).is_running() {
                return;
            }
        }

        // Duration
        let start = Instant::now();
        let mut duration = start.elapsed();

        //Event
        let origin = env::current_dir().unwrap();
        let event = Event::new();
        Git::new().teleport();

        // Set Pid and Status and Duration
        unsafe {
            (*ptr).event = Some(event);
            (*ptr).set_status(Some(Status::Running));
            (*ptr).duration = Some(duration);
            (*ptr).log();
        }

        unsafe {
            for step in &mut (*ptr).steps {
                step.run(ptr);

                // Duration
                duration = start.elapsed();
                (*ptr).duration = Some(duration);

                if step.get_status() != Some(Status::Succeeded)
                    && (step.non_blocking().is_none() || step.non_blocking() == Some(false))
                {
                    break;
                }
            }
        }

        //Event
        env::set_current_dir(origin).unwrap();

        // Duration
        duration = start.elapsed();
        unsafe {
            (*ptr).duration = Some(duration);
        }

        // Set pipeline status to last Step status
        unsafe {
            let last_step = (*ptr).steps.last().unwrap();
            if last_step.get_status().is_some() {
                if last_step.non_blocking() == Some(true) {
                    if last_step.get_status() == Some(Status::Failed) {
                        (*ptr).set_status(Some(Status::Succeeded))
                    } else {
                        (*ptr).set_status(Some(last_step.get_status().clone().unwrap()))
                    }
                } else {
                    (*ptr).set_status(Some(last_step.get_status().clone().unwrap()))
                }
            } else {
                (*ptr).set_status(Some(Status::Failed))
            }
            (*ptr).log();
        }

        // Execute fallbacks
        unsafe {
            if (*ptr).fallback.is_some() {
                let fallback = &mut (*ptr).fallback.as_mut().unwrap();
                if (*ptr).status == Some(Status::Failed) && fallback.on_failure.is_some() {
                    // let steps = (*ptr).on_failure.as_mut().unwrap();
                    for step in fallback.on_failure.as_mut().unwrap() {
                        step.run(ptr);
                    }
                }
                if (*ptr).status == Some(Status::Succeeded) && fallback.on_failure.is_some() {
                    // let steps = (*ptr).on_failure.as_mut().unwrap();
                    for step in fallback.on_success.as_mut().unwrap() {
                        step.run(ptr);
                    }
                }
                if (*ptr).status == Some(Status::Aborted) && fallback.on_success.is_some() {
                    // let steps = (*ptr).on_failure.as_mut().unwrap();
                    for step in fallback.on_abortion.as_mut().unwrap() {
                        step.run(ptr);
                    }
                }
                // Duration
                duration = start.elapsed();
                (*ptr).duration = Some(duration);
                (*ptr).log();
            }
        }
        unsafe {
            let global_pipe = &mut (*ptr);
            *self = global_pipe.to_owned();
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
        // Duration
        let start = Instant::now();
        let duration;

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
        // Duration
        duration = start.elapsed();
        self.duration = Some(duration);

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
        // Duration
        let start = Instant::now();
        let duration;

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

        // Duration
        duration = start.elapsed();
        self.duration = Some(duration);

        unsafe {
            (*ptr).log();
        }
        // Execute post-run steps
        if self.fallback.is_some() {
            let fallback = &mut self.fallback.as_mut().unwrap();
            if self.status == Some(Status::Failed) && fallback.on_failure.is_some() {
                for step in fallback.on_failure.as_mut().unwrap() {
                    step.run(ptr);
                }
            }
            if self.status == Some(Status::Succeeded) && fallback.on_success.is_some() {
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(ptr);
                }
            }
            if self.status == Some(Status::Aborted) && fallback.on_abortion.is_some() {
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(ptr);
                }
            }
        }
    }
}

impl Command {
    fn run(&mut self, ptr: *mut Pipeline) {
        // Duration
        let start = Instant::now();
        let duration;

        self.status = Some(Status::Running);
        self.output = Some(StrOutput {
            stdout: Some(String::new()),
            stderr: Some(String::new()),
            status: Some(Status::Running),
        });
        unsafe {
            (*ptr).log();
        }

        let mut child = Exec::new().simple_early(&self.stdin).unwrap();

        let mut wait = child.try_wait();
        let mut stdout_reader = BufReader::new(child.stdout.unwrap());
        let mut stderr_reader = BufReader::new(child.stderr.unwrap());
        let mut out_line = String::new();
        let mut err_line = String::new();
        loop {
            // println!("exit: {:?}", &wait);
            match wait {
                Ok(Some(status)) => match status.success() {
                    true => {
                        self.output.as_mut().unwrap().status = Some(Status::Succeeded);
                        self.status = self.output.clone().unwrap().status;
                        break;
                    }
                    false => {
                        self.output.as_mut().unwrap().status = Some(Status::Failed);
                        self.status = self.output.clone().unwrap().status;
                        break;
                    }
                },
                Ok(None) => {
                    let stdout_lines = stdout_reader.read_line(&mut out_line);

                    self.output.as_mut().unwrap().stdout =
                        Some(self.output.clone().unwrap().stdout.unwrap() + &out_line);
                    println!("{:?}", &self.output.as_mut().unwrap().stdout);
                    println!("{:?}", &out_line);

                    let stderr_lines = stderr_reader.read_line(&mut err_line);

                    self.output.as_mut().unwrap().stderr =
                        Some(self.output.clone().unwrap().stderr.unwrap() + &err_line);
                    println!("{:?}", &self.output.as_mut().unwrap().stderr);
                    println!("{:?}", &err_line);
                }
                Err(_) => {
                    println!("couldn't read process status");
                }
            }
            // wait = child.try_wait();
        }

        // Duration
        duration = start.elapsed();
        self.duration = Some(duration);

        unsafe {
            (*ptr).log();
        }
    }
}
