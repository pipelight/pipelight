// Types
use crate::types::{Command, Event, Mode, Parallel, Pipeline, Step, StepOrParallel};
use pipelight_exec::dates::Duration;
// Error Handling
use miette::Result;
// Traits
use pipelight_exec::{Statuable, Status};
// Global var
use once_cell::sync::Lazy;
// Parallelism
use rayon::prelude::*;
// Tests
mod test;

// Global var
static mut PIPELINE: Lazy<Pipeline> = Lazy::new(Pipeline::default);

impl Pipeline {
    /// Execute the pipeline
    pub fn run(&mut self) -> Result<()> {
        // Globals
        let ptr: *mut Pipeline;
        unsafe {
            ptr = &mut *PIPELINE;
            *ptr = self.to_owned();
        }
        // Guards
        unsafe {
            if (*ptr).has_homologous_already_running().is_ok() {
                return Ok(());
            }
            if (*ptr).triggers.is_some() {}
        }

        // Duration
        let mut d = Duration::default();
        d.start()?;
        unsafe {
            (*ptr).duration = Some(d.clone());
        }

        // Event
        let event = Event::default();

        // Set event = Pid , Status and Duration
        unsafe {
            (*ptr).event = Some(event);
            (*ptr).set_status(Some(Status::Started));
            (*ptr).log()?;
        }

        unsafe {
            (*ptr).set_status(Some(Status::Running));
            (*ptr).log()?;

            for step in &mut (*ptr).steps {
                step.run(ptr)?;
                if (step.get_status() != Some(Status::Succeeded))
                    && (step.get_mode().is_none() || step.get_mode() == Some(Mode::StopOnFailure))
                {
                    break;
                }
            }
        }

        // Duration
        d.stop()?;
        unsafe {
            (*ptr).duration = Some(d.clone());
        }

        // Set pipeline status to last Step status
        unsafe {
            let last_step = (*ptr).steps.last().unwrap();
            if last_step.get_status().is_some() {
                if last_step.get_mode() == Some(Mode::JumpNextOnFailure) {
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
            (*ptr).log()?;
        }

        // Execute fallbacks
        unsafe {
            if (*ptr).fallback.is_some() {
                let fallback = &mut (*ptr).fallback.as_mut().unwrap();
                if (*ptr).status == Some(Status::Failed) && fallback.on_failure.is_some() {
                    // let steps = (*ptr).on_failure.as_mut().unwrap();
                    for step in fallback.on_failure.as_mut().unwrap() {
                        step.run(ptr)?;
                    }
                }
                if (*ptr).status == Some(Status::Succeeded) && fallback.on_failure.is_some() {
                    // let steps = (*ptr).on_failure.as_mut().unwrap();
                    for step in fallback.on_success.as_mut().unwrap() {
                        step.run(ptr)?;
                    }
                }
                if (*ptr).status == Some(Status::Aborted) && fallback.on_success.is_some() {
                    // let steps = (*ptr).on_failure.as_mut().unwrap();
                    for step in fallback.on_abortion.as_mut().unwrap() {
                        step.run(ptr)?;
                    }
                }
                // Duration
                d.stop()?;
                (*ptr).duration = Some(d);
                (*ptr).log()?;
            }
        }
        unsafe {
            let global_pipe = &mut (*ptr);
            *self = global_pipe.to_owned();
        }
        Ok(())
    }
}

impl StepOrParallel {
    fn run(&mut self, ptr: *mut Pipeline) -> Result<()> {
        match self {
            StepOrParallel::Step(res) => res.run(ptr),
            StepOrParallel::Parallel(res) => res.run(ptr),
        }
    }
}

impl Parallel {
    fn run(&mut self, ptr: *mut Pipeline) -> Result<()> {
        // Duration
        let mut d = Duration::default();
        d.start()?;
        self.duration = Some(d.clone());

        self.set_status(Some(Status::Running));

        // Pass wrapped pointer to threads
        let ptr_wrapper = PtrWrapper(ptr);
        self.steps
            .par_iter_mut()
            .for_each(|e| e.unsafe_run(ptr_wrapper).unwrap());

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
        d.stop()?;
        self.duration = Some(d);

        unsafe {
            (*ptr).log()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct PtrWrapper(*mut Pipeline);
unsafe impl Sync for PtrWrapper {}
unsafe impl Send for PtrWrapper {}
impl Step {
    fn unsafe_run(&mut self, ptr: PtrWrapper) -> Result<()> {
        let ptr = ptr.0;
        self.run(ptr)
    }
    fn run(&mut self, ptr: *mut Pipeline) -> Result<()> {
        // Options
        let mode = self.get_mode();
        // Duration
        let mut d = Duration::default();
        d.start()?;
        self.duration = Some(d.clone());

        self.set_status(Some(Status::Running));

        // Run commands
        for command in &mut self.commands {
            command.run(ptr)?;

            if (command.get_status().is_none() || command.get_status() != Some(Status::Succeeded))
                && (mode.is_none() || mode != Some(Mode::ContinueOnFailure))
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
        d.stop()?;
        self.duration = Some(d);

        unsafe {
            (*ptr).log()?;
        }
        // Execute post-run steps
        if self.fallback.is_some() {
            let fallback = &mut self.fallback.as_mut().unwrap();
            if self.status == Some(Status::Failed) && fallback.on_failure.is_some() {
                for step in fallback.on_failure.as_mut().unwrap() {
                    step.run(ptr)?;
                }
            }
            if self.status == Some(Status::Succeeded) && fallback.on_success.is_some() {
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(ptr)?;
                }
            }
            if self.status == Some(Status::Aborted) && fallback.on_abortion.is_some() {
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(ptr)?;
                }
            }
            unsafe {
                (*ptr).log()?;
            }
        }
        Ok(())
    }
}

impl Command {
    fn run(&mut self, ptr: *mut Pipeline) -> Result<()> {
        // Duration
        let mut d = Duration::default();
        d.start()?;
        self.duration = Some(d.clone());

        self.set_status(Some(Status::Running));
        unsafe {
            (*ptr).log()?;
        }

        // Run process
        let res = self.process.fs().run();
        let _ = match res {
            Ok(_) => Ok(()),
            Err(e) => {
                self.set_status(Some(Status::Aborted));
                Err(e)
            }
        };

        // Duration
        d.stop()?;
        self.duration = Some(d);

        unsafe {
            (*ptr).log()?;
        }
        Ok(())
    }
}
