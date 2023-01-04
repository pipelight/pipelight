use super::{Pipeline, Status};
use chrono::{DateTime, Local};
use colored::Colorize;
use log::{debug, info};
use std::fmt;

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - ", &self.status);
        let str_date = &self.date.as_ref().unwrap();
        let date = str_date.parse::<DateTime<Local>>().unwrap();
        // let date: &str = &binding.as_ref();
        write!(f, "{}\n", date.to_rfc2822());
        write!(f, "\t\tpipeline: {}\n", self.name);
        if self.pid.is_some() {
            write!(f, "   pid: {}\n", &self.pid.unwrap());
        }
        for step in &self.steps {
            info!(target :"nude","\t\t\tstep: {}\n", step.name);
            for command in &step.commands {
                let stdout = command.output.as_ref().unwrap().stdout.as_ref().unwrap();
                let stderr = command.output.as_ref().unwrap().stderr.as_ref().unwrap();
                let status = command.output.as_ref().unwrap().status;
                if status {
                    info!(target: "nude", "\t\t\t\t{}\n", &command.stdin.green());
                    debug!(target: "nude", "{}\n", stdout)
                } else {
                    info!(target: "nude", "\t\t\t\t{}\n", &command.stdin.red());
                    debug!(target: "nude", "\r{}\n", stderr);
                }
            }
        }
        Ok(())
    }
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = "●";
        match *self {
            Status::Started => write!(f, "{} Started", icon),
            Status::Succeeded => write!(f, "{} Succeeded", icon.blue()),
            Status::Failed => write!(f, "{} Failed", icon.red()),
            Status::Running => write!(f, "{} Running", icon.green()),
            Status::Aborted => write!(f, "{} Aborted", icon.yellow()),
            Status::Never => write!(f, "{} Never", icon),
        };
        Ok(())
    }
}
