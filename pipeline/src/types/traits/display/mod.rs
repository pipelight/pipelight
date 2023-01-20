use crate::types::{Command, Event, Pipeline, Status, Step};
use chrono::{DateTime, Local};
use colored::Colorize;
use log::{debug, info};
use std::fmt;

// Tests
mod test;

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = self.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        write!(f, "{}\n", date)?;
        if self.pid.is_some() {
            write!(f, "  pid: {}\n", &self.pid.unwrap())?;
        }
        Ok(())
    }
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mtop = '┬';
        let lbot = '╰';
        let hbar = '─';
        info!(target :"nude","  {mtop}\n  {lbot}{hbar} step: {}\n", &self.name);
        for command in &self.commands {
            write!(f, "{}", command);
        }
        Ok(())
    }
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mtop = '┬';
        let lbot = '╰';
        let hbar = '─';
        if self.output.is_none() {
            info!(target: "nude", "      {}", &self.stdin.green());
            return Ok(());
        }
        let stdout = self
            .output
            .as_ref()
            .unwrap()
            .stdout
            .as_ref()
            .unwrap()
            .replace("\n", "\n\t\t");
        let stderr = self.output.as_ref().unwrap().stderr.as_ref().unwrap();
        let status = self.output.as_ref().unwrap().status;
        if status {
            info!(target: "nude", "\r      {mtop}\n      {lbot}{hbar} {}\n", &self.stdin.blue());
            debug!(target: "nude", "\t\t{}\n", stdout)
        } else {
            info!(target: "nude", "\r      {mtop}\n      {lbot}{hbar} {}\n", &self.stdin.red());
            debug!(target: "nude", "\t\t{}\n", stderr);
        }
        Ok(())
    }
}
impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.clone().status.is_some() {
            write!(f, "{} - ", &self.clone().status.unwrap())?;
        }
        if self.clone().event.is_some() {
            write!(f, "{}", &self.clone().event.unwrap())?;
        }
        write!(f, "  pipeline: {}\n", &self.name)?;
        for step in &self.steps {
            write!(f, "{}", step);
        }
        Ok(())
    }
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = "●";
        match *self {
            Status::Started => write!(f, "{} Started", icon),
            Status::Succeeded => write!(f, "{} {}", icon.blue(), "Succeeded".bold()),
            Status::Failed => write!(f, "{} {}", icon.red(), "Failed".bold()),
            Status::Running => write!(f, "{} {}", icon.green(), "Running".bold()),
            Status::Aborted => write!(f, "{} {}", icon.yellow(), "Aborted".bold()),
        };
        Ok(())
    }
}
