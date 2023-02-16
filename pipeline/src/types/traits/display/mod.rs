use crate::types::traits::tree::def::Node;
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
        let mut indent = "".to_owned();
        if self.status.is_some() {
            let tree = Node::from(self);
            write!(f, "{}\n", tree)?;
            // let printable = self.clone().make_statefull_tree(&mut indent);
            // write!(f, "{}", printable);
        } else {
            // let printable = self.clone().make_stateless_tree(&mut indent);
            // write!(f, "{}", printable);
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
        let mut indent = "".to_owned();
        if self.status.is_some() {
            let printable = self.clone().make_statefull_tree(&mut indent);
            write!(f, "{}", printable);
        } else {
            let printable = self.clone().make_stateless_tree(&mut indent);
            write!(f, "{}", printable);
        }
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
        let mut indent = "".to_owned();
        if self.output.is_some() {
            let printable = self.clone().make_statefull_tree(&mut indent);
            write!(f, "{}", printable);
        } else {
            let printable = self.clone().make_stateless_tree(&mut indent);
            write!(f, "{}", printable);
        }
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
