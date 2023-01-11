use crate::types::{Pipeline, Status};
use chrono::{DateTime, Local};
use colored::Colorize;
use log::{debug, info};
use std::fmt;

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mtop = '┬';
        let lbot = '╰';
        let hbar = '─';
        write!(f, "{} - ", &self.clone().status.unwrap())?;
        let str_date = &self.date.as_ref().unwrap();
        let date = str_date.parse::<DateTime<Local>>().unwrap();
        write!(f, "{}\n", date.to_rfc2822())?;
        if self.pid.is_some() {
            write!(f, "  pid: {}\n", &self.pid.unwrap())?;
        }
        write!(f, "  pipeline: {}\n", self.name)?;
        for step in &self.steps {
            info!(target :"nude","  {mtop}\n  {lbot}{hbar} step: {}\n", step.name);
            for command in &step.commands {
                if command.output.as_ref().is_some() {
                    let stdout = command
                        .output
                        .as_ref()
                        .unwrap()
                        .stdout
                        .as_ref()
                        .unwrap()
                        .replace("\n", "\n\t\t");
                    let stderr = command.output.as_ref().unwrap().stderr.as_ref().unwrap();
                    let status = command.output.as_ref().unwrap().status;
                    if status {
                        info!(target: "nude", "\r      {mtop}\n      {lbot}{hbar} {}\n", &command.stdin.blue());
                        debug!(target: "nude", "\t\t{}\n", stdout)
                    } else {
                        info!(target: "nude", "\r      {mtop}\n      {lbot}{hbar} {}\n", &command.stdin.red());
                        debug!(target: "nude", "\t\t{}\n", stderr);
                    }
                } else {
                    info!(target: "nude", "      {}", &command.stdin.green());
                    break;
                }
            }
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
            Status::Never => write!(f, "{} {}", icon, "Never".bold()),
        };
        Ok(())
    }
}
