use crate::types::traits::tree::Tree;
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use chrono::{DateTime, Local};
use colored::Colorize;
use exec::types::Status;
use log::{debug, info, warn};
use std::fmt;

// Tests
mod test;

static INDENT: &str = " ";

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level = 1;
        if self.status.is_some() {
            let printable = self.make_tree(level).unwrap();
            write!(f, "{}", printable);
        } else {
            let printable = self.make_stateless_tree(level).unwrap();
            write!(f, "{}", printable);
        }

        // let i = INDENT.repeat(1);
        // if self.clone().status.is_some() {
        //     write!(f, "{} - ", &self.clone().status.unwrap())?;
        // } else {
        //     let icon = "●";
        //     write!(f, "{} {:?} - ", icon, &self.clone().status)?;
        // }
        // if self.clone().event.is_some() {
        //     write!(f, "{}", &self.clone().event.unwrap())?;
        // }
        // write!(f, "{}pipeline: {}\n", i, &self.name)?;
        // for step in &self.steps {
        //     if self.status.is_some() {
        //         if step.get_status() != None {
        //             write!(f, "{}", step);
        //         }
        //     } else {
        //         write!(f, "{}", step);
        //     }
        // }
        Ok(())
    }
}
impl fmt::Display for StepOrParallel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StepOrParallel::Step(res) => {
                write!(f, "{}", res);
            }
            StepOrParallel::Parallel(res) => {
                write!(f, "{}", res);
            }
        }
        Ok(())
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level = 0;
        if self.status.is_some() {
            let printable = self.make_tree(level).unwrap();
            write!(f, "{}", printable);
        } else {
            let printable = self.make_stateless_tree(level).unwrap();
            write!(f, "{}", printable);
        }
        //     let i = INDENT.repeat(2);
        //     let mtop = '┬';
        //     let lbot = '╰';
        //     let hbar = '─';
        //     warn!(target :"nude","{}{mtop}\n  {lbot}{hbar} step: {}\n",i, &self.name);
        //     if self.status.is_some() {
        //         for command in &self.commands {
        //             write!(f, "{}", command);
        //             if command.output.is_none() {
        //                 break;
        //             }
        //         }
        //     } else {
        //         for command in &self.commands {
        //             write!(f, "{}", command);
        //         }
        //     }
        Ok(())
    }
}
impl fmt::Display for Parallel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for step in &self.steps {
            write!(f, "{}", step);
        }
        Ok(())
    }
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level = 0;
        if self.output.is_some() {
            let printable = self.make_tree(level).unwrap();
            write!(f, "{}", printable);
        } else {
            let printable = self.make_stateless_tree(level).unwrap();
            write!(f, "{}", printable);
        }
        // self.make_tree();
        // let i = INDENT.repeat(4);
        // let j = INDENT.repeat(6);
        // let command: Command = self.clone();
        // let mtop = '┬';
        // let lbot = '╰';
        // let hbar = '─';
        // if self.output.is_none() {
        //     info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin ,i=i);
        // } else {
        //     if self.output.clone().unwrap().status == Status::Running {
        //         info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin.green() ,i=i);
        //         return Ok(());
        //     } else {
        //         let output = self.output.clone().unwrap();
        //         let mut stdout = "".to_owned();
        //         let mut stderr = "".to_owned();
        //
        //         if output.stdout.is_some() {
        //             stdout = output.stdout.unwrap().replace("\n", &format!("\n{}", j));
        //         }
        //         if output.stderr.is_some() {
        //             stderr = output.stderr.unwrap().replace("\n", &format!("\n{}", j));
        //         }
        //         let status = output.status;
        //         if status == Status::Succeeded {
        //             info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin.blue() ,i=i);
        //             debug!(target: "nude", "{}{}\n", j,stdout);
        //         } else {
        //             info!(target: "nude", "\r{i:} {mtop}\n{i:} {lbot}{hbar} {}\n", &self.stdin.red() ,i=i);
        //             debug!(target: "nude", "{}{}\n", j,stderr);
        //         }
        //     }
        // }
        Ok(())
    }
}
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = INDENT.repeat(1);
        let date = self.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        write!(f, "{}\n", date)?;
        warn!(target:"nude", "{}trigger:\n{}", i, &self.trigger);
        Ok(())
    }
}
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
