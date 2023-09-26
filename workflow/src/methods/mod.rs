// Internal imports

use crate::types::{Config, Duration, Mode, StepOrParallel};

// Error Handling
use miette::{Error, Result};

//Duration
use chrono::{DateTime, Local};

// Standard libs
use log::warn;

//sys

// Globbing

// External imports

// Tests
mod logs;
mod pipeline;
mod trigger;
pub use pipeline::filters::Filters;

impl Config {
    /// Remove pipelines with the same name
    pub fn dedup_pipelines(&mut self) -> Self {
        if self.pipelines.is_some() {
            let init_length = &self.pipelines.clone().unwrap().len();
            self.pipelines
                .as_mut()
                .unwrap()
                .sort_by_key(|p| p.clone().name);
            self.pipelines
                .as_mut()
                .unwrap()
                .dedup_by_key(|p| p.clone().name);

            let end_length = &self.pipelines.clone().unwrap().len();
            if init_length != end_length {
                let message = "Removed pipelines with identical names";
                warn!("{}", message)
            }
        }
        self.to_owned()
    }
    pub fn has_watch_flag(&self) -> Result<()> {
        for pipeline in self.pipelines.clone().unwrap() {
            if pipeline.is_watchable().is_ok() {
                return Ok(());
            }
        }
        let message = "no watchable pipelines";
        Err(Error::msg(message))
    }
}

impl StepOrParallel {
    pub fn mode(&self) -> Option<Mode> {
        match self {
            StepOrParallel::Step(res) => res.mode.clone(),
            StepOrParallel::Parallel(res) => res.mode.clone(),
        }
    }
}
