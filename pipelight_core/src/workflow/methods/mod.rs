// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

// Internal imports
use super::traits::Getters;
use crate::workflow::types::{Config, Duration, Logs, Mode, Pipeline, StepOrParallel, Trigger};

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

//Duration
use chrono::{DateTime, Local};

// Standard libs
use log::{error, info, warn};

//sys
use rustix::process::{kill_process_group, test_kill_process, Pid, Signal};

// Globbing
use glob::Pattern;

// External imports
use crate::globals::LOGGER;
use exec::{Statuable, Status};
use utils::git::{Flag, Git, Special};

// Tests
mod pipeline;
mod trigger;

impl Logs {
    /// Pretty print logs from json log file
    pub fn sanitize(pipelines: &mut [Pipeline]) -> Result<()> {
        Ok(())
    }
}

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

pub fn std_duration_to_iso8601(duration: std::time::Duration) -> Result<String> {
    let chrono_duration = chrono::Duration::from_std(duration).ok();
    if let Some(chrono_duration) = chrono_duration {
        let duration_iso_8601 = format!("{}", chrono_duration);
        Ok(duration_iso_8601)
    } else {
        Err(Error::msg("Bad std::Duration instance"))
    }
}
pub fn iso8601_to_std_duration(duration: String) -> Result<std::time::Duration> {
    let duration = &duration.as_str();
    let chrono_duration: Option<iso8601_duration::Duration> = duration.parse().ok();
    if let Some(chrono_duration) = chrono_duration {
        let std_duration = chrono_duration.to_std();
        if let Some(std_duration) = std_duration {
            return Ok(std_duration);
        }
    }
    Err(Error::msg("Couldn't parse duration: Bad iso8601 duration"))
}
pub fn compute_duration(duration: Duration) -> Result<std::time::Duration> {
    let computed_duration: Option<Duration> = None;
    let now = Local::now();

    let date = duration
        .started_at
        .unwrap()
        .parse::<DateTime<Local>>()
        .unwrap();

    let diff: chrono::Duration = now - date;
    let duration = diff.to_std().unwrap();

    Ok(duration)
}
