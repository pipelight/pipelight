use crate::types::{Command, Event, Pipeline, Status, Step, Trigger};
use chrono::{DateTime, Local};
use colored::Colorize;
use log::{debug, info, warn};
use std::fmt;

// Tests
mod test;

static INDENT: &str = " ";

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = INDENT.repeat(2);
        if self.action.is_some() {
            write!(
                f,
                "{}action: {}\n",
                i,
                String::from(&self.clone().action.unwrap())
            )?;
        }
        if self.branch.is_some() {
            write!(f, "{}branch: {}\n", i, self.clone().branch.unwrap())?;
        }
        Ok(())
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = INDENT.repeat(1);
        let date = self.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        write!(f, "{}\n", date)?;
        if self.pid.is_some() {
            warn!(target:"nude", "{}pid: {}\n", i, &self.pid.unwrap());
        }
        warn!(target:"nude", "{}trigger:\n{}", i, &self.trigger);
        Ok(())
    }
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = INDENT.repeat(2);
        let mtop = '┬';
        let lbot = '╰';
        let hbar = '─';
        info!(target :"nude","{}{mtop}\n  {lbot}{hbar} step: {}\n",i, &self.name);
        for command in &self.commands {
            write!(f, "{}", command);
            if command.output.is_none() {
                break;
            }
        }
        Ok(())
    }
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = INDENT.repeat(4);
        let j = INDENT.repeat(6);
        let command: Command = self.clone();
        let mtop = '┬';
        let lbot = '╰';
        let hbar = '─';
        if self.output.is_none() {
            info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin.green() ,i=i);
            return Ok(());
        } else {
            let output = self.output.clone().unwrap();
            let mut stdout = "".to_owned();
            let mut stderr = "".to_owned();

            if output.stdout.is_some() {
                stdout = output.stdout.unwrap().replace("\n", &format!("\n{}", j));
            }
            if output.stderr.is_some() {
                stderr = output.stderr.unwrap().replace("\n", &format!("\n{}", j));
            }
            let status = output.status;
            if status {
                info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin.blue() ,i=i);
                debug!(target: "nude", "{}{}\n", j,stdout);
            } else {
                info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin.red() ,i=i);
                debug!(target: "nude", "{}{}\n", j,stderr);
            }
        }
        Ok(())
    }
}
impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = INDENT.repeat(1);
        if self.clone().status.is_some() {
            write!(f, "{} - ", &self.clone().status.unwrap())?;
        }
        if self.clone().event.is_some() {
            write!(f, "{}", &self.clone().event.unwrap())?;
        }
        write!(f, "{}pipeline: {}\n", i, &self.name)?;
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
