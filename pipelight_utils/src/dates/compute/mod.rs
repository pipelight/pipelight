// Test
mod test;
// Duration
use super::Duration;
use chrono::{DateTime, Local};
use std::time;
// Convertion functions
use crate::dates::convert::*;
// Error Handling
use miette::{IntoDiagnostic, Result};

impl Duration {
    /**
    The favorite way to get the duration as a standard duration struct(std::time::Duration).
    Compute the duration from started/ended dates or
    from started/now dates if the duration hasn't been stopped yet.
    */
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
    /**
    Set the duration start date to now.
    Start the duration count.
    */
    pub fn start(&mut self) -> Result<()> {
        let now = Local::now();
        self.started_at = Some(now.to_string());
        Ok(())
    }
    /**
    Set the duration end date to now.
    Stop the duration count.
    */
    pub fn stop(&mut self) -> Result<()> {
        let now = Local::now();
        self.ended_at = Some(now.to_string());
        let diff = self.get_ended_at()? - self.get_started_at()?;
        let diff = diff.to_std().unwrap();
        self.computed = Some(std_duration_to_iso8601(&diff)?);
        Ok(())
    }
    /**
    Returns the started_at field as an exploitable DateTime.
    */
    fn get_started_at(&self) -> Result<DateTime<Local>> {
        let parsed = self
            .started_at
            .clone()
            .unwrap()
            .parse::<DateTime<Local>>()
            .into_diagnostic()?;
        Ok(parsed)
    }
    /**
    Returns the ended_at field as an exploitable DateTime.
    */
    fn get_ended_at(&self) -> Result<DateTime<Local>> {
        let parsed = self
            .ended_at
            .clone()
            .unwrap()
            .parse::<DateTime<Local>>()
            .into_diagnostic()?;
        Ok(parsed)
    }
}
