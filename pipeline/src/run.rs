// Disable warnings
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(unused_imports)]
// #![allow(unused_must_use)]
// #[allow(dead_code)]

// Types
use super::{Command, Event, Mode, Parallel, Pipeline, Step, StepOrParallel, Trigger};
// Traits
use super::Getters;

use exec::{Process, Statuable, Status};
use std::clone::Clone;
use std::env;
use std::thread;
use utils::git::Git;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;
// use std::error::Error;

// Global var
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
// Parallelism
use rayon::prelude::*;
// Duraion
use std::time::{Duration, Instant};
// Globbing
use glob::Pattern;

// Global var
static mut PIPELINE: Lazy<Pipeline> = Lazy::new(Pipeline::new);

impl Trigger {
    pub fn is_match(&self, list: Vec<Trigger>) -> Result<bool> {
        for trigger in list {
            // println!("trigger={:#?}", &trigger);
            // println!("env={:#?}", &env);
            match &trigger {
                // Transform tag/branch into Globbing pattern
                Trigger::TriggerBranch(res) => {
                    let glob: Pattern;
                    if trigger.branch().is_some() {
                        // glob = Pattern::new(&trigger.branch().unwrap()).into_diagnostic()?;
                    }
                    // if trigger.action() == self.action() && glob.matches(&self.branch().unwrap()) {
                    //     return Ok(true);
                    // }
                }
                Trigger::TriggerTag(res) => {
                    let glob = Pattern::new(&trigger.tag().unwrap()).into_diagnostic()?;
                    if trigger.tag().is_some()
                        && self.tag().is_some()
                        && trigger.action() == self.action()
                        && glob.matches(&self.tag().unwrap())
                    {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}
impl Pipeline {
    /// Verify if pipeline can be triggered
    pub fn is_triggerable(&self) -> Result<bool> {
        let env = Trigger::env()?;

        // If in git repo
        if Git::new().exists() {
            if self.triggers.is_some() {
                env.is_match(self.triggers.clone().unwrap())
            } else {
                Ok(true)
            }
        } else {
            Ok(true)
        }
    }
    /// Execute the pipeline
    pub fn run(&mut self) {
        // Globals
        let mut ptr: *mut Pipeline;
        unsafe {
            ptr = &mut *PIPELINE;
            *ptr = self.to_owned();
        }
        // Guards
        unsafe {
            if (*ptr).is_running() {
                return;
            }
            if (*ptr).triggers.is_some() {}
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
            (*ptr).set_status(Some(Status::Started));
            (*ptr).log();
        }

        unsafe {
            (*ptr).set_status(Some(Status::Running));
            (*ptr).duration = Some(duration);
            (*ptr).log();

            for step in &mut (*ptr).steps {
                step.run(ptr);

                // Duration
                duration = start.elapsed();
                (*ptr).duration = Some(duration);

                if (step.get_status() != Some(Status::Succeeded))
                    && (step.mode().is_none() || step.mode() == Some(Mode::StopOnFailure))
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
                if last_step.mode() == Some(Mode::JumpNextOnFailure) {
                    if last_step.get_status() == Some(Status::Failed) {
                        (*ptr).set_status(Some(Status::Succeeded))
                    } else {
                        (*ptr).set_status(last_step.get_status())
                    }
                } else {
                    (*ptr).set_status(last_step.get_status())
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
        } else if steps_res.contains(&Status::Aborted) {
            self.set_status(Some(Status::Aborted));
        } else {
            self.set_status(Some(Status::Succeeded));
        }
        // Duration
        let duration = start.elapsed();
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
        let ptr = ptr.0;
        self.run(ptr);
    }
    fn run(&mut self, ptr: *mut Pipeline) {
        // Duration
        let start = Instant::now();

        self.set_status(Some(Status::Running));

        // Run commands
        for command in &mut self.commands {
            command.run(ptr);

            if (command.get_status().is_none() || command.get_status() != Some(Status::Succeeded))
                && (self.mode.is_none() || self.mode != Some(Mode::ContinueOnFailure))
            {
                break;
            }
        }

        // Set global status after run
        let final_status = &self.commands.last().unwrap().get_status();
        if final_status.is_some() {
            self.status = final_status.clone();
        } else {
            self.set_status(Some(Status::Failed))
        }

        // Duration
        let duration = start.elapsed();
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
            unsafe {
                (*ptr).log();
            }
        }
    }
}

impl Command {
    fn run(&mut self, ptr: *mut Pipeline) {
        // Duration
        let start = Instant::now();

        self.set_status(Some(Status::Running));
        unsafe {
            (*ptr).log();
        }

        // Run process
        let res = self.process.run();
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                self.set_status(Some(Status::Aborted));
                Err(e)
            }
        };

        // Duration
        let duration = start.elapsed();
        self.duration = Some(duration);

        unsafe {
            (*ptr).log();
        }
    }
}
