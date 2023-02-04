// Relative paths
use super::characters::Characters;
use super::Tree;

// Absolute paths
use crate::types::{Command, Event, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use log::{debug, info, warn};
use std::error::Error;

static INDENT: &str = " ";

impl Tree<Pipeline> for Pipeline {
    fn make_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level;
        let indent = INDENT.repeat(level);
        let has_status = self.status.is_some();
        let mut printable: String = "".to_owned();

        let root = format!(" pipeline: {}\n", &self.name);
        printable.push_str(&root);

        let steps_length = &self.steps.len() - 1;
        level = level + 1;
        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
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
    fn make_stateless_tree(&self, level: usize) -> Result<String, Box<dyn Error>> {
        let mut level = level;
        let indent = INDENT.repeat(level);
        let has_status = self.status.is_some();
        let mut printable: String = "".to_owned();

        let root = format!(" pipeline: {}\n", &self.name);
        printable.push_str(&root);

        let steps_length = &self.steps.len() - 1;
        level = level + 1;
        let mut leaf = format!(
            "{indent:}{}\n{indent:}{}{}",
            Characters::unicode().mtop,
            Characters::unicode().lcross,
            Characters::unicode().hbar,
            indent = indent,
        );
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
            let step_with_leaf = format!("{}{}", leaf, &step.make_stateless_tree(level)?);
            printable.push_str(&step_with_leaf);
        }

        Ok(printable)
    }
}
