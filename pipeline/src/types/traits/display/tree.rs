use super::characters::Characters;
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use log::{debug, info, warn};
use std::error::Error;

pub trait Tree<T> {
    /// Return a tree structure
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>>;
}

static INDENT: &str = " ";

impl Tree<Pipeline> for Pipeline {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level + 1;
        let indent = INDENT.repeat(level);
        let has_status = self.status.is_some();
        let mut printable: String = "".to_owned();

        let root = format!(" pipeline: {}\n", &self.name);
        printable.push_str(&root);

        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );

        let steps_length = &self.steps.len() - 1;
        level = level + 1;
        for (i, step) in self.steps.iter().enumerate() {
            let indent = INDENT.repeat(level);
            if 0 <= i && i < steps_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lcross,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            if i == steps_length {
                leaf = format!(
                    "{indent:}{}\n{indent:}{}{}",
                    Characters::unicode().vbar,
                    Characters::unicode().lbot,
                    Characters::unicode().hbar,
                    indent = indent,
                );
            }
            // warn!(target: "nude", "{}{}", leaf, step)
            let step_with_leaf = format!("{}{}", leaf, &step.make_tree(level)?);
            printable.push_str(&step_with_leaf);
        }

        Ok(printable)
    }
}
impl Tree<Step> for StepOrParallel {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let printable: String;
        match self {
            StepOrParallel::Step(res) => printable = res.make_tree(level)?,
            StepOrParallel::Parallel(res) => printable = res.make_tree(level)?,
        }
        Ok(printable)
    }
}
impl Tree<Step> for Parallel {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut printable: String = "".to_owned();
        for step in &self.steps {
            printable.push_str(&step.make_tree(level)?);
        }
        Ok(printable)
    }
}

impl Tree<Step> for Step {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level + 1;
        let indent = INDENT.repeat(level);
        let has_status = self.status.is_some();
        let leaf = Characters::unicode().hbar;
        let mut printable: String = "".to_owned();

        let root = format!(" step: {}\n", &self.name);
        printable.push_str(&root);

        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );

        let cmds_length = &self.commands.len() - 1;
        level = level + 1;
        for (i, cmd) in self.commands.iter().enumerate() {
            let indent = INDENT.repeat(level + 2);
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
}
impl Tree<Command> for Command {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level + 1;
        let indent = INDENT.repeat(level);
        let leaf = Characters::unicode().hbar;

        let mut printable: String = "".to_owned();

        let leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().vbar,
            Characters::unicode().lbot,
            Characters::unicode().hbar,
            indent = indent,
        );
        if self.output.is_some() {
            printable.push_str(&format!("{}{:?}", leaf, &self.output.clone().unwrap()));
        }
        Ok(printable)
    }
}
