use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use colored::Colorize;
use exec::types::Status;
use log::{debug, info, warn};
use std::error::Error;

pub mod characters;
pub mod pipeline;
use super::tree::characters::Characters;

static INDENT: &str = "  ";

pub trait Tree<T> {
    /// Return a tree structure
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>>;
    fn make_stateless_tree(&self, level: usize) -> Result<String, Box<dyn Error>>;
}

impl Tree<StepOrParallel> for StepOrParallel {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_tree(level)?,
            StepOrParallel::Parallel(res) => printable = res.make_tree(level)?,
        }
        Ok(printable)
    }
    fn make_stateless_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_stateless_tree(level)?,
            StepOrParallel::Parallel(res) => printable = res.make_stateless_tree(level)?,
        }
        Ok(printable)
    }
}
impl Tree<Parallel> for Parallel {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut printable: String = "".to_owned();
        for step in &self.steps {
            printable.push_str(&step.make_tree(level)?);
        }
        Ok(printable)
    }
    fn make_stateless_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut printable: String = "".to_owned();
        for step in &self.steps {
            printable.push_str(&step.make_stateless_tree(level)?);
        }
        Ok(printable)
    }
}

impl Tree<Step> for Step {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level;

        let indent = INDENT.repeat(level);
        let has_status = self.status.is_some();
        let leaf = Characters::unicode().hbar;
        let mut printable: String = "".to_owned();

        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        let cmds_length = &self.commands.len() - 1;
        level = level + 1;
        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            Characters::unicode().rarrow,
            indent = indent,
        );

        for (i, cmd) in self.commands.iter().enumerate() {
            let indent = INDENT.repeat(level);
            if 0 <= i && i < cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    Characters::unicode().rarrow,
                    indent = indent,
                );
            }
            if i == cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    Characters::unicode().rarrow,
                    indent = indent,
                );
            }
            // printable.push_str(cmd.make_tree(level)?);
            printable.push_str(&format!("{}{}", leaf, cmd.make_tree(level)?))
        }
        Ok(printable)
    }
    fn make_stateless_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level;
        let indent = INDENT.repeat(level);
        let has_status = self.status.is_some();
        let leaf = Characters::unicode().hbar;
        let mut printable: String = "".to_owned();

        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        let cmds_length = &self.commands.len() - 1;
        level = level + 1;
        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            Characters::unicode().rarrow,
            indent = indent,
        );
        for (i, cmd) in self.commands.iter().enumerate() {
            let indent = INDENT.repeat(level);
            if 0 <= i && i < cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    Characters::unicode().rarrow,
                    indent = indent,
                );
            }
            if i == cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lbot,
                    Characters::unicode().hbar,
                    Characters::unicode().rarrow,
                    indent = indent,
                );
            }
            printable.push_str(&format!("{}{}", leaf, cmd.make_stateless_tree(level)?))
        }
        Ok(printable)
    }
}
impl Tree<Command> for Command {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let level = level + 2;
        let mut indent = INDENT.repeat(level);

        let mut printable: String = "".to_owned();

        let mut root: String;
        let mut cmd_with_leaf: String;

        let leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lbot,
            Characters::unicode().hbar,
            indent = indent,
        );

        if self.output.is_some() {
            match self.output.clone().unwrap().status {
                Status::Started => {
                    root = format!(" {}\n", &self.stdin);
                    printable.push_str(&root);
                }
                Status::Running => {
                    root = format!(" {}\n", &self.stdin.green());
                    printable.push_str(&root);
                }

                Status::Succeeded => {
                    root = format!(" {}\n", &self.stdin.blue());
                    printable.push_str(&root);
                    if self.output.clone().unwrap().stdout.is_some() {
                        let mut stdout = self.output.clone().unwrap().stdout.unwrap();
                        indent = INDENT.repeat(level + 1);
                        stdout = stdout.replace("\n", &format!("\n{indent:} ", indent = indent));
                        cmd_with_leaf = format!("{} {}\n", leaf, stdout);
                        printable.push_str(&cmd_with_leaf);
                    }
                }
                Status::Failed => {
                    root = format!(" {}\n", &self.stdin.red());
                    printable.push_str(&root);
                    if self.output.clone().unwrap().stderr.is_some() {
                        let mut stderr = self.output.clone().unwrap().stderr.unwrap();
                        indent = INDENT.repeat(level + 1);
                        stderr = stderr.replace("\n", &format!("\n{indent:} ", indent = indent));
                        cmd_with_leaf = format!("{} {}\n", leaf, stderr);
                        printable.push_str(&cmd_with_leaf);
                    }
                }
                Status::Aborted => {
                    root = format!(" {}\n", &self.stdin.yellow());
                    printable.push_str(&root);
                }
            }
        }
        Ok(printable)
    }
    fn make_stateless_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut printable: String = "".to_owned();

        let root = format!(" {}\n", &self.stdin);
        printable.push_str(&root);

        Ok(printable)
    }
}
