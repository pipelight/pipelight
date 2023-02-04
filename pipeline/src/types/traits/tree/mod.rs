use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
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
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );

        for (i, cmd) in self.commands.iter().enumerate() {
            let indent = INDENT.repeat(level);
            if 0 <= i && i < cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            if i == cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lbot,
                    Characters::unicode().hbar,
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
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
        for (i, cmd) in self.commands.iter().enumerate() {
            let indent = INDENT.repeat(level);
            if 0 <= i && i < cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            if i == cmds_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lbot,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            // printable.push_str(cmd.make_tree(level)?);
            printable.push_str(&format!("{}{}", leaf, cmd.make_stateless_tree(level)?))
        }
        Ok(printable)
    }
}
impl Tree<Command> for Command {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level + 2;
        let indent = INDENT.repeat(level);

        let mut printable: String = "".to_owned();

        let root = format!(" {}\n", &self.stdin);
        printable.push_str(&root);

        let leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lbot,
            Characters::unicode().hbar,
            indent = indent,
        );

        if self.output.is_some() {
            let cmd_with_leaf = format!("{} {}\n", leaf, "my_output");
            printable.push_str(&cmd_with_leaf);
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
