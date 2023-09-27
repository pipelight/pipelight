// Error Handling
use miette::{Error, IntoDiagnostic, Result};

//Duration
use super::Duration;
use chrono::{DateTime, Local};
use std::time;

// Test
mod test;

use crate::dates::convert::*;

impl From<&Duration> for String {
    fn from(e: &Duration) -> Self {
        let mut e = e.clone();
        let std = e.get().unwrap();
        std_duration_to_iso8601(&std).unwrap()
    }
}
impl Duration {
    pub fn get(&mut self) -> Result<time::Duration> {
        let duration: time::Duration;
        if self.computed.is_some() {
            duration = iso8601_to_std_duration(&self.computed.clone().unwrap())?;
        } else if self.ended_at.is_some() {
            let diff = self.get_ended_at()? - self.get_started_at()?;
            duration = diff.to_std().unwrap();
            self.computed = std_duration_to_iso8601(&duration).ok();
        } else {
            let diff = Local::now() - self.get_started_at()?;
            duration = diff.to_std().unwrap();
        }
        Ok(duration)
    }
    pub fn get_started_at(&self) -> Result<DateTime<Local>> {
        let parsed = self
            .started_at
            .clone()
            .unwrap()
            .parse::<DateTime<Local>>()
            .into_diagnostic()?;
        Ok(parsed)
    }
    pub fn get_ended_at(&self) -> Result<DateTime<Local>> {
        let parsed = self
            .ended_at
            .clone()
            .unwrap()
            .parse::<DateTime<Local>>()
            .into_diagnostic()?;
        Ok(parsed)
    }
    pub fn start(&mut self) -> Result<()> {
        let now = Local::now();
        self.started_at = Some(now.to_string());
        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        let now = Local::now();
        self.ended_at = Some(now.to_string());
        let diff = self.get_ended_at()? - self.get_started_at()?;
        let diff = diff.to_std().unwrap();
        self.computed = Some(std_duration_to_iso8601(&diff)?);
        Ok(())
    }
}
