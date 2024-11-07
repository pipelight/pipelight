// Types
use crate::types::{Command, Event, Mode, Parallel, Pipeline, Step, StepOrParallel};
use pipelight_exec::dates::Duration;
// Error Handling
use miette::Result;
// Traits
use pipelight_exec::{Statuable, Status};
// Global var
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Parallelism
use rayon::prelude::*;
// Tests
mod test;

// Global var
// Globals
static mut PIPELINE: Lazy<Pipeline> = Lazy::new(Pipeline::new);

impl Pipeline {
    /**
     * Run the pipeline
     * - append the pipeline state as json to the corresponding file in the log directory.
     */
    pub fn run(&mut self) -> Result<()> {
        // Use a clone of self that is wrapped in safe pointers and sharable between threads.
        let p = Arc::new(Mutex::new(self.to_owned()));
        let mut ptr = p.lock().unwrap().clone();

        // Guards
        if ptr.has_homologous_already_running().is_ok() {
            return Ok(());
        }
        if ptr.triggers.is_some() {}

        // Duration
        let mut d = Duration::default();
        d.start()?;
        ptr.duration = Some(d.clone());

        // Event
        let event = Event::new();

        // Set event = Pid , Status and Duration
        ptr.event = Some(event);
        ptr.set_status(Some(Status::Started));
        ptr.log()?;

        ptr.set_status(Some(Status::Running));
        ptr.log()?;

        for step in &mut ptr.steps {
            step.run(p.clone())?;
            if (step.get_status() != Some(Status::Succeeded))
                && (step.get_mode().is_none() || step.get_mode() == Some(Mode::StopOnFailure))
            {
                break;
            }
        }

        // Duration
        d.stop()?;
        ptr.duration = Some(d.clone());

        // Set pipeline status to last Step status
        let last_step = ptr.steps.last().unwrap();
        if last_step.get_status().is_some() {
            if last_step.get_mode() == Some(Mode::JumpNextOnFailure) {
                if last_step.get_status() == Some(Status::Failed) {
                    ptr.set_status(Some(Status::Succeeded))
                } else {
                    ptr.set_status(last_step.get_status())
                }
            } else {
                ptr.set_status(last_step.get_status())
            }
        } else {
            ptr.set_status(Some(Status::Failed))
        }
        ptr.log()?;

        // Execute fallbacks
        if ptr.fallback.is_some() {
            let fallback = &mut self.fallback.clone().unwrap();
            if ptr.status == Some(Status::Failed) && fallback.on_failure.is_some() {
                // let steps = (*ptr).on_failure.as_mut().unwrap();
                for step in fallback.on_failure.as_mut().unwrap() {
                    step.run(p.clone())?;
                }
            }
            if ptr.status == Some(Status::Succeeded) && fallback.on_failure.is_some() {
                // let steps = (*ptr).on_failure.as_mut().unwrap();
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(p.clone())?;
                }
            }
            if ptr.status == Some(Status::Aborted) && fallback.on_success.is_some() {
                // let steps = (*ptr).on_failure.as_mut().unwrap();
                for step in fallback.on_abortion.as_mut().unwrap() {
                    step.run(p.clone())?;
                }
            }
            // Duration
            d.stop()?;
            ptr.duration = Some(d);
            ptr.log()?;
        }
        *self = ptr.clone();
        Ok(())
    }
}

impl StepOrParallel {
    fn run(&mut self, p: Arc<Mutex<Pipeline>>) -> Result<()> {
        match self {
            StepOrParallel::Step(res) => res.run(p.clone()),
            StepOrParallel::Parallel(res) => res.run(p.clone()),
        }
    }
}

impl Parallel {
    fn run(&mut self, p: Arc<Mutex<Pipeline>>) -> Result<()> {
        let mut ptr = p.lock().unwrap().clone();
        // Duration
        let mut d = Duration::default();
        d.start()?;
        self.duration = Some(d.clone());

        self.set_status(Some(Status::Running));

        // Pass wrapped pointer to threads
        self.steps
            .par_iter_mut()
            .for_each(|e| e.run(p.clone()).unwrap());

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

        ptr.log()?;
        Ok(())
    }
}

impl Step {
    fn run(&mut self, p: Arc<Mutex<Pipeline>>) -> Result<()> {
        let mut ptr = p.lock().unwrap().clone();
        // Options
        let mode = self.get_mode();
        // Duration
        let mut d = Duration::default();
        d.start()?;
        self.duration = Some(d.clone());

        self.set_status(Some(Status::Running));

        // Run commands
        for command in &mut self.commands {
            command.run(p.clone())?;

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

        ptr.log()?;
        // Execute post-run steps
        if self.fallback.is_some() {
            let fallback = &mut self.fallback.as_mut().unwrap();
            if self.status == Some(Status::Failed) && fallback.on_failure.is_some() {
                for step in fallback.on_failure.as_mut().unwrap() {
                    step.run(p.clone())?;
                }
            }
            if self.status == Some(Status::Succeeded) && fallback.on_success.is_some() {
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(p.clone())?;
                }
            }
            if self.status == Some(Status::Aborted) && fallback.on_abortion.is_some() {
                for step in fallback.on_success.as_mut().unwrap() {
                    step.run(p.clone())?;
                }
            }
            ptr.log()?;
        }
        Ok(())
    }
}

impl Command {
    fn run(&mut self, p: Arc<Mutex<Pipeline>>) -> Result<()> {
        let mut ptr = p.lock().unwrap().clone();

        // Duration
        let mut d = Duration::default();
        d.start()?;
        self.duration = Some(d.clone());

        self.set_status(Some(Status::Running));
        ptr.log()?;

        // Run process
        self.process.term().run()?;
        // let res = self.process.term().run();
        // let _ = match res {
        //     Ok(val) => Ok(val),
        //     Err(e) => {
        //         self.set_status(Some(Status::Aborted));
        //         Err(e)
        //     }
        // };

        // Duration
        d.stop()?;
        self.duration = Some(d);

        ptr.log()?;
        Ok(())
    }
}
